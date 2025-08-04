use std::{fs::File, io::Write, path::{self, PathBuf}};

use tera::{Tera, Context};

use crate::{config::Config, error::BGError};

pub fn render(config: &Config, template_path: &str, target_path: PathBuf) -> Result<(), BGError> {
    let tera = Tera::new(template_path)?;
    let mut context = Context::new();
    context.insert("config", config);
    for template in tera.get_template_names() {
        let rendered = tera.render(template, &context)?;
        let full_path = target_path.join(template).with_extension("");
        let mut file = File::create(full_path)?;
        file.write_all(rendered.as_bytes())?;
    }
    Ok(())
}