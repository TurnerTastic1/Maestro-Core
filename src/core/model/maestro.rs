use serde::{Serialize, Deserialize};
use crate::core::model::error::MaestroError;
use crate::core::model::workspace::Workspace;

#[derive(Serialize, Deserialize, Debug)]
pub struct Maestro {
    pub workspaces: Vec<Workspace>,
}

impl Maestro {
    pub(crate) fn validate(&self) -> Result<(), MaestroError> {
        for workspace in &self.workspaces {
            workspace.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_maestro() {
        let maestro = Maestro {
            workspaces: vec![
                Workspace {
                    name: "WorkspaceA".to_string(),
                    description: "Description for workspaceA".to_string(),
                    workspace_path: "/path/to/workspaceA".to_string(),
                    container_working_dir: None,
                }
            ]
        };
        assert!(maestro.validate().is_ok());
    }

    #[test]
    fn test_invalid_workspace_in_maestro() {
        let maestro = Maestro {
            workspaces: vec![
                Workspace {
                    name: "WorkspaceA".to_string(),
                    description: "Description for workspaceA".to_string(),
                    workspace_path: "/path/to/workspaceA".to_string(),
                    container_working_dir: None,
                },
                Workspace {
                    name: "Workspace B".to_string(),
                    description: "Description for workspaceB".to_string(),
                    workspace_path: "/path/to/workspaceB".to_string(),
                    container_working_dir: None,
                }
            ]
        };
        assert!(maestro.validate().is_err());
    }
}