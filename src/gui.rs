use crate::err::Error;
use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    Display, Surface,
};
use imgui::{Context, FontConfig, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::{default::Default, time::Instant};

pub struct System {
    event_loop: EventLoop<()>,
    display: Display,
    imgui: Context,
    platform: WinitPlatform,
    renderer: Renderer,
    _font_size: f32,
}

impl System {
    pub fn init() -> Result<System, Error> {
        let title = "Town links";
        let event_loop = EventLoop::new();
        let context = ContextBuilder::new().with_vsync(true);
        let builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(1024f64, 768f64));
        let display = Display::new(builder, context, &event_loop)?;
        let mut imgui = Context::create();
        imgui.set_ini_filename(None);

        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            let dpi_mode = HiDpiMode::Default;

            platform.attach_window(imgui.io_mut(), window, dpi_mode);
        }

        let font_size = 13.0;
        imgui.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../resources/Roboto-Regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.5,
                oversample_h: 4,
                oversample_v: 4,
                ..Default::default()
            }),
        }]);

        let renderer = Renderer::init(&mut imgui, &display)?;

        Ok(System {
            event_loop,
            display,
            imgui,
            platform,
            renderer,
            _font_size: font_size,
        })
    }

    pub fn main_loop(self, mut run_ui: impl FnMut(&mut bool, &mut Ui) + 'static) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;

        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                if let Err(err) = platform.prepare_frame(imgui.io_mut(), gl_window.window()) {
                    eprintln!("Failed to prepare frame: {:?}", err);
                } else {
                    gl_window.window().request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                let mut ui = imgui.frame();

                let mut run = true;
                run_ui(&mut run, &mut ui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                if let Err(err) = renderer.render(&mut target, draw_data) {
                    eprintln!("Failed to render: {:?}", err);
                } else if let Err(err) = target.finish() {
                    eprintln!("Failed to finish target: {:?}", err);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}
