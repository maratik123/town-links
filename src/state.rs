use crate::{
    err::Error,
    pipeline::create_pipeline,
    vertex::{INDICES, INDICES_CHALLENGE2, VERTICES},
};
use bytemuck::cast_slice;
use image::{load_from_memory_with_format, GenericImageView, ImageFormat};
use std::{iter, num::NonZeroU32};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    AddressMode, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
    BufferUsages, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Extent3d, Features,
    FilterMode, ImageCopyTexture, ImageDataLayout, IndexFormat, Instance, Limits, LoadOp,
    Operations, Origin3d, PowerPreference, PresentMode, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RequestAdapterOptions, SamplerBindingType,
    SamplerDescriptor, ShaderStages, Surface, SurfaceConfiguration, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
    TextureViewDescriptor, TextureViewDimension,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    window::Window,
};

enum Challenge {
    First,
    Second,
}

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    clear_color: Color,
    render_pipeline: RenderPipeline,
    challenge_pipeline: RenderPipeline,
    challenge: Option<Challenge>,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
    index_buffer_challenge2: Buffer,
    num_indices_challenge2: u32,
    diffuse_bind_group: BindGroup,
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

        let diffuse_bytes = include_bytes!("../resources/happy-tree.png");
        let diffuse_image = load_from_memory_with_format(diffuse_bytes, ImageFormat::Png)?;
        let diffuse_rgba = diffuse_image.to_rgba8();
        let (dimensions_x, dimensions_y) = diffuse_image.dimensions();

        let texture_size = Extent3d {
            width: dimensions_x,
            height: dimensions_y,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(&TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some("Diffuse texture"),
        });

        queue.write_texture(
            ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &diffuse_rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions_x),
                rows_per_image: NonZeroU32::new(dimensions_y),
            },
            texture_size,
        );

        let diffuse_texture_view = diffuse_texture.create_view(&TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

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

        let diffuse_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&diffuse_texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("Diffuse bind group descriptor"),
        });

        let clear_color = Color {
            r: 0.0,
            g: 0.2,
            b: 0.0,
            a: 1.0,
        };

        let (render_pipeline, challenge_pipeline) =
            create_pipeline(&device, &config, &[&texture_bind_group_layout]);

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

        let result = Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            challenge_pipeline,
            challenge: None,
            vertex_buffer,
            index_buffer,
            num_indices,
            index_buffer_challenge2,
            num_indices_challenge2,
            diffuse_bind_group,
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
                self.rotate_challenge();
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}

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

            match self.challenge {
                None | Some(Challenge::Second) => {
                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                    let (index_buffer, num_indices) =
                        if let Some(Challenge::Second) = self.challenge {
                            (&self.index_buffer_challenge2, self.num_indices_challenge2)
                        } else {
                            (&self.index_buffer, self.num_indices)
                        };
                    render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                    render_pass.draw_indexed(0..num_indices, 0, 0..1);
                }
                Some(Challenge::First) => {
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
        self.clear_color.r = position.x / f64::from(self.size.width);
        self.clear_color.b = position.y / f64::from(self.size.height);
    }

    #[inline]
    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    #[inline]
    pub fn rotate_challenge(&mut self) {
        self.challenge = match self.challenge {
            None => Some(Challenge::First),
            Some(Challenge::First) => Some(Challenge::Second),
            Some(Challenge::Second) => None,
        };
    }

    fn set_cursor_to_center(&self, window: &Window) -> Result<(), Error> {
        let cursor_position = PhysicalPosition::<f64> {
            x: f64::from(self.size.width) / 2.0,
            y: f64::from(self.size.height) / 2.0,
        };

        window.set_cursor_position(cursor_position)?;

        Ok(())
    }
}
