#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod display;
mod error;
mod hotreload;
mod process;

use config::Config;
use display::Display;
use process::ProcessChecker;
use std::error::Error;
use std::sync::{Arc, RwLock};

type RuntimeConfig = Arc<RwLock<Config>>;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Refresh Rate Control");

    let config_path = config::default_path()?;
    if !config_path.exists() {
        return Err("Configuration file (config.toml) does not exist!".into());
    }

    let config = Arc::new(RwLock::new(Config::load(&config_path)?));
    let _watcher = hotreload::watch(&config_path, config.clone())?;

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
                        println!("Applied new refresh rate");
                        active = true;
                    }
                }
            }
        } else if active {
            if let Err(error) = display.reset_settings() {
                eprintln!("Failed to reset display: {}", error);
            } else {
                println!("Reset old refresh rate");
            }
            active = false;
        }

        let wait = config.read().unwrap().check_interval;
        std::thread::sleep(std::time::Duration::from_secs(wait));
    }
}
