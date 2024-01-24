mod display;

slint::include_modules!();

use crate::display::Display;
use slint::{ModelRc, SharedString, VecModel};
use std::collections::HashMap;
use std::rc::Rc;

struct UiDisplay {
    pub device: String,
    pub monitor: String,
    pub refresh: Vec<String>,
}

impl UiDisplay {
    pub fn from(displays: Vec<Display>) -> Vec<Self> {
        displays
            .into_iter()
            .enumerate()
            .map(|(i, d)| Self {
                device: d.device,
                monitor: format!("{}. {}", i + 1, d.monitor),
                refresh: d.refresh.into_iter().map(|r| format!("{r} Hz")).collect(),
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");

    let displays = UiDisplay::from(Display::query().expect("Failed to query display infos"));

    for display in &displays {
        println!("Device: {} | Monitor: {}", display.device, display.monitor);
    }

    if let Err(error) = window(&displays) {
        eprintln!("Window error: {error}");
    }
}

fn window(displays: &[UiDisplay]) -> Result<(), Box<dyn std::error::Error>> {
    let monitors = Rc::new(VecModel::from(
        displays
            .iter()
            .map(|d| SharedString::from(&d.monitor))
            .collect::<Vec<SharedString>>(),
    ));

    let refresh_rates = displays
        .iter()
        .map(|d| {
            (
                d.monitor.clone(),
                Rc::new(VecModel::from(
                    d.refresh
                        .iter()
                        .map(SharedString::from)
                        .collect::<Vec<SharedString>>(),
                )),
            )
        })
        .collect::<HashMap<String, Rc<VecModel<SharedString>>>>();

    let ui = SettingsWindow::new()?;

    let config = CurrentConfig {
        applications: ModelRc::from(Rc::new(VecModel::from(vec![]))),
        monitor: SharedString::from(&displays[0].monitor),
        refresh: SharedString::from(&displays[0].refresh[0]),
    };

    ui.set_config(config);

    ui.global::<Backend>()
        .on_monitors(move || ModelRc::from(monitors.clone()));

    ui.global::<Backend>().on_refresh_rates(move |monitor| {
        if let Some(rates) = refresh_rates.get(monitor.as_str()) {
            ModelRc::from(rates.clone())
        } else {
            ModelRc::default()
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_apply(move || {
        let ui = ui_handle.unwrap();
        println!("Apply : {:?}", ui.get_config());
    });

    // let ui_handle = ui.as_weak();
    // ui.on_request_increase_value(move || {
    //     let ui = ui_handle.unwrap();
    //     ui.set_counter(ui.get_counter() + 1);
    // });

    ui.run()?;
    Ok(())
}
/*
fn monitors(displays: &[UiDisplay]) -> Rc<> {

}

fn refresh_rates(displays: &[UiDisplay]) {

}
*/
