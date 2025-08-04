use std::fs::{copy, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use crate::{config::CorpusSrc, error::BGError};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn get_many(src: &Vec<CorpusSrc>, target_dir: &PathBuf) -> Result<(), BGError> {
    let client = Client::new();
    src.iter().map(|src| get(&client, src, target_dir)).collect::<Result<(), BGError>>()
}

pub fn get_one(src: &CorpusSrc, target_dir: &PathBuf) -> Result<(), BGError> {
    let client = Client::new();
    get(&client, src, target_dir)
}

/// Obtain a corpus file.
fn get(client: &Client, src: &CorpusSrc, target_dir: &PathBuf) -> Result<(), BGError> {
    let target_path = gen_path(src, target_dir);
    log::debug!("Downloading {:?} to {}", src, &target_path.to_str().unwrap());
    if target_path.exists() { 
        log::debug!("{:?} already exists. Skipping...", src);
        Ok(()) 
    } else {
        match src {
            CorpusSrc::Url(path) => download(client, path.as_str())
                .and_then(|content| save(content, &target_path)),
            CorpusSrc::Gutenberg(id) => download(client, format!("https://www.gutenberg.org/cache/epub/{0}/pg{0}.txt", id).as_str())
                .and_then(simplify_gutenberg)
                .and_then(|content| save(content, &target_path)),
            CorpusSrc::Path(path) => {
                if let Some(p) = target_path.parent() { 
                    log::debug!("Constructing path {}...", &p.to_str().unwrap());
                    create_dir_all(p)?
                };
                copy(path, &target_path)?;
                Ok(())
            }
        }
    }
}

/// Read the file from the source location.
fn download(client: &Client, src: &str) -> Result<String, BGError> {
    let result = client.get(src).send()?;
    match result.status() {
        StatusCode::OK => {
            let text = result.text()?;
            Ok(text)
        },
        code => {
            log::error!("Request to {} returned status code {}", src, code);
            Err(BGError::AppError(format!("Request to {} returned status code {}", src, code)))
        }
    }
}

/// Remove meta content from downloaded plaintext format Project Gutenberg books for training data.
fn simplify_gutenberg(content: String) -> Result<String, BGError> {
    let start = Regex::new(r"^\*{3} START OF THE PROJECT GUTENBERG EBOOK")?;
    let end = Regex::new(r"^\*{3} END OF THE PROJECT GUTENBERG EBOOK")?;
    Ok(content.lines()
        .skip_while(|line| !start.is_match(*line))
        .skip(1)
        .take_while(|line| !end.is_match(*line))
        .collect::<Vec<&str>>()
        .join("\n"))
}

fn save(content: String, path: &PathBuf) -> Result<(), BGError> {
    if let Some(p) = path.parent() { 
        log::debug!("Constructing path {}...", &p.to_str().unwrap());
        create_dir_all(p)?
    };
    let mut f = File::create(path)?;
    Ok(f.write_all(content.as_bytes())?)
}

pub fn gen_path(src: &CorpusSrc, target_path: &PathBuf) -> PathBuf {
    match src {
        CorpusSrc::Url(path) => {
            let mut hasher = DefaultHasher::new();
            path.hash(&mut hasher);
            target_path.join(format!("url_{:x}.txt", hasher.finish()))
        },
        CorpusSrc::Gutenberg(id) => target_path.join(format!("gutenberg_{}.txt", id)),
        CorpusSrc::Path(src) => target_path.join(PathBuf::from(src).file_name().unwrap())
    }
}