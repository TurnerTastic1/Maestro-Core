use serde::{Serialize, Deserialize};
use crate::core::model::project::Project;

#[derive(Serialize, Deserialize, Debug)]
pub struct Maestro {
    pub(crate) projects: Vec<Project>,
}
