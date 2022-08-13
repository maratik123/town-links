use crate::err::Error;
use std::iter;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
    Limits, LoadOp, Operations, PowerPreference, PresentMode, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
    TextureViewDescriptor,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::WindowEvent,
    window::Window,
};

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    clear_color: Color,
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

        let cursor_position = PhysicalPosition::<f64> {
            x: f64::from(size.width) / 2.0,
            y: f64::from(size.height) / 2.0,
        };

        let mut result = Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
        };

        result.update_color(&cursor_position);

        window.set_cursor_position(cursor_position)?;

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

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
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
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
}
