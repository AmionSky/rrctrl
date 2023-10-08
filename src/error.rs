use std::error::Error;
use std::fmt::Display;

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
            ConfigError::DefaultPath(error) => write!(f, "Failed to get config path ({error})"),
            ConfigError::FileOpen(error) => write!(f, "Failed to open config file ({error})"),
            ConfigError::FileRead(error) => write!(f, "Failed to read config file ({error})"),
            ConfigError::Deserialize(error) => write!(f, "Failed to deserialize config ({error})"),
        }
    }
}
