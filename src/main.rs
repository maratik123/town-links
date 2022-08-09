use imgui::{Condition, Window};
use town_links::{err::Error, gui::System};

fn main() -> Result<(), Error> {
    let system = System::init()?;
    system.main_loop(|run, ui| {
        Window::new("Town links")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text_wrapped("Town links");
                if ui.button("test button") {
                    *run = false;
                }
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
    Ok(())
}
