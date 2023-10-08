use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::path::PathBuf;
use std::{io::Read, path::Path};

use crate::error::ConfigError;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub display_index: u32,
    pub target_refresh: u32,
    pub check_interval: u64,
    #[serde(deserialize_with = "string_to_native")]
    pub apps: Vec<Vec<u16>>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let mut buffer = String::new();
        let mut file = File::open(path).map_err(ConfigError::FileOpen)?;
        file.read_to_string(&mut buffer).map_err(ConfigError::FileRead)?;
        toml::from_str(&buffer).map_err(ConfigError::Deserialize)
    }
}

fn string_to_native<'de, D>(deserializer: D) -> Result<Vec<Vec<u16>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RawData {
        Data(Vec<String>),
    }

    let RawData::Data(strvec) = RawData::deserialize(deserializer)?;
    let mut uft16vec = Vec::with_capacity(strvec.len());

    for exe in strvec {
        uft16vec.push(r"\".encode_utf16().chain(exe.encode_utf16()).collect());
    }

    Ok(uft16vec)
}

pub fn default_path() -> Result<PathBuf, ConfigError> {
    let mut path = std::env::current_exe().map_err(ConfigError::DefaultPath)?;
    path.set_file_name("config.toml");
    Ok(path)
}
