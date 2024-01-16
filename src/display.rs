use crate::error::DisplayError;
use crate::wstring::WString;
use std::mem::size_of;
use std::ptr::null;
use windows_sys::Win32::Graphics::Gdi::{
    ChangeDisplaySettingsW, EnumDisplayDevicesW, EnumDisplaySettingsW, DEVMODEW, DISPLAY_DEVICEW,
    DISP_CHANGE_SUCCESSFUL, DM_DISPLAYFREQUENCY, ENUM_CURRENT_SETTINGS,
};

pub struct Display {
    name: WString,
    target: u32,
    active: bool,
    settings: DEVMODEW,
}

impl Display {
    pub fn new() -> Self {
        let mut settings: DEVMODEW = unsafe { std::mem::zeroed() };
        settings.dmSize = size_of::<DEVMODEW>() as u16;

        Self {
            name: get_display_name(0),
            target: 0,
            active: false,
            settings,
        }
    }

    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        let new_name = WString::new(name);
        if self.name != new_name {
            if self.active {
                self.deactivate();
            }
            self.name = new_name;
        }
    }

    pub fn set_target(&mut self, value: u32) {
        if self.target != value {
            self.target = value;
            if self.active {
                self.activate();
            }
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self) {
        if let Err(error) = self.action_activate() {
            eprintln!("Failed to activate: {}", error);
        }
    }

    pub fn deactivate(&mut self) {
        if let Err(error) = self.action_deactivate() {
            eprintln!("Failed to deactivate: {}", error);
        }
    }

    fn action_activate(&mut self) -> Result<(), DisplayError> {
        self.load_settings()?;
        if self.refresh() != self.target {
            self.set_refresh(self.target);
            self.apply_settings()?;
            self.active = true;
            println!("Applied new refresh rate: {}", self.target);
        }
        Ok(())
    }

    fn action_deactivate(&mut self) -> Result<(), DisplayError> {
        self.reset_settings()?;
        self.active = false;
        println!("Reset to the original refresh rate");
        Ok(())
    }

    fn refresh(&self) -> u32 {
        self.settings.dmDisplayFrequency
    }

    fn set_refresh(&mut self, rate: u32) {
        self.settings.dmDisplayFrequency = rate;
        self.settings.dmFields = DM_DISPLAYFREQUENCY;
    }

    fn load_settings(&mut self) -> Result<(), DisplayError> {
        match get_display_settings(self.name.as_ptr(), &mut self.settings) {
            true => Ok(()),
            false => Err(DisplayError::EnumSettingsFailed),
        }
    }

    fn apply_settings(&self) -> Result<(), DisplayError> {
        match set_display_settings(Some(&self.settings)) {
            true => Ok(()),
            false => Err(DisplayError::ChangeSettingsFailed),
        }
    }

    fn reset_settings(&self) -> Result<(), DisplayError> {
        match set_display_settings(None) {
            true => Ok(()),
            false => Err(DisplayError::ChangeSettingsFailed),
        }
    }
}

fn get_display_name(device: u32) -> WString {
    let mut display: DISPLAY_DEVICEW = unsafe { std::mem::zeroed() };
    display.cb = size_of::<DISPLAY_DEVICEW>() as u32;

    match unsafe { EnumDisplayDevicesW(null(), device, &mut display, 0) } {
        0 => WString::new("GetDisplayNameError"),
        _ => WString::from_uft16(&display.DeviceName),
    }
}

fn get_display_settings(name: *const u16, settings: &mut DEVMODEW) -> bool {
    unsafe { EnumDisplaySettingsW(name, ENUM_CURRENT_SETTINGS, settings) != 0 }
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
