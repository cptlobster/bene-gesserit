use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum BGError {
    #[error("Template rendering error: {0}")]
    TemplatingError(#[from] tera::Error),
    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Config parse error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("{0}")]
    AppError(String)
}