use town_links::{err::Error, gui};

fn main() -> Result<(), Error> {
    gui::run()?;
    Ok(())
}
