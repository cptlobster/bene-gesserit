use std::{fs::File, io::Read};

use crate::{config::Config, error::BGError};

pub mod config;
pub mod template;
pub mod error;
pub mod corpus;

fn main() -> Result<(), BGError> {
    log::info!("Loading configuration file...");
    let mut cfg_file = File::open("./config.toml")?;
    log::info!("Parsing configuration file...");
    let mut buf = String::new();
    cfg_file.read_to_string(&mut buf)?;
    let config: Config = toml::from_str(&buf)?;
    log::info!("Rendering templates...");
    template::render(&config, "./templates/**/*", &config.target)?;
    log::info!("Downloading corpus files...");
    corpus::get_many(&config.labyrinth.iocaine.corpus, &config.target.join("iocaine/corpus"))?;
    corpus::get_one(&config.labyrinth.iocaine.words, &config.target.join("iocaine/corpus"))?;
    log::info!("Configuration generated. As it was written.");
    Ok(())
}
