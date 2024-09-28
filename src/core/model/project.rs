use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub(crate) name: String,
    pub(crate) description: String,
}
