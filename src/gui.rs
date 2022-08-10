use crate::err::Error;
use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, PresentMode,
    Queue, RequestAdapterOptions, RequestAdapterOptionsBase, Surface, SurfaceConfiguration,
    TextureUsages,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub async fn run() -> Result<(), Error> {
    env_logger::try_init()?;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    let mut state = State::new(&window).await?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window.id() == window_id => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}

struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

impl State {
    async fn new(window: &Window) -> Result<Self, Error> {
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
            width: size.width.min(1),
            height: size.height.min(1),
            present_mode: PresentMode::AutoVsync,
        };
        surface.configure(&device, &config);
        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
        })
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), Error> {
        todo!()
    }
}
