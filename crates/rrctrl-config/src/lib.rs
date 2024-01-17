use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub display_name: String,
    pub target_refresh: u32,
    pub check_interval: u64,
    pub apps: Vec<String>,
}
