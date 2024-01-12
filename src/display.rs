use crate::error::DisplayError;
use std::mem::size_of;
use std::ptr::null;
use windows_sys::Win32::Graphics::Gdi::{
    ChangeDisplaySettingsW, EnumDisplayDevicesW, EnumDisplaySettingsW, DEVMODEW, DISPLAY_DEVICEW,
    DISP_CHANGE_SUCCESSFUL, DM_DISPLAYFREQUENCY, ENUM_CURRENT_SETTINGS,
};

type DisplayName = [u16; 33];

pub struct Display {
    name: DisplayName,
    settings: DEVMODEW,
}

impl Display {
    pub fn create(device: u32) -> Result<Self, DisplayError> {
        match get_display_name(device) {
            Some(name) => {
                let mut settings: DEVMODEW = unsafe { std::mem::zeroed() };
                settings.dmSize = size_of::<DEVMODEW>() as u16;
                Ok(Self { name, settings })
            }
            None => Err(DisplayError::IncorrectDevice),
        }
    }

    pub fn refresh(&self) -> u32 {
        self.settings.dmDisplayFrequency
    }

    pub fn set_refresh(&mut self, rate: u32) {
        self.settings.dmDisplayFrequency = rate;
        self.settings.dmFields = DM_DISPLAYFREQUENCY;
    }

    pub fn load_settings(&mut self) -> Result<(), DisplayError> {
        match get_display_settings(&self.name, &mut self.settings) {
            true => Ok(()),
            false => Err(DisplayError::EnumSettingsFailed),
        }
    }

    pub fn apply_settings(&self) -> Result<(), DisplayError> {
        match set_display_settings(Some(&self.settings)) {
            true => Ok(()),
            false => Err(DisplayError::ChangeSettingsFailed),
        }
    }

    pub fn reset_settings(&self) -> Result<(), DisplayError> {
        match set_display_settings(None) {
            true => Ok(()),
            false => Err(DisplayError::ChangeSettingsFailed),
        }
    }
}

fn get_display_name(device: u32) -> Option<DisplayName> {
    let mut display: DISPLAY_DEVICEW = unsafe { std::mem::zeroed() };
    display.cb = size_of::<DISPLAY_DEVICEW>() as u32;

    match unsafe { EnumDisplayDevicesW(null(), device, &mut display, 0) } {
        0 => None,
        _ => {
            let mut name: DisplayName = [0; 33];
            name[..32].copy_from_slice(&display.DeviceName);
            Some(name)
        }
    }
}

fn get_display_settings(name: &DisplayName, settings: &mut DEVMODEW) -> bool {
    unsafe { EnumDisplaySettingsW(name.as_ptr(), ENUM_CURRENT_SETTINGS, settings) != 0 }
}

fn set_display_settings(settings: Option<*const DEVMODEW>) -> bool {
    unsafe { ChangeDisplaySettingsW(settings.unwrap_or(null()), 0) == DISP_CHANGE_SUCCESSFUL }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_devmode_struct() {
        let mut dm: DEVMODEW = unsafe { std::mem::zeroed() };
        dm.dmSize = size_of::<DEVMODEW>() as u16;

        let display = windows_sys::core::w!("\\\\.\\Display1");
        assert!(unsafe { EnumDisplaySettingsW(display, ENUM_CURRENT_SETTINGS, &mut dm) } != 0);

        println!(
            "Display1: {}x{}@{}hz",
            dm.dmPelsWidth, dm.dmPelsHeight, dm.dmDisplayFrequency
        );

        assert!(dm.dmPelsWidth > dm.dmPelsHeight);
        assert!(dm.dmPelsHeight > dm.dmDisplayFrequency);
    }
}
