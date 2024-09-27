use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    config_file_path: String,
}

impl Config {
    pub fn new(config_file_path: String) -> Self {
        Self {
            config_file_path,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_file_path: "default.json".to_string(),
        }
    }
}
