use crate::config::Config;
use crate::RuntimeConfig;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::error::Error;
use std::path::Path;

pub fn watch<P: AsRef<Path>>(
    path: P,
    config: RuntimeConfig,
) -> Result<RecommendedWatcher, Box<dyn Error>> {
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| match res {
        Ok(event) => {
            if event.kind.is_modify() {
                if let Err(error) = reload_config(&config) {
                    eprintln!(
                        "Failed to reload config (Maybe reload is too fast): {}",
                        error
                    );
                }
            }
        }
        Err(e) => eprintln!("Watch error: {:?}", e),
    })?;

    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    Ok(watcher)
}

fn reload_config(config: &RuntimeConfig) -> Result<(), Box<dyn Error>> {
    println!("Reloading config");

    let path = crate::config_path()?;
    let new_config = Config::load(path)?;
    *config.write().unwrap() = new_config;

    Ok(())
}
