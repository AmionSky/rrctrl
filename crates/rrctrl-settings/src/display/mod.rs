mod config;
mod info;
mod refresh;

use windows_sys::Win32::Devices::Display::DISPLAYCONFIG_PATH_INFO;

pub struct Display {
    pub device: String,
    pub monitor: String,
    pub refresh: Vec<u32>,
}

impl Display {
    pub fn query() -> Result<Vec<Self>, &'static str> {
        let configs = self::config::DisplayConfigs::query()?;
        let displays = configs
            .paths
            .into_iter()
            .filter_map(|path| Self::try_from(path).ok())
            .collect();
        Ok(displays)
    }
}

impl TryFrom<DISPLAYCONFIG_PATH_INFO> for Display {
    type Error = i32;

    fn try_from(path: DISPLAYCONFIG_PATH_INFO) -> Result<Self, Self::Error> {
        let device = self::info::get_device_name(path.sourceInfo)?;
        let monitor = self::info::get_monitor_name(path.targetInfo)?;
        let rates = unsafe { self::refresh::get_refresh_rates(&device) };

        Ok(Self {
            device: utf16_to_string(&device),
            monitor: utf16_to_string(&monitor),
            refresh: rates,
        })
    }
}

fn utf16_to_string(utf16: &[u16]) -> String {
    let mut name = String::from_utf16_lossy(utf16);
    name.retain(|c| c != '\0');
    name
}
