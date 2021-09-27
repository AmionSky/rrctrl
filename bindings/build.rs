fn main() {
    windows::build! {
        Windows::Win32::Graphics::Gdi::EnumDisplayDevicesW,
        Windows::Win32::Graphics::Gdi::EnumDisplaySettingsW,
        Windows::Win32::Graphics::Gdi::ChangeDisplaySettingsW,
        Windows::Win32::System::ProcessStatus::K32EnumProcesses,
        Windows::Win32::System::ProcessStatus::K32GetProcessImageFileNameW,
        Windows::Win32::System::Threading::OpenProcess,
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::Foundation::MAX_PATH,
    };
}
