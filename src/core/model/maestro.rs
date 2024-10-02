use serde::{Serialize, Deserialize};
use crate::core::model::workspace::Workspace;

#[derive(Serialize, Deserialize, Debug)]
pub struct Maestro {
    pub(crate) workspaces: Vec<Workspace>,
}
