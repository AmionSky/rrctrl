use winscribe::icon::Icon;
use winscribe::manifest::{DpiMode, Feature, Manifest};
use winscribe::{ResBuilder, ResError};

const ICON: &str = "./assets/application.ico";

fn main() {
    println!("cargo:rerun-if-changed={ICON}");

    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        resource().expect("Failed to include resource!");
    }
}

fn resource() -> Result<(), ResError> {
    ResBuilder::from_env()?
        .push(Icon::app(ICON))
        .push(Manifest::from([
            Feature::DpiAware(DpiMode::PerMonitorV2),
            Feature::ControlsV6,
        ]))
        .compile()
}
