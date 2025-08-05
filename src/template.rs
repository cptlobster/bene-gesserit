use std::{fs::{create_dir_all, File}, io::Write, path::{Component, Path, PathBuf}};

use tera::{Context, Tera};

use crate::{config::Config, corpus, error::BGError};

pub fn render(config: &Config, template_path: &str) -> Result<(), BGError> {
    log::debug!("Creating Tera instance...");
    let tera = Tera::new(template_path)?;
    log::debug!("Creating context...");
    let mut context = Context::new();
    context.insert("config", config);

    let base_path = config.targets.iocaine.join("corpus");
    context.insert("corpus", &config.labyrinth.iocaine.corpus.iter().map(|crp| {
        corpus::gen_path(crp, &base_path)
    }).collect::<Vec<PathBuf>>());
    context.insert("words", &corpus::gen_path(&config.labyrinth.iocaine.words, &base_path));

    for template in tera.get_template_names() {
        log::debug!("Applying template {}", &template);
        let rendered = tera.render(template, &context)?;
        let full_path = get_target_from_path(config, &PathBuf::from(template))?.parent().unwrap().join(template).with_extension("");
        log::debug!("Saving to file {}", &full_path.to_str().unwrap());
        if let Some(p) = full_path.parent() { 
            log::debug!("Constructing path {}...", &p.to_str().unwrap());
            create_dir_all(p)?
        };
        let mut file = File::create(full_path)?;
        file.write_all(rendered.as_bytes())?;
    }
    Ok(())
}

pub fn get_target_from_str(config: &Config, path: &str) -> Result<PathBuf, BGError> {
    match path {
        "anubis" => Ok(config.targets.anubis.clone()),
        "iocaine" => Ok(config.targets.iocaine.clone()),
        "nginx" => Ok(config.targets.nginx.clone()),
        "prometheus" => Ok(config.targets.prometheus.clone()),
        "supervisord" => Ok(config.targets.supervisord.clone()),
        cat => Err(BGError::AppError(format!("Failed to get target path: {} is not a valid category", cat)))
    }
}

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