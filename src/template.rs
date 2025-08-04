use std::{fs::File, io::Write, path::PathBuf};

use tera::{Context, Tera};

use crate::{config::Config, corpus, error::BGError};

pub fn render(config: &Config, template_path: &str, target_path: &PathBuf) -> Result<(), BGError> {
    log::debug!("Creating Tera instance...");
    let tera = Tera::new(template_path)?;
    log::debug!("Creating context...");
    let mut context = Context::new();
    context.insert("config", config);

    let base_path = PathBuf::from(&config.labyrinth.iocaine.base_path);
    context.insert("corpus", &config.labyrinth.iocaine.corpus.iter().map(|crp| {
        corpus::gen_path(crp, &base_path)
    }).collect::<Vec<PathBuf>>());
    context.insert("words", &corpus::gen_path(&config.labyrinth.iocaine.words, &base_path));
    for template in tera.get_template_names() {
        log::debug!("Applying template {}", &template);
        let rendered = tera.render(template, &context)?;
        let full_path = target_path.join(template).with_extension("");
        log::debug!("Saving to file {}", &full_path.to_str().unwrap());
        let mut file = File::create(full_path)?;
        file.write_all(rendered.as_bytes())?;
    }
    Ok(())
}