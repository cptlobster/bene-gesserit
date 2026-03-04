//! Template-based configuration generator. Templated configs are written using
//! [Tera](https://keats.github.io/tera/), a Django-like templating language.
//! 
//! ## Context Parameters
//! The following context paths are available in templates:
//! - `config`: Direct mapping to the configuration object.
//! - `corpus`: All corpus file paths, normalized to their target locations.
//! - `words`: The word list file, normalized to its target location.
use std::{fs::{create_dir_all, File}, io::Write, path::{Component, PathBuf}};
use tera::{Context, Tera};
use crate::{config::{Config, RatelimitRule}, generator::{corpus, environment::EnvConfig}, error::BGError};

/// Render all available configuration files and place them in their target
/// directories.
pub fn render(config: &Config, template_path: String) -> Result<(), BGError> {
    let env: EnvConfig = config.env.config();
    // create the instance, loading all templates in the template path
    log::debug!("Creating Tera instance...");
    let tera = Tera::new(template_path.as_str())?;
    log::debug!("Creating context...");
    let mut context = Context::new();
    // shove all config keys into the config path
    context.insert("config", config);
    context.insert("env", &env);
    // Insert the ratelimit rules as JSON strings
    context.insert("rules", &sermap_rules(config.ratelimit.as_ref().map(|c| c.rules.iter().cloned().collect()).unwrap_or_default())?);
    // Since Iocaine corpus file config has some special handling to facilitate
    // downloading, those filepaths are stored as a separate context entry.
    let base_path = env.targets.iocaine.join("corpus");
    context.insert("corpus", &config.labyrinth.iocaine.corpus.iter().map(|crp| {
        corpus::gen_path(crp, &base_path)
    }).collect::<Vec<PathBuf>>());
    // Same thing for the words file.
    context.insert("words", &corpus::gen_path(&config.labyrinth.iocaine.words, &base_path));

    // Apply the configuration to all of the templates and place them in their
    // target directories.
    for template in tera.get_template_names() {
        log::debug!("Applying template {}", &template);
        let rendered = tera.render(template, &context)?;
        let full_path = get_target_from_path(config, &PathBuf::from(template))?.parent().unwrap().join(template).with_extension("");
        log::debug!("Saving to file {}", &full_path.to_str().unwrap());
        // If the filepath doesn't exist, make all the directories that lead up
        // to that path.
        if let Some(p) = full_path.parent() { 
            log::debug!("Constructing path {}...", &p.to_str().unwrap());
            create_dir_all(p)?
        };
        let mut file = File::create(full_path)?;
        file.write_all(rendered.as_bytes())?;
    }
    Ok(())
}

/// Get the target directory for a file from the config. The path should exist
/// in the config, otherwise throw an error.
pub fn get_target_from_str(config: &Config, path: &str) -> Result<PathBuf, BGError> {
    let env: EnvConfig = config.env.config();
    // this codebase is abysmal
    match path {
        "anubis" => Ok(env.targets.anubis.clone()),
        "iocaine" => Ok(env.targets.iocaine.clone()),
        "nginx" => Ok(env.targets.nginx.clone()),
        "prometheus" => Ok(env.targets.prometheus.clone()),
        "supervisord" => Ok(env.targets.supervisord.clone()),
        cat => Err(BGError::AppError(format!("Failed to get target path: {} is not a valid category", cat)))
    }
}

/// Get the target directory from a path. This should extract the first
/// component of the path, and then pass it to [get_target_from_str].
pub fn get_target_from_path(config: &Config, path: &PathBuf) -> Result<PathBuf, BGError> {
    if let Some(Component::Normal(cat_os_str)) = path.components().next() {
        let Some(category) = cat_os_str.to_str() else {
            return Err(BGError::AppError("Failed to extract path from OSString".to_string()))
        };
        get_target_from_str(config, category)
    } else {
        Err(BGError::AppError(format!("Failed to get path from PathBuf: {:?}", path)))
    }
}

fn sermap_rules(rules: Vec<RatelimitRule>) -> Result<Vec<String>, BGError> {
    rules
        .into_iter()
        .map(|rule| serde_json::to_string(&rule))
        .collect::<Result<Vec<String>, serde_json::Error>>()
        .map_err(|e| e.into())
}