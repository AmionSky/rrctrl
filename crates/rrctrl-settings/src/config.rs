use crate::{CurrentConfig, Display};
use rrctrl_config::Config;
use slint::{ModelRc, SharedString, VecModel};
use std::error::Error;

pub fn current(displays: &[Display]) -> CurrentConfig {
    match load() {
        Ok(config) => {
            let apps = config
                .apps
                .into_iter()
                .map(SharedString::from)
                .collect::<Vec<_>>();
            let monitor = displays
                .iter()
                .enumerate()
                .find(|(_, d)| d.device == config.display_name)
                .map_or_else(
                    || crate::format_monitor(&config.display_name, displays.len()),
                    |(i, d)| crate::format_monitor(&d.monitor, i),
                );

            CurrentConfig {
                applications: ModelRc::new(VecModel::from(apps)),
                interval: config.check_interval as i32,
                monitor: SharedString::from(monitor),
                refresh: SharedString::from(crate::format_refresh(config.target_refresh)),
            }
        }
        Err(_) => {
            let monitor = displays.first().map_or_else(
                || String::from("No display found"),
                |d| crate::format_monitor(&d.monitor, 0),
            );
            let refresh = displays
                .first()
                .and_then(|d| d.refresh.first().map(|r| crate::format_refresh(*r)))
                .unwrap_or_else(|| String::from("No refresh rate found"));

            CurrentConfig {
                applications: ModelRc::new(VecModel::from(vec![])),
                interval: 12,
                monitor: SharedString::from(monitor),
                refresh: SharedString::from(refresh),
            }
        }
    }
}

pub fn load() -> Result<Config, Box<dyn Error>> {
    let path = Config::path();
    let file = std::fs::read_to_string(path)?;
    let cfg = toml::from_str(&file)?;
    Ok(cfg)
}

pub fn save(config: &Config) -> Result<(), Box<dyn Error>> {
    let path = Config::path();
    let contents = toml::to_string_pretty(config)?;
    std::fs::write(path, contents)?;
    Ok(())
}
