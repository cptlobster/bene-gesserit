use std::{fs::File, io::Read, path::PathBuf};

use crate::{config::Config, error::BGError};

pub mod config;
pub mod template;
pub mod error;

fn main() -> Result<(), BGError> {
    let mut cfg_file = File::open("./config.toml")?;
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;
    let config: Config = toml::from_str(&buf)?;
    template::render(&config, "./templates/**/*", PathBuf::from("./docker_include"))?;
    Ok(())
}
