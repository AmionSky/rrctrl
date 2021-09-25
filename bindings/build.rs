fn main() {
    windows::build! {
        Windows::Win32::Graphics::Gdi::EnumDisplayDevicesW,
        Windows::Win32::Graphics::Gdi::EnumDisplaySettingsW,
        Windows::Win32::Graphics::Gdi::ChangeDisplaySettingsW,
    };
}
