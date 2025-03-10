use std::error::Error;
use std::fmt::Display;
use std::ptr::null;
use windows_sys::Win32::Foundation::{GetLastError, WIN32_ERROR};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW,
};

#[derive(Debug)]
pub enum DisplayError {
    EnumSettingsFailed,
    ChangeSettingsFailed,
}

impl Error for DisplayError {}

impl Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EnumSettingsFailed => write!(f, "Failed to get the display settings"),
            Self::ChangeSettingsFailed => write!(f, "Failed to change the display settings"),
        }
    }
}

#[derive(Debug)]
pub struct WinError {
    code: WIN32_ERROR,
    message: String,
}

impl WinError {
    pub(super) fn last() -> Self {
        unsafe {
            let code = GetLastError();
            let message = get_error_message(code);
            Self { code, message }
        }
    }
}

impl std::error::Error for WinError {}

impl std::fmt::Display for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:X})", self.message, self.code)
    }
}

fn get_error_message(code: WIN32_ERROR) -> String {
    let mut buffer = [0u16; 4096];
    let length = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            null(),
            code,
            0,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            null(),
        )
    };
    String::from_utf16_lossy(&buffer[..length as usize])
}
