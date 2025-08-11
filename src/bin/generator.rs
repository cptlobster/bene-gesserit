use std::{fs::{create_dir_all, File}, io::Read, path::{Path, PathBuf}};
use bene_gesserit::{config::Config, environment::EnvConfig, error::BGError, template, corpus};
use env_logger::Env;
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
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Load and parse the configuration file.
    log::debug!("Loading configuration file...");
    let mut cfg_file = File::open(cli.cfg_file)?;
    log::info!("Parsing configuration...");
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;
    let config: Config = toml::from_str(&buf)?;
    let env: EnvConfig = config.env.config();
    log::debug!("Environment: {:?}", env);
    // Copy static files from static directory into target directory.
    log::info!("Cloning static files...");
    copy_if_exists(format!("{}/anubis/.", cli.static_dir), &env.targets.anubis)?;
    copy_if_exists(format!("{}/iocaine/.", cli.static_dir), &env.targets.iocaine)?;
    copy_if_exists(format!("{}/nginx/.", cli.static_dir), &env.targets.nginx)?;
    copy_if_exists(format!("{}/prometheus/.", cli.static_dir), &env.targets.prometheus)?;
    copy_if_exists(format!("{}/supervisord/.", cli.static_dir), &env.targets.supervisord)?;
    // Render templated configuration files and place them in the target
    // directory.
    log::info!("Rendering templates...");
    template::render(&config, format!("{}/**/*.tera", cli.template_dir))?;
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
fn copy_if_exists(src: String, target: &PathBuf) -> Result<(), BGError> {
    log::debug!("Copying {} to {}...", src, target.to_str().unwrap());
    create_dir_all(target)?;
    if Path::new(&src).exists() {
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
