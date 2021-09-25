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
