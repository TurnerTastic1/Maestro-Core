use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) workspace_path: String,
    pub(crate) last_updated: Option<DateTime<Utc>>
}
