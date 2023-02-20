use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::{error::Error, io::Read, path::Path};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub display_index: u32,
    pub target_refresh: u32,
    pub check_interval: u64,
    #[serde(deserialize_with = "string_to_native")]
    pub apps: Vec<Vec<u16>>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut buffer = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut buffer)?;
        Ok(toml::from_str(&buffer)?)
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
