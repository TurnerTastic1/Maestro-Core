use thiserror::Error;

#[derive(Error, Debug)]
pub enum MaestroError {
    #[error("Serde error: {0}")]
    SerdeError(String),
    #[error("Config error: {0}")]
    ConfigError(String),
}
