use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use regex::Regex;
use crate::core::model::error::MaestroError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) workspace_path: String,
    pub(crate) last_updated: Option<DateTime<Utc>>
}

impl Workspace {

    /// Validates the workspace by checking if the name, and workspace path are not empty. The name must also have no whitespace.
    pub(crate) fn validate(&self) -> Result<(), MaestroError> {
        let re = Regex::new(r"^\w+$").unwrap();
        if !re.is_match(&self.name) {
            return Err(
                MaestroError::MaestroConfigValidationError("Name must be a single word (underscores are allowed)".to_string())
            );
        }
        if self.name.is_empty() {
            return Err(
                MaestroError::MaestroConfigValidationError("Name cannot be empty".to_string())
            );
        }
        if self.workspace_path.is_empty() {
            return Err(
                MaestroError::MaestroConfigValidationError("Workspace path cannot be empty".to_string())
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_workspace() {
        let workspace = Workspace {
            name: "WorkspaceA".to_string(),
            description: "Description for workspaceA".to_string(),
            workspace_path: "/path/to/workspaceA".to_string(),
            last_updated: None
        };
        assert!(workspace.validate().is_ok());
    }

    #[test]
    fn test_invalid_name_workspace() {
        let workspace = Workspace {
            name: "Workspace A".to_string(),
            description: "Description for workspaceA".to_string(),
            workspace_path: "workspace".to_string(),
            last_updated: None
        };
        assert!(workspace.validate().is_err());
        assert!(workspace.validate().unwrap_err().to_string().contains("Name must be a single word"));
    }

    #[test]
    fn test_empty_workspace_path_workspace() {
        let workspace = Workspace {
            name: "WorkspaceA".to_string(),
            description: "Description for workspaceA".to_string(),
            workspace_path: "".to_string(),
            last_updated: None
        };
        assert!(workspace.validate().is_err());
        assert!(workspace.validate().unwrap_err().to_string().contains("Workspace path cannot be empty"));
    }
}