#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod display;
mod error;
mod process;
mod state;
mod tray;
mod wstring;

use config::Config;
use hotreload::Hotreload;
use process::ProcessChecker;
use std::error::Error;
use std::time::Duration;

use crate::state::State;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Refresh Rate Control");

    // Show tray icon
    std::thread::spawn(|| match tray::show() {
        Ok(_) => std::process::exit(0),
        Err(error) => eprintln!("Tray icon error: {error}"),
    });

    let config_path = config::default_path();
    if !config_path.exists() {
        return Err("Configuration file (config.toml) does not exist!".into());
    }

    let watcher = Hotreload::<State, Config>::new(config_path)?;
    let state = watcher.config();
    let mut checker = ProcessChecker::new();

    loop {
        {
            let mut display = state.display();
            if checker.check(&state.apps()) {
                display.activate();
            } else if display.active() {
                display.deactivate();
            }
        } // state.display() must be dropped before sleep

        std::thread::sleep(Duration::from_secs(state.check_interval()));
    }
}
