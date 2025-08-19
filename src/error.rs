use thiserror::Error;
use std::io;

/// Catchall error enumeration.
#[derive(Debug, Error)]
pub enum BGError {
    #[error("Template rendering error: {0}")]
    TemplatingError(#[from] tera::Error),
    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Config parse error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "http")]
    #[error("HTTP Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(feature = "cli")]
    #[error("Command error: {0}")]
    ClapError(#[from] clap::Error),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("{0}")]
    AppError(String)
}