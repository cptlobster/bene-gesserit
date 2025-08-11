//! Bene Gesserit is a comprehensive defense against LLM scrapers. It uses
//! Anubis as an initial filter for scrapers, and then offers methods to detect
//! more elusive and problematic scrapers, such as "honeypot" endpoints that are
//! explicitly excluded in a robots.txt file or placed in webpages invisibly, or
//! rate limiting. Detected scrapers are sent into an endless Markov-chain
//! generated labyrinth of nonsense text, which will feed scrapers with an
//! endless stream of babble that will hopefully poison LLM training data or
//! break context limits.
//! 
//! This library is the configuration utility that orchestrates all other
//! services. It will populate configuration directories with static files and
//! generate configuration files from templates using parameters passed in the
//! initial configuration file. It will also download corpus files for feeding
//! the labyrinth if they are not present.
use std::{fs::{create_dir_all, File}, io::Read, path::{Path, PathBuf}};
use crate::{config::Config, environment::EnvConfig, error::BGError};
use env_logger::Env;
use std::process::Command;

pub mod config;
pub mod template;
pub mod error;
pub mod corpus;
pub mod environment;

/// Main entrypoint.
fn main() -> Result<(), BGError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Load and parse the configuration file.
    log::debug!("Loading configuration file...");
    let mut cfg_file = File::open("./config.toml")?;
    log::info!("Parsing configuration...");
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;
    let config: Config = toml::from_str(&buf)?;
    let env: EnvConfig = config.env.config();
    log::debug!("Environment: {:?}", env);
    // Copy static files from static directory into target directory.
    log::info!("Cloning static files...");
    copy_if_exists("./static/anubis/.", &env.targets.anubis)?;
    copy_if_exists("./static/iocaine/.", &env.targets.iocaine)?;
    copy_if_exists("./static/nginx/.", &env.targets.nginx)?;
    copy_if_exists("./static/prometheus/.", &env.targets.prometheus)?;
    copy_if_exists("./static/supervisord/.", &env.targets.supervisord)?;
    // Render templated configuration files and place them in the target
    // directory.
    log::info!("Rendering templates...");
    template::render(&config, "./templates/**/*.tera")?;
    // Download corpus files and place them in the target directory.
    log::info!("Downloading corpus files...");
    corpus::get_many(&config.labyrinth.iocaine.corpus, &env.targets.iocaine.join("corpus"))?;
    corpus::get_one(&config.labyrinth.iocaine.words, &env.targets.iocaine.join("corpus"))?;
    // Everything finished correctly, return ok
    log::info!("Configuration generated. Context length is the mind-killer.");
    Ok(())
}

/// Recursively copy a directory's contents from one location to another. It
/// expects a filepath in the format `/path/to/directory/.` in order to copy the
/// contents, without an extra directory. This ensures that the directory paths
/// are predictable.
/// 
/// This function depends on the `cp` utility being present in the running
/// system, as it invokes that command directly.
fn copy_if_exists(src: &str, target: &PathBuf) -> Result<(), BGError> {
    log::debug!("Copying {} to {}...", src, target.to_str().unwrap());
    create_dir_all(target)?;
    if Path::new(src).exists() {
        // Somehow this is the easiest way to do this. I tried messing with 
        // another crate and it didn't work right. please std::fs give me a way
        // to recursively copy a directory
        let mut command = Command::new("cp");
        command.arg("-r");
        command.arg(src);
        command.arg(target.to_str().unwrap());
        let output = command.output()?;
        if output.status.success() {
            Ok(())
        }
        else {
            log::error!("Failed to copy file!");
            log::error!("Status: {}", output.status);
            log::error!("Captured stderr: \n{}", String::from_utf8_lossy(&output.stderr));
            Err(BGError::AppError(format!("cp process call returned non-zero exit code {}", output.status)))
        }
    } else { Ok(()) }
}
