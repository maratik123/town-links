use town_links::{err::Error, gui};

fn main() -> Result<(), Error> {
    pollster::block_on(gui::run())?;
    Ok(())
}
