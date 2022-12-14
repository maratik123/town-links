use crate::model::{Model, ModelUniform};
use crate::{
    camera::{Camera, CameraUniform},
    camera_controller::CameraController,
    challenge::{Challenge, ChallengeEnum},
    err::Error,
    pipeline::create_pipeline,
    texture::TextureState,
    vertex::{INDICES, INDICES_CHALLENGE2, VERTICES},
};
use bytemuck::cast_slice;
use cgmath::Vector3;
use image::ImageFormat;
use std::iter;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferUsages,
    Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, IndexFormat, Instance,
    Limits, LoadOp, Operations, PowerPreference, PresentMode, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RequestAdapterOptions, SamplerBindingType, ShaderStages,
    Surface, SurfaceConfiguration, TextureSampleType, TextureUsages, TextureViewDescriptor,
    TextureViewDimension,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    window::Window,
};

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    clear_color: Color,
    render_pipeline: RenderPipeline,
    challenge_pipeline: RenderPipeline,
    challenge: Challenge,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    index_buffer_challenge2: Buffer,
    num_indices_challenge2: u32,
    diffuse_bind_group: BindGroup,
    challenge3_bind_group: BindGroup,
    _diffuse_texture: TextureState,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    camera_controller: CameraController,
    model: Model,
    model_uniform: ModelUniform,
    model_buffer: Buffer,
    model_bind_group: BindGroup,
}

impl State {
    pub async fn new(window: &Window) -> Result<Self, Error> {
        let size = window.inner_size();

        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(Error::RequestAdapterError)?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await?;

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: PresentMode::AutoVsync,
        };
        surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("Texture binding group layout"),
            });

        let diffuse_bytes = include_bytes!("../resources/happy-tree.png");
        let diffuse_texture = TextureState::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            ImageFormat::Png,
            "happy-tree.png texture",
        )?;

        let challenge3_bytes = include_bytes!("../resources/house.png");
        let challenge3_texture = TextureState::from_bytes(
            &device,
            &queue,
            challenge3_bytes,
            ImageFormat::Png,
            "house.png texture",
        )?;

        let diffuse_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&diffuse_texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("Diffuse bind group descriptor"),
        });

        let challenge3_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&challenge3_texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&challenge3_texture.sampler),
                },
            ],
            label: Some("Challenge3 bind group descriptor"),
        });

        let mat4x4_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Mat4x4 bind group layout"),
            });

        let (render_pipeline, challenge_pipeline, challenge4_pipeline) = create_pipeline(
            &device,
            &config,
            &[&texture_bind_group_layout, &mat4x4_bind_group_layout],
        );

        let clear_color = Color {
            r: 0.0,
            g: 0.2,
            b: 0.0,
            a: 1.0,
        };

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: cast_slice(INDICES),
            usage: BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        let index_buffer_challenge2 = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index buffer challenge"),
            contents: cast_slice(INDICES_CHALLENGE2),
            usage: BufferUsages::INDEX,
        });

        let num_indices_challenge2 = INDICES_CHALLENGE2.len() as u32;

        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform::default();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &mat4x4_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera bind group"),
        });

        let camera_controller = CameraController::new(0.2);

        let challenge = Challenge::default();

        let result = Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            challenge_pipeline,
            challenge,
            vertex_buffer,
            index_buffer,
            num_indices,
            index_buffer_challenge2,
            num_indices_challenge2,
            diffuse_bind_group,
            challenge3_bind_group,
            _diffuse_texture: diffuse_texture,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,
        };

        result.set_cursor_to_center(window)?;

        Ok(result)
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.camera.aspect = new_size.width as f32 / new_size.height as f32;
        self.update_camera_uniform();
        self.surface.configure(&self.device, &self.config);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.challenge = self.challenge.rotate();
                true
            }
            event => self.camera_controller.process_events(event),
        }
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.update_camera_uniform();
    }

    fn update_camera_uniform(&mut self) {
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue
            .write_buffer(&self.camera_buffer, 0, cast_slice(&[self.camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), Error> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            match self.challenge.into() {
                None | Some(ChallengeEnum::Second | ChallengeEnum::Third) => {
                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_bind_group(
                        0,
                        if let Some(ChallengeEnum::Third) = self.challenge.into() {
                            &self.challenge3_bind_group
                        } else {
                            &self.diffuse_bind_group
                        },
                        &[],
                    );
                    render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                    let (index_buffer, num_indices) =
                        if let Some(ChallengeEnum::Second) = self.challenge.into() {
                            (&self.index_buffer_challenge2, self.num_indices_challenge2)
                        } else {
                            (&self.index_buffer, self.num_indices)
                        };
                    render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                    render_pass.draw_indexed(0..num_indices, 0, 0..1);
                }
                Some(ChallengeEnum::First) => {
                    render_pass.set_pipeline(&self.challenge_pipeline);
                    render_pass.draw(0..3, 0..1);
                }
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    pub fn update_color(&mut self, position: &PhysicalPosition<f64>) {
        self.clear_color.r = position.x / self.size.width as f64;
        self.clear_color.b = position.y / self.size.height as f64;
    }

    #[inline]
    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    fn set_cursor_to_center(&self, window: &Window) -> Result<(), Error> {
        let cursor_position = PhysicalPosition::<f64> {
            x: self.size.width as f64 / 2.0,
            y: self.size.height as f64 / 2.0,
        };

        window.set_cursor_position(cursor_position)?;

        Ok(())
    }
}
