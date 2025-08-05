use std::{fs::{create_dir_all, File}, io::Read, path::{Path, PathBuf}};
use crate::{config::Config, error::BGError};
use env_logger::Env;
use fs_extra::{copy_items, dir};
use std::process::Command;

pub mod config;
pub mod template;
pub mod error;
pub mod corpus;

fn main() -> Result<(), BGError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::debug!("Loading configuration file...");
    let mut cfg_file = File::open("./config.toml")?;
    log::info!("Parsing configuration...");
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;
    let config: Config = toml::from_str(&buf)?;
    log::info!("Cloning static files...");
    copy_if_exists("./static/anubis/.", &config.targets.anubis)?;
    copy_if_exists("./static/iocaine/.", &config.targets.iocaine)?;
    copy_if_exists("./static/nginx/.", &config.targets.nginx)?;
    copy_if_exists("./static/prometheus/.", &config.targets.prometheus)?;
    copy_if_exists("./static/supervisord/.", &config.targets.supervisord)?;
    log::info!("Rendering templates...");
    template::render(&config, "./templates/**/*.tera")?;
    log::info!("Downloading corpus files...");
    corpus::get_many(&config.labyrinth.iocaine.corpus, &config.targets.iocaine.join("corpus"))?;
    corpus::get_one(&config.labyrinth.iocaine.words, &config.targets.iocaine.join("corpus"))?;
    log::info!("Configuration generated. Context length is the mind-killer.");
    Ok(())
}

fn copy_if_exists(src: &str, target: &PathBuf) -> Result<(), BGError> {
    log::debug!("Copying {} to {}...", src, target.to_str().unwrap());
    create_dir_all(target)?;
    if Path::new(src).exists() {
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
