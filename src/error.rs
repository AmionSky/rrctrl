use windows::Win32::Foundation::WIN32_ERROR;

#[derive(Debug, Clone)]
pub enum DisplayError {
    IncorrectDevice,
    EnumSettingsFailed,
    ChangeSettingsFailed,
}

impl std::error::Error for DisplayError {}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DisplayError::IncorrectDevice => {
                write!(f, "Failed to get display name for specified device")
            }
            DisplayError::EnumSettingsFailed => write!(f, "Failed to get the display settings"),
            DisplayError::ChangeSettingsFailed => {
                write!(f, "Failed to change the display settings")
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ProcessError {
    InvalidParameter,
    AccessDenied,
    UnknownError(WIN32_ERROR),
}

impl std::error::Error for ProcessError {}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProcessError::InvalidParameter => write!(f, "Invalid parameter"),
            ProcessError::AccessDenied => write!(f, "Access denied"),
            ProcessError::UnknownError(e) => write!(f, "Unknown error: E{}", e.0),
        }
    }
}
