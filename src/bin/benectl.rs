use std::{fs::{create_dir_all, File}, io::Read, path::{Path, PathBuf}};
use std::process::Command;
use bene_gesserit::{cli::{generate, start, stop, reload}, config::Config, environment::EnvConfig, error::BGError, template, corpus};
use clap::{Parser, Subcommand};
use figment::{Figment, providers::{Format, Toml, Env}};

/// Management CLI for the bene-gesserit AI scraper defense proxy. This can be
/// used with a running instance of Bene Gesserit to manage certain components
/// during execution.
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
    template_dir: String,

    #[command(subcommand)]
    command: Subcmd
}

/// Possible commands.
#[derive(Subcommand)]
enum Subcmd {
    /// Generate the configuration for underlying services.
    Generate,
    /// Start running the Bene Gesserit stack.
    Start {
        /// Will not regenerate the config automatically if set.
        #[arg(short, long, default_value = "false")]
        keep_config: bool,
    },
    /// Reload the Bene Gesserit stack. Run this after changing configuration.
    Reload {
        /// Will not regenerate the config automatically if set.
        #[arg(short, long, default_value = "false")]
        keep_config: bool,
    },
    /// Stop running the Bene Gesserit stack.
    Stop
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
    let env: EnvConfig = config.env.config();
    log::debug!("Environment: {:?}", env);

    match &cli.command {
        Subcmd::Generate => {
            generate(&config, cli.static_dir, cli.template_dir)
        },
        Subcmd::Start {keep_config} => {
            if !keep_config {
                generate(&config, cli.static_dir, cli.template_dir)?;
            }
            start(&config)
        },
        Subcmd::Reload {keep_config} => {
            if !keep_config {
                generate(&config, cli.static_dir, cli.template_dir)?;
            }
            reload(&config)
        },
        Subcmd::Stop => {
            stop(&config)
        }
    }
}
