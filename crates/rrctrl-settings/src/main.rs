mod config;
mod display;

slint::include_modules!();

use crate::display::Display;
use rrctrl_config::Config;
use slint::{ModelRc, SharedString, VecModel};
use std::collections::HashMap;

struct UiDisplay {
    pub device: String,
    pub monitor: SharedString,
    pub refresh: ModelRc<SharedString>,
}

impl UiDisplay {
    pub fn from(displays: Vec<Display>) -> Vec<Self> {
        displays
            .into_iter()
            .enumerate()
            .map(|(i, d)| Self {
                device: d.device,
                monitor: SharedString::from(format_monitor(&d.monitor, i)),
                refresh: ModelRc::new(VecModel::from(
                    d.refresh
                        .into_iter()
                        .map(|r| SharedString::from(format_refresh(r)))
                        .collect::<Vec<_>>(),
                )),
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");

    let displays = Display::query().expect("Failed to query display infos");
    let current = config::current(&displays);

    if let Err(error) = window(current, UiDisplay::from(displays)) {
        eprintln!("Window error: {error}");
    }
}

fn window(
    current: CurrentConfig,
    displays: Vec<UiDisplay>,
) -> Result<(), Box<dyn std::error::Error>> {
    let monitors = ModelRc::new(VecModel::from(
        displays
            .iter()
            .map(|d| d.monitor.clone())
            .collect::<Vec<_>>(),
    ));

    let refresh_rates = displays
        .iter()
        .map(|d| (d.monitor.clone(), d.refresh.clone()))
        .collect::<HashMap<_, _>>();

    let ui = SettingsWindow::new()?;

    ui.set_config(current);

    ui.global::<Backend>().on_monitors(move || monitors.clone());
    ui.global::<Backend>()
        .on_refresh_rates(move |monitor| match refresh_rates.get(&monitor) {
            Some(rates) => rates.clone(),
            None => ModelRc::default(),
        });

    let ui_handle = ui.as_weak();
    ui.on_apply(move || {
        let ui = ui_handle.unwrap();

        let Some(display_name) = displays
            .iter()
            .find(|d| d.monitor == ui.get_monitor())
            .map(|d| d.device.clone())
        else {
            eprintln!("Apply failed: Not a valid display");
            return;
        };

        let Some(Ok(target_refresh)) = ui
            .get_refresh()
            .split_once(' ')
            .map(|(hz, _)| hz.parse::<u32>())
        else {
            eprintln!("Apply failed: Not a valid refresh rate");
            return;
        };

        let cfg = Config {
            display_name,
            target_refresh,
            check_interval: 12,
            apps: vec![],
        };

        println!("Applying: {:#?}", cfg);
        if let Err(error) = config::save(&cfg) {
            eprintln!("Failed to save config: {error}");
        }
    });

    ui.run()?;
    Ok(())
}

fn format_monitor(monitor: &str, index: usize) -> String {
    format!("{}. {}", index + 1, monitor)
}

fn format_refresh(refresh: u32) -> String {
    format!("{} Hz", refresh)
}
