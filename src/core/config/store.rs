//! This module contains functions for loading and saving the path to the user's configuration file.
//! The maestro configuration file is a JSON file that contains the path to the user's configuration file, which can be updated at any time by the user.
//!
//! This module includes the following functions:
//! - `load_config` - Loads the user configuration from a file.
//! - `save_user_config_file` - Saves the user configuration to a file.

use std::fs;
use std::fs::File;
use std::path::{PathBuf};
use std::io::{BufReader};
use crate::core::model::config::Config;
use crate::core::model::error::MaestroError;
use crate::core::model::maestro::Maestro;

const MAESTRO_CONFIG_FILE: &str = "maestro.json";

/// Loads the user configuration from a file.
///
/// # Returns
///
/// A `Result` containing a `Maestro` struct if the file is successfully loaded.
///
/// # Errors
///
/// Returns a `MaestroError` if the file cannot be opened or if deserialization fails.
///
/// # Examples
///
/// ```ignore
/// let result = load_config();
/// assert!(result.is_ok());
/// let maestro: Maestro = result.unwrap();
/// assert_eq!(maestro.projects.len(), 2);
/// ```
pub fn load_config() -> Result<Maestro, MaestroError> {
    let path_buf = PathBuf::from(MAESTRO_CONFIG_FILE);
    let maestro_config_file = File::open(&path_buf).map_err(|err| {
        MaestroError::ConfigError(format!(
            "Failed to load Maestro configuration: {}\nEnsure Maestro is configured",
            err
        ))
    })?;

    let reader = BufReader::new(maestro_config_file);
    let config: Config = serde_json::from_reader(reader).map_err(|err| {
        MaestroError::SerdeError(format!(
            "Failed to parse Maestro configuration: {}",
            err
        ))
    })?;

    let user_config_file = File::open(&config.config_file_path).map_err(|err| {
        MaestroError::ConfigError(format!(
            "Failed to load user configuration: {}\nEnsure Maestro is configured",
            err
        ))
    })?;
    let reader = BufReader::new(user_config_file);

    let maestro: Maestro = serde_json::from_reader(reader).map_err(|err| {
        MaestroError::SerdeError(format!(
            "Failed to parse user configuration: {}",
            err
        ))
    })?;

    // Validate the maestro configuration
    maestro.validate()?;

    Ok(maestro)
}

/// Saves the user configuration to a file.
///
/// # Arguments
///
/// * `user_config_path` - A string that holds the path to the user configuration file.
///
/// # Returns
///
/// A `Result` containing the path to the user configuration file if configuration is successful.
///
/// # Errors
///
/// Returns a `MaestroError` if the file cannot be created or if serialization fails.
///
/// # Examples
///
/// ```ignore
/// let result = save_user_config_file("/path/to/config.json".to_string());
/// assert!(result.is_ok());
/// ```
pub fn save_user_config_file(user_config_path: String) -> Result<String, MaestroError> {
    let path_buf = PathBuf::from(MAESTRO_CONFIG_FILE);
    let file = File::create(&path_buf).map_err(|err| {
        MaestroError::ConfigError(format!(
                "Failed to configure Maestro: {}\nEnsure Maestro has write permissions and reconfigure",
                err
        ))
    })?;

    // Convert the path to an absolute path
    let absolute_path = fs::canonicalize(PathBuf::from(user_config_path.to_string()))
        .map_err(|err|
            MaestroError::ConfigError(format!("Failed to canonicalize path '{}': {}\nEnsure the file exists", user_config_path, err))
        )?;

    let config = Config::new(absolute_path.to_str().unwrap().to_string());

    serde_json::to_writer_pretty(&file, &config).map_err(|err| {
        MaestroError::SerdeError(format!(
            "Failed to configure Maestro: {}",
            err
        ))
    })?;

    Ok(absolute_path.to_str().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_save_and_load_config() {
        let user_config_path = "user_config.json".to_string();
        let mut file = File::create(user_config_path.clone()).expect("Failed to create test config file");
        let test_config_content = r#"
        {
            "workspaces": [
                {
                    "name": "WorkspaceA",
                    "description": "Description for workspaceA",
                    "workspace_path": "/path/to/workspaceA"
                },
                {
                    "name": "WorkspaceB",
                    "description": "Description for workspaceB",
                    "workspace_path": "/path/to/workspaceB"
                }
            ]
        }
        "#;

        file.write_all(test_config_content.as_bytes()).expect("Failed to write to test config file");

        // Save the configuration to the maestro config file
        let save_result = save_user_config_file(user_config_path.clone());
        assert!(save_result.is_ok());
        assert!(save_result.unwrap().contains("user_config.json"));

        // Load the configuration from the maestro config file
        let load_result = load_config();
        assert!(load_result.is_ok());
        let maestro = load_result.unwrap();
        assert_eq!(maestro.workspaces.len(), 2);
        assert_eq!(maestro.workspaces[0].name, "WorkspaceA");
        assert_eq!(maestro.workspaces[0].description, "Description for workspaceA");

        // Clean up the files
        fs::remove_file(MAESTRO_CONFIG_FILE ).expect("Failed to delete test maestro config file");
        fs::remove_file("user_config.json").expect("Failed to delete test user config file");
    }

    #[test]
    #[serial]
    fn test_load_config_with_no_saved_file() {
        let load_result = load_config();
        assert!(load_result.is_err());
    }

    #[test]
    #[serial]
    fn test_load_config_with_invalid_json() {
        let user_config_path = "invalid_user_config.json".to_string();
        let mut file = File::create(user_config_path.clone()).expect("Failed to create test config file");
        let test_config_content = r#"
        {
            "invalid": "json"
        }
        "#;

        file.write_all(test_config_content.as_bytes()).expect("Failed to write to test config file");

        // Save the configuration to the maestro config file
        let save_result = save_user_config_file(user_config_path.clone());
        assert!(save_result.is_ok());
        assert!(save_result.unwrap().contains("invalid_user_config.json"));

        // Load the configuration from the maestro config file
        let load_result = load_config();
        println!("{:?}", load_result);
        // Assert: Check that the result is an Err and of type MaestroError::SerdeError
        match load_result {
            Err(MaestroError::SerdeError(_)) => assert!(true),
            _ => assert!(false, "Expected MaestroError::ConfigError"),
        }

        // Clean up the files
        fs::remove_file(MAESTRO_CONFIG_FILE ).expect("Failed to delete test maestro config file");
        fs::remove_file("invalid_user_config.json").expect("Failed to delete test user config file");
    }

    #[test]
    #[serial]
    fn test_load_config_with_invalid_maestro_config() {
        let user_config_path = "invalid_user_config.json".to_string();
        let mut file = File::create(user_config_path.clone()).expect("Failed to create test config file");
        let test_config_content = r#"
        {
            "workspaces": [
                {
                    "name": "WorkspaceA",
                    "description": "Description for workspaceA",
                    "workspace_path": "/path/to/workspaceA"
                },
                {
                    "name": "Workspace B",
                    "description": "Description for workspaceB",
                    "workspace_path": "/path/to/workspaceB"
                }
            ]
        }
        "#;

        file.write_all(test_config_content.as_bytes()).expect("Failed to write to test config file");

        // Save the configuration to the maestro config file
        let save_result = save_user_config_file(user_config_path.clone());
        assert!(save_result.is_ok());
        assert!(save_result.unwrap().contains("invalid_user_config.json"));

        // Load the configuration from the maestro config file
        let load_result = load_config();
        // Assert: Check that the result is an Err and of type MaestroError::ConfigError
        match load_result {
            Err(MaestroError::MaestroConfigValidationError(_)) => assert!(true),
            _ => assert!(false, "Expected MaestroError::ConfigError"),
        }

        // Clean up the files
        fs::remove_file(MAESTRO_CONFIG_FILE ).expect("Failed to delete test maestro config file");
        fs::remove_file("invalid_user_config.json").expect("Failed to delete test user config file");
    }
}
