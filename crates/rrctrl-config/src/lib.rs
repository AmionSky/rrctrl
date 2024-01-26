use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub display_name: String,
    pub target_refresh: u32,
    pub check_interval: u64,
    pub apps: Vec<String>,
}

impl Config {
    pub fn path() -> std::path::PathBuf {
        let mut path = std::env::current_exe().expect("Failed to get current executable path");
        path.set_file_name("config.toml");
        path
    }
}
