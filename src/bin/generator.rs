use std::{fs::{create_dir_all, File}, io::Read, path::{Path, PathBuf}};
use bene_gesserit::{cli::generate, config::Config, environment::EnvConfig, error::BGError, template, corpus};
use figment::{Figment, providers::{Format, Toml, Env}};
use std::process::Command;
use clap::Parser;

/// Configuration generator for the bene-gesserit AI scraper defense proxy.
#[derive(Parser)]
struct Cli {
    /// The configuration file to read from.
    #[arg(short = 'f', long, default_value = "./config.toml")]
    cfg_file: PathBuf,
    /// The directory to read static files from. DO NOT INCLUDE A TRAILING SLASH.
    #[arg(long, default_value = "./static")]
    static_dir: String,
    /// The directory to read template files from. DO NOT INCLUDE A TRAILING SLASH.
    #[arg(long, default_value = "./templates")]
    template_dir: String
}

/// Main entrypoint.
fn main() -> Result<(), BGError> {
    let cli = Cli::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // Load and parse the configuration file.
    log::debug!("Loading configuration file...");
    let config: Config = Figment::new()
        .merge(Toml::file(cli.cfg_file))
        .merge(Env::prefixed("BG_"))
        .extract()?;
    generate(&config, cli.static_dir, cli.template_dir)
}