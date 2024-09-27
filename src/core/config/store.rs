use std::fs::File;
use std::path::{Path, PathBuf};
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

pub fn save_config(user_config_path: String) -> Result<String, String> {
    let path_buf = PathBuf::from("maestro.json");
    let file = File::create(&path_buf).map_err(|err|
        format!(
            "Failed to configure Maestro: {}\nEnsure Maestro has write permissions and reconfigure",
            err
        )
    )?;

    let config = Config::new(user_config_path.clone());

    serde_json::to_writer_pretty(&file, &config).map_err(|err|
        format!(
            "Failed to configure Maestro: {}",
            err
        )
    )?;

    Ok(user_config_path)
}