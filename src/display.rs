use crate::error::DisplayError;
use std::mem::size_of;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::{
    ChangeDisplaySettingsW, EnumDisplayDevicesW, EnumDisplaySettingsW, CDS_TYPE, DEVMODEW,
    DISPLAY_DEVICEW, DISP_CHANGE_SUCCESSFUL, DM_DISPLAYFREQUENCY, ENUM_CURRENT_SETTINGS,
};

type DisplayName = [u16; 33];

pub struct Display {
    name: DisplayName,
    settings: DEVMODEW,
}

impl Display {
    pub fn create(device: u32) -> Result<Self, DisplayError> {
        if let Some(name) = get_display_name(device) {
            Ok(Self {
                name,
                settings: DEVMODEW {
                    dmSize: size_of::<DEVMODEW>() as u16,
                    dmDriverExtra: 0,
                    ..Default::default()
                },
            })
        } else {
            Err(DisplayError::IncorrectDevice)
        }
    }

    pub fn refresh(&self) -> u32 {
        self.settings.dmDisplayFrequency
    }

    pub fn set_refresh(&mut self, rate: u32) {
        println!("RR: {}", self.settings.dmPelsWidth);
        self.settings.dmDisplayFrequency = rate;
        self.settings.dmFields = DM_DISPLAYFREQUENCY;
    }

    pub fn load_settings(&mut self) -> Result<(), DisplayError> {
        if get_display_settings(&self.name, &mut self.settings) {
            Ok(())
        } else {
            Err(DisplayError::EnumSettingsFailed)
        }
    }

    pub fn apply_settings(&self) -> Result<(), DisplayError> {
        if set_display_settings(Some(&self.settings)) {
            Ok(())
        } else {
            Err(DisplayError::ChangeSettingsFailed)
        }
    }

    pub fn reset_settings(&self) -> Result<(), DisplayError> {
        if set_display_settings(None) {
            Ok(())
        } else {
            Err(DisplayError::ChangeSettingsFailed)
        }
    }
}

fn get_display_name(device: u32) -> Option<DisplayName> {
    let mut display = DISPLAY_DEVICEW {
        cb: size_of::<DISPLAY_DEVICEW>() as u32,
        ..Default::default()
    };

    if unsafe { EnumDisplayDevicesW(PCWSTR::null(), device, &mut display, 0).as_bool() } {
        let mut name: DisplayName = [0; 33];
        name[..32].copy_from_slice(&display.DeviceName);
        return Some(name);
    }

    None
}

fn get_display_settings(name: &DisplayName, settings: &mut DEVMODEW) -> bool {
    unsafe {
        EnumDisplaySettingsW(
            PCWSTR::from_raw(name.as_ptr()),
            ENUM_CURRENT_SETTINGS,
            settings,
        )
        .as_bool()
    }
}

fn set_display_settings(settings: Option<*const DEVMODEW>) -> bool {
    unsafe { ChangeDisplaySettingsW(settings, CDS_TYPE(0)) == DISP_CHANGE_SUCCESSFUL }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of_val;

    use windows::w;

    use super::*;

    #[test]
    fn display_settings_load() {
        unsafe {
            let mut dm = DEVMODEW::default();
            dm.dmSize = size_of::<DEVMODEW>() as u16;

            let display = w!("\\\\.\\Display1");

            let res = EnumDisplaySettingsW(display, ENUM_CURRENT_SETTINGS, &mut dm);

            println!("RES: {}", res.0);
            println!(
                "DATA: {}x{}@{}hz",
                dm.dmPelsWidth, dm.dmPelsHeight, dm.dmDisplayFrequency
            );
            panic!()
        }

        //let mut display = Display::create(0).unwrap();
        //display.load_settings().unwrap();
        //assert_ne!(display.refresh(), 0);
    }
}
