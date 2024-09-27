use std::fs::File;
use std::path::Path;
use std::io::{BufReader};
use crate::core::config::model::Config;

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, String> {
    let file = File::open(path).map_err(|err|
        format!(
            "Failed to open configuration file: {}\nTry reconfiguring Maestro",
            err
        )
    )?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader).map_err(|err|
        format!(
            "Failed to parse JSON when loading configuration: {}\nFix any json errors and reconfigure Maestro",
            err
        )
    )?;
    Ok(config)
}

// pub fn save_config<P: AsRef<Path>>(path: P) -> Result<(), String> {
//
// }