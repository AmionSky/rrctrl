mod display;
use std::collections::HashMap;
use std::rc::Rc;

use slint::{ModelRc, SharedString, VecModel};

use crate::display::Display;

slint::include_modules!();

fn main() {
    println!("Hello, world!");

    let displays = Display::query().unwrap();

    for display in &displays {
        println!("Device: {} | Monitor: {}", display.device, display.monitor);
    }

    if let Err(error) = window(displays) {
        eprintln!("Window error: {error}");
    }
}

fn window(displays: Vec<Display>) -> Result<(), Box<dyn std::error::Error>> {
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
                d.monitor.to_string(),
                Rc::new(VecModel::from(vec![
                    "144".into(),
                    "120".into(),
                    "60".into(),
                ])),
            )
        })
        .collect::<HashMap<String, Rc<VecModel<SharedString>>>>();

    let ui = SettingsWindow::new()?;

    let config = CurrentConfig {
        applications: ModelRc::from(Rc::new(VecModel::from(vec![]))),
        monitor: SharedString::from(&displays[0].monitor),
        target_refresh: 120,
    };

    ui.set_config(config);

    ui.global::<Backend>()
        .on_monitors(move || ModelRc::from(monitors.clone()));

    ui.global::<Backend>()
        .on_refresh_rates(move |monitor| {
            if let Some(rates) = refresh_rates.get(&monitor.to_string()) {
                ModelRc::from(rates.clone())
            } else {
                ModelRc::from(Rc::new(VecModel::from(vec![])))
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
