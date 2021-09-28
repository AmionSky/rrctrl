use std::{error::Error, io::Read, path::Path};

use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize,Debug, Clone)]
pub struct Config {
    pub base_refresh: u32,
    pub target_refresh: u32,
    pub check_interval: u64,
    pub apps: Vec<String>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P)->Result<Self,Box<dyn Error>> {
        let mut buffer = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;
        Ok(toml::from_slice(&buffer)?)
    }
}