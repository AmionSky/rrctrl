use std::mem::size_of;

use windows_sys::Win32::Graphics::Gdi::{EnumDisplaySettingsW, DEVMODEW, ENUM_CURRENT_SETTINGS};

pub unsafe fn get_refresh_rates(device: &[u16]) -> Vec<u32> {
    let mut settings: DEVMODEW = std::mem::zeroed();
    settings.dmSize = size_of::<DEVMODEW>() as u16;

    if EnumDisplaySettingsW(device.as_ptr(), ENUM_CURRENT_SETTINGS, &mut settings) == 0 {
        return Vec::new();
    }

    let resw = settings.dmPelsWidth;
    let resh = settings.dmPelsHeight;

    let mut rates = Vec::with_capacity(1);
    let mut index = 0u32;

    while EnumDisplaySettingsW(device.as_ptr(), index, &mut settings) != 0 {
        if settings.dmPelsWidth == resw && settings.dmPelsHeight == resh {
            rates.push(settings.dmDisplayFrequency);
        }
        index += 1;
    }

    rates.sort_unstable();
    rates.dedup();
    rates.reverse();
    rates
}
