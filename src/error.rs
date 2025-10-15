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
    ParseError(#[from] figment::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "http")]
    #[error("HTTP Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(feature = "cli")]
    #[error("Command error: {0}")]
    ClapError(#[from] clap::Error),
    #[cfg(feature = "cli")]
    #[error("UNIX error: {0}")]
    UnixError(#[from] nix::Error),
    #[error("Integer parse error: {0}")]
    IntParseError(#[from] std::num::ParseIntError),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("{0}")]
    AppError(String)
}