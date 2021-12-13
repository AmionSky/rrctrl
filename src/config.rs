use serde::Deserialize;
use std::fs::File;
use std::{error::Error, io::Read, path::Path};

#[derive(Deserialize, Debug, Clone)]
struct ConfigData {
    pub base_refresh: u32,
    pub target_refresh: u32,
    pub check_interval: u64,
    pub apps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub base_refresh: u32,
    pub target_refresh: u32,
    pub check_interval: u64,
    pub apps: Vec<Vec<u16>>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut buffer = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;

        let data = toml::from_slice(&buffer)?;

        Ok(Self::from_data(data))
    }

    fn from_data(data: ConfigData) -> Self {
        let mut apps = Vec::with_capacity(data.apps.len());

        for app in data.apps {
            apps.push(r"\".encode_utf16().chain(app.encode_utf16()).collect());
        }

        Self {
            base_refresh: data.base_refresh,
            target_refresh: data.target_refresh,
            check_interval: data.check_interval,
            apps,
        }
    }
}
