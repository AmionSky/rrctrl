use windows::core::HRESULT;

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
    Win32Error(HRESULT),
    UnknownError,
}

impl std::error::Error for ProcessError {}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProcessError::InvalidParameter => write!(f, "Invalid parameter"),
            ProcessError::AccessDenied => write!(f, "Access denied"),
            ProcessError::Win32Error(e) => write!(f, "E{}: {}", e.0, e.message()),
            ProcessError::UnknownError => write!(f, "Unknown error"),
        }
    }
}
