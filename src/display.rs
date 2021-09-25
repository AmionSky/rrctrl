use crate::error::DisplayError;
use bindings::Windows::Win32::{
    Foundation::PWSTR,
    Graphics::Gdi::{
        ChangeDisplaySettingsW, EnumDisplayDevicesW, EnumDisplaySettingsW, CDS_TYPE,
        DISPLAY_DEVICEW, DISP_CHANGE_SUCCESSFUL, ENUM_CURRENT_SETTINGS,
    },
    UI::DisplayDevices::DEVMODEW,
};
use std::{
    mem::size_of,
    ptr::{null, null_mut},
};
use widestring::{U16Str, U16String};

pub struct Display {
    name: U16String,
    settings: DEVMODEW,
}

impl Display {
    pub fn create(device: u32) -> Result<Self, DisplayError> {
        if let Some(name) = get_display_name(device) {
            Ok(Display {
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

    pub fn name(&self) -> String {
        self.name.to_string_lossy()
    }

    pub fn settings(&mut self) -> &mut DEVMODEW {
        &mut self.settings
    }

    pub fn load_settings(&mut self) -> Result<(), DisplayError> {
        if get_display_settings(&self.name, &mut self.settings) {
            Ok(())
        } else {
            Err(DisplayError::EnumSettingsFailed)
        }
    }

    pub fn apply_settings(&self) -> Result<(), DisplayError> {
        if set_display_settings(&self.settings) {
            Ok(())
        } else {
            Err(DisplayError::ChangeSettingsFailed)
        }
    }

    pub fn reset_settings(&self) -> Result<(), DisplayError> {
        if set_display_settings(null()) {
            Ok(())
        } else {
            Err(DisplayError::ChangeSettingsFailed)
        }
    }
}

fn get_display_name(device: u32) -> Option<U16String> {
    let mut display = DISPLAY_DEVICEW {
        cb: size_of::<DISPLAY_DEVICEW>() as u32,
        ..Default::default()
    };

    if unsafe { EnumDisplayDevicesW(PWSTR(null_mut()), device, &mut display, 0).as_bool() } {
        return Some(U16String::from_vec(display.DeviceName));
    }

    None
}

fn get_display_settings(name: &U16Str, settings: &mut DEVMODEW) -> bool {
    unsafe {
        EnumDisplaySettingsW(
            PWSTR(name.as_ptr() as *mut u16),
            ENUM_CURRENT_SETTINGS,
            settings,
        )
        .as_bool()
    }
}

fn set_display_settings(settings: *const DEVMODEW) -> bool {
    unsafe { ChangeDisplaySettingsW(settings, CDS_TYPE(0)) == DISP_CHANGE_SUCCESSFUL }
}
