mod display;
use crate::display::Display;

slint::include_modules!();

fn main() {
    println!("Hello, world!");

    let displays = Display::query().unwrap();

    for display in displays {
        println!("Device: {} | Monitor: {}", display.device, display.monitor);
    }

    if let Err(error) = window() {
        eprintln!("Window error: {error}");
    }
}

fn window() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    // let ui_handle = ui.as_weak();
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });

    ui.run()?;
    Ok(())
}
