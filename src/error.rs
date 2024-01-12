use std::error::Error;
use std::fmt::Display;
use std::ptr::null;
use windows_sys::Win32::Foundation::{GetLastError, WIN32_ERROR};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
};

#[derive(Debug)]
pub enum DisplayError {
    IncorrectDevice,
    EnumSettingsFailed,
    ChangeSettingsFailed,
}

impl Error for DisplayError {}

impl Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IncorrectDevice => write!(f, "Failed to get display name for specified device"),
            Self::EnumSettingsFailed => write!(f, "Failed to get the display settings"),
            Self::ChangeSettingsFailed => write!(f, "Failed to change the display settings"),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    DefaultPath(std::io::Error),
    FileOpen(std::io::Error),
    FileRead(std::io::Error),
    Deserialize(toml::de::Error),
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DefaultPath(error) => write!(f, "Failed to get config path. {error}"),
            Self::FileOpen(error) => write!(f, "Failed to open config file. {error}"),
            Self::FileRead(error) => write!(f, "Failed to read config file. {error}"),
            Self::Deserialize(error) => write!(f, "Failed to deserialize config. {error}"),
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

    pub fn code(&self) -> WIN32_ERROR {
        self.code
    }
}

impl std::error::Error for WinError {}

impl std::fmt::Display for WinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:X})", self.message, self.code)
    }
}

unsafe fn get_error_message(code: WIN32_ERROR) -> String {
    let mut buffer = [0u16; 4096];
    let length = FormatMessageW(
        FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
        null(),
        code,
        0,
        buffer.as_mut_ptr(),
        buffer.len() as u32,
        null(),
    );
    String::from_utf16_lossy(&buffer[..length as usize])
}
