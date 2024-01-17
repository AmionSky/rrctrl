use crate::display::Display;
use crate::wstring::WString;
use hotreload::HotreloadApply;
use rrctrl_config::Config;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

pub struct State {
    display: Mutex<Display>,
    check_interval: AtomicU64,
    apps: Mutex<Vec<WString>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            display: Mutex::new(Display::new()),
            check_interval: AtomicU64::new(60),
            apps: Mutex::new(Vec::new()),
        }
    }
}

impl State {
    pub fn display(&self) -> MutexGuard<'_, Display> {
        self.display.lock().expect("State.display mutex paniced.")
    }

    pub fn update_display(&self, name: String, target: u32) {
        let mut display = self.display();
        display.set_name(name);
        display.set_target(target);
    }

    pub fn check_interval(&self) -> u64 {
        self.check_interval.load(Ordering::Acquire)
    }

    pub fn set_check_interval(&self, value: u64) {
        self.check_interval.store(value, Ordering::Release)
    }

    pub fn apps(&self) -> MutexGuard<'_, Vec<WString>> {
        self.apps.lock().expect("State.apps mutex paniced.")
    }

    pub fn set_apps(&self, value: Vec<String>) {
        // Since there should be full filenames and will be compared to running apps,
        // add a '\' to the start of every path
        let mut apps = Vec::with_capacity(value.len());
        for mut string in value {
            if !string.starts_with('\\') {
                string.insert(0, '\\');
            }
            apps.push(WString::new(string))
        }
        *self.apps() = apps;
    }
}

impl HotreloadApply<Config> for State {
    fn apply(&self, data: Config) {
        self.update_display(data.display_name, data.target_refresh);
        self.set_check_interval(data.check_interval);
        self.set_apps(data.apps);
    }
}
