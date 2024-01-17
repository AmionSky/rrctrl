mod display;
use crate::display::Display;

fn main() {
    println!("Hello, world!");

    let displays = Display::query().unwrap();

    for display in displays {
        println!("Device: {} | Monitor: {}", display.device, display.monitor);
    }
}
