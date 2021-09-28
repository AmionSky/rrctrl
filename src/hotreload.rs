use std::error::Error;
use std::path::Path;
use notify::{Watcher,Config as NotifyConfig, RecommendedWatcher, RecursiveMode};
use crate::config::Config;
use crate::RuntimeConfig;

pub fn watch<P: AsRef<Path>>(path: P, config: RuntimeConfig)->Result<RecommendedWatcher,Box<dyn Error>>{
    let mut watcher = RecommendedWatcher::new(move |res| {
        match res {
           Ok(_) => if let Err(error) = reload_config(&config) {
               eprintln!("Failed to reload config (Maybe reload is too fast): {}",error);
           },
           Err(e) => eprintln!("Watch error: {:?}", e),
        }
    })?;

    watcher.configure(NotifyConfig::NoticeEvents(false))?;
    watcher.configure(NotifyConfig::OngoingEvents(None))?;
    watcher.configure(NotifyConfig::PreciseEvents(false))?;
    
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    Ok(watcher)
}

fn reload_config(config: &RuntimeConfig)->Result<(),Box<dyn Error>> {
    println!("Reloading config");

    let path = crate::config_path()?;
    let new_config = Config::load(path)?;
    *config.write().unwrap() = new_config;

    Ok(())
}