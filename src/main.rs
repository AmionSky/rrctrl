use bindings::Windows::Win32::Foundation::PWSTR;
use bindings::Windows::Win32::Graphics::Gdi::{
    EnumDisplayDevicesW, EnumDisplaySettingsW, DISPLAY_DEVICEW, ENUM_CURRENT_SETTINGS,
};
use bindings::Windows::Win32::UI::DisplayDevices::DEVMODEW;
use std::{mem::size_of, ptr::null_mut};
use widestring::{U16Str, U16String};

fn main() {
    println!("Hello, world!");
    let dn = get_display_name(0).unwrap();
    println!("Device name: {}", dn.to_string().unwrap());
    let mut dm = DEVMODEW::default();
    get_display_settings(&dn, &mut dm);
    println!("Display refresh rate: {}", dm.dmDisplayFrequency);
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

fn get_display_settings(display_name: &U16Str, display_settings: &mut DEVMODEW) -> bool {
    display_settings.dmSize = size_of::<DEVMODEW>() as u16;
    display_settings.dmDriverExtra = 0;

    unsafe {
        EnumDisplaySettingsW(
            PWSTR(display_name.as_ptr() as *mut u16),
            ENUM_CURRENT_SETTINGS,
            display_settings,
        )
        .as_bool()
    }
}
