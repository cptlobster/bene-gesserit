mod supervisord;

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::config::Config;
use crate::environment::EnvConfig;
use crate::error::BGError;
use crate::{corpus, template};

/// Generate the configuration for underlying services.
pub fn generate(config: &Config, static_dir: String, template_dir: String) -> Result<(), BGError> {
    let env: EnvConfig = config.env.config();
    // Copy static files from static directory into target directory.
    log::info!("Cloning static files...");
    copy_if_exists(format!("{}/anubis/.", static_dir), &env.targets.anubis)?;
    copy_if_exists(format!("{}/iocaine/.", static_dir), &env.targets.iocaine)?;
    copy_if_exists(format!("{}/nginx/.", static_dir), &env.targets.nginx)?;
    copy_if_exists(format!("{}/prometheus/.", static_dir), &env.targets.prometheus)?;
    copy_if_exists(format!("{}/supervisord/.", static_dir), &env.targets.supervisord)?;
    // Render templated configuration files and place them in the target
    // directory.
    log::info!("Rendering templates...");
    template::render(&config, format!("{}/**/*.tera", template_dir))?;
    // Download corpus files and place them in the target directory.
    log::info!("Downloading corpus files...");
    corpus::get_many(&config.labyrinth.iocaine.corpus, &env.targets.iocaine.join("corpus"))?;
    corpus::get_one(&config.labyrinth.iocaine.words, &env.targets.iocaine.join("corpus"))?;
    // Everything finished correctly, return ok
    log::info!("Configuration generated. Context length is the mind-killer.");
    Ok(())
}

/// Start the Bene Gesserit stack.
pub fn start(config: &Config) -> Result<(), BGError> {
    let mut supervisord = Command::new("supervisord");

    Ok(())
}

/// Reload the Bene Gesserit stack.
pub fn reload(config: &Config) -> Result<(), BGError> {
    Ok(())
}

/// Stop the Bene Gesserit stack.
pub fn stop(config: &Config) -> Result<(), BGError> {
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