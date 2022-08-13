use crate::err::Error;
use crate::state::State;
use wgpu::SurfaceError;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
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
        } if window.id() == window_id => {
            if !state.input(event) {
                match event {
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
                    WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size)
                    }
                    WindowEvent::CursorMoved { position, .. } => state.update_color(position),
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window.id() == window_id => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(err) => match err {
                    Error::WgpuSurfaceError(SurfaceError::Lost) => state.resize(state.get_size()),
                    Error::WgpuSurfaceError(SurfaceError::OutOfMemory) => {
                        eprintln!("{:?}", err);
                        *control_flow = ControlFlow::Exit;
                    }
                    err => eprintln!("{:?}", err),
                },
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });
}
