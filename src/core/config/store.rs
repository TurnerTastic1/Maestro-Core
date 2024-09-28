use std::fs::File;
use std::path::{PathBuf};
use std::io::{BufReader};
use crate::core::model::config::Config;
use crate::core::model::maestro::Maestro;

const MAESTRO_CONFIG_FILE: &str = "maestro.json";

pub fn load_config() -> Result<Maestro, String> {
    let path_buf = PathBuf::from(MAESTRO_CONFIG_FILE);
    let maestro_config_file = File::open(&path_buf).map_err(|err|
        format!(
            "Failed to load Maestro configuration: {}\nEnsure Maestro is configured",
            err
        )
    )?;

    let reader = BufReader::new(maestro_config_file);
    let config: Config = serde_json::from_reader(reader).map_err(|err|
        format!(
            "Failed to parse Maestro configuration: {}",
            err
        )
    )?;

    let user_config_file = File::open(&config.config_file_path).map_err(|err|
        format!(
            "Failed to load user configuration: {}\nEnsure Maestro is configured",
            err
        )
    )?;
    let reader = BufReader::new(user_config_file);

    let maestro: Maestro = serde_json::from_reader(reader).map_err(|err|
        format!(
            "Failed to parse user configuration: {}",
            err
        )
    )?;

    Ok(maestro)
}

pub fn save_user_config_file(user_config_path: String) -> Result<String, String> {
    let path_buf = PathBuf::from(MAESTRO_CONFIG_FILE);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_save_and_load_config() {
        let user_config_path = "user_config.json".to_string();
        let mut file = File::create(user_config_path.clone()).expect("Failed to create test config file");
        let test_config_content = r#"
        {
            "projects": [
                {
                    "name": "Project A",
                    "description": "Description for Project A"
                },
                {
                    "name": "Project B",
                    "description": "Description for Project B"
                }
            ]
        }
        "#;

        file.write_all(test_config_content.as_bytes()).expect("Failed to write to test config file");

        // Save the configuration
        let save_result = save_user_config_file(user_config_path.clone());
        assert!(save_result.is_ok());
        assert_eq!(save_result.unwrap(), user_config_path);



        // Load the configuration
        let load_result = load_config();
        assert!(load_result.is_ok());
        let maestro = load_result.unwrap();
        assert_eq!(maestro.projects.len(), 2);
        assert_eq!(maestro.projects[0].name, "Project A");
        assert_eq!(maestro.projects[0].description, "Description for Project A");

        // Clean up the files
        fs::remove_file(MAESTRO_CONFIG_FILE ).expect("Failed to delete test maestro config file");
        fs::remove_file("user_config.json").expect("Failed to delete test user config file");
    }
}