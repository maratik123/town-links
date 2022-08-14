use crate::{err::Error, pipeline::create_pipeline, vertex::VERTICES};
use bytemuck::cast_slice;
use std::iter;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Backends, Buffer, BufferUsages, Color, CommandEncoderDescriptor, Device, DeviceDescriptor,
    Features, Instance, Limits, LoadOp, Operations, PowerPreference, PresentMode, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
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
    challenge: bool,
    vertex_buffer: Buffer,
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

        let clear_color = Color {
            r: 0.0,
            g: 0.2,
            b: 0.0,
            a: 1.0,
        };

        let (render_pipeline, challenge_pipeline) = create_pipeline(&device, &config);

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let result = Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            challenge_pipeline,
            challenge: false,
            vertex_buffer,
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
                self.inverse_challenge();
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

            render_pass.set_pipeline(if self.challenge {
                &self.challenge_pipeline
            } else {
                &self.render_pipeline
            });

            render_pass.draw(0..3, 0..1);
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
    pub fn inverse_challenge(&mut self) {
        self.challenge = !self.challenge;
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
