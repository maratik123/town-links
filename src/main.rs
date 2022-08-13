use town_links::{err::Error, event_loop};

fn main() -> Result<(), Error> {
    pollster::block_on(event_loop::run())?;
    Ok(())
}
