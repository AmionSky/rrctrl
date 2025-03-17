use std::error::Error;
use std::fmt::Display;
use windows_sys::Win32::Foundation::{GetLastError, WIN32_ERROR};

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
}

impl WinError {
    pub(super) fn last() -> Self {
        unsafe {
            Self {
                code: GetLastError(),
            }
        }
    }
}

impl std::error::Error for WinError {}

impl std::fmt::Display for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Win32 Error {:X}", self.code)
    }
}
