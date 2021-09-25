mod display;
mod error;
mod process;

use display::Display;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Refresh Rate Control");

    process::asd();
    return Ok(());

    // Create
    let mut display = Display::create(0)?;
    println!("Device name: {}", display.name());

    // Check refresh rate
    display.load_settings()?;
    println!(
        "Display refresh rate: {}",
        display.settings().dmDisplayFrequency
    );

    // Apply new refresh rate
    display.settings().dmDisplayFrequency = 144;
    display.apply_settings()?;

    // Check new refresh rate
    display.load_settings()?;
    println!(
        "Display refresh rate: {}",
        display.settings().dmDisplayFrequency
    );

    // Wait and reset
    std::thread::sleep(std::time::Duration::from_secs(5));
    display.reset_settings()?;

    // Check reset refresh rate
    display.load_settings()?;
    println!(
        "Display refresh rate: {}",
        display.settings().dmDisplayFrequency
    );

    Ok(())
}
