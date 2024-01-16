use hotreload::HotreloadApply;
use serde::{Deserialize, Deserializer};
use std::path::PathBuf;
use std::sync::RwLock;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub display_index: u32,
    pub target_refresh: u32,
    pub check_interval: u64,
    #[serde(deserialize_with = "string_to_native")]
    pub apps: Vec<Vec<u16>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display_index: 0,
            target_refresh: 0,
            check_interval: 60,
            apps: Default::default(),
        }
    }
}

impl HotreloadApply<Config> for RwLock<Config> {
    fn apply(&self, data: Config) {
        println!("Reloading config");
        *self.write().unwrap() = data;
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

pub fn default_path() -> PathBuf {
    let mut path = std::env::current_exe().expect("Failed to get current executable path");
    path.set_file_name("config.toml");
    path
}
