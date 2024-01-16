#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod display;
mod error;
mod process;
mod tray;

use config::Config;
use display::Display;
use hotreload::Hotreload;
use process::ProcessChecker;
use std::error::Error;
use std::sync::RwLock;

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

    let watcher = Hotreload::<RwLock<Config>, Config>::new(config_path)?;
    let config = watcher.config();

    let mut checker = ProcessChecker::new();
    let mut display = Display::create(config.read().unwrap().display_index)?; // TODO make it hotreload aware
    let mut active = false;

    loop {
        if checker.check(&config.read().unwrap().apps) {
            if let Err(error) = display.load_settings() {
                eprintln!("Failed to load display settings: {}", error);
            } else {
                let target = config.read().unwrap().target_refresh;
                if display.refresh() != target {
                    display.set_refresh(target);
                    if let Err(error) = display.apply_settings() {
                        eprintln!("Failed to change display settings: {}", error);
                    } else {
                        println!("Applied new refresh rate: {target}");
                        active = true;
                    }
                }
            }
        } else if active {
            if let Err(error) = display.reset_settings() {
                eprintln!("Failed to reset display: {}", error);
            } else {
                println!("Reset to the original refresh rate");
            }
            active = false;
        }

        let wait = config.read().unwrap().check_interval;
        std::thread::sleep(std::time::Duration::from_secs(wait));
    }
}
