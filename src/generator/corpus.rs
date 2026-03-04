//! Downloading, processing, and storing corpus files for training the Markov
//! chain.
use std::fs::{copy, create_dir_all};
use std::path::PathBuf;
use crate::{config::CorpusSrc, error::BGError};
use std::hash::{DefaultHasher, Hash, Hasher};
#[cfg(feature = "http")]
use std::fs::File;
#[cfg(feature = "http")]
use std::io::Write;
#[cfg(feature = "http")]
use regex::Regex;
#[cfg(feature = "http")]
use reqwest::{blocking::Client, StatusCode};

/// Download many corpus files. Reuses the same client to avoid recreating it
/// several times.
#[cfg(feature = "http")]
pub fn get_many(src: &Vec<CorpusSrc>, target_dir: &PathBuf) -> Result<(), BGError> {
    let client = Client::new();
    src.iter().map(|src| get(&client, src, target_dir)).collect::<Result<(), BGError>>()
}

/// Download many corpus files. Reuses the same client to avoid recreating it
/// several times.
#[cfg(not(feature = "http"))]
pub fn get_many(src: &Vec<CorpusSrc>, target_dir: &PathBuf) -> Result<(), BGError> {
    log::warn!("This version of bene-gesserit is compiled without HTTP support. Requests for corpus files that are not downloaded will fail.");
    src.iter().map(|src| get_nohttp(src, target_dir)).collect::<Result<(), BGError>>()
}

/// Download one corpus file.
#[cfg(feature = "http")]
pub fn get_one(src: &CorpusSrc, target_dir: &PathBuf) -> Result<(), BGError> {
    let client = Client::new();
    get(&client, src, target_dir)
}

/// Download one corpus file.
#[cfg(not(feature = "http"))]
pub fn get_one(src: &CorpusSrc, target_dir: &PathBuf) -> Result<(), BGError> {
    log::warn!("This version of bene-gesserit is compiled without HTTP support. Requests for corpus files that are not downloaded will fail.");
    get_nohttp(src, target_dir)
}

/// Obtain a corpus file from its source, apply custom handling functions, and
/// place it in the configuration directory.
#[cfg(feature = "http")]
fn get(client: &Client, src: &CorpusSrc, target_dir: &PathBuf) -> Result<(), BGError> {
    let target_path = gen_path(src, target_dir);
    // Check if a file already exists and skip downloading / generating if so.
    if target_path.exists() { 
        log::debug!("{:?} already exists. Skipping...", src);
        Ok(()) 
    } else {
        log::debug!("Downloading {:?} to {}", src, &target_path.to_str().unwrap());
        match src {
            // for URLs, just download and save them as-is.
            CorpusSrc::Url(path) => {
                download(client, path.as_str())
                    .and_then(|content| save(content, &target_path))
            },
            // For Project Gutenberg entries, inject the book ID into the URL,
            // remove headers/footers, and then save them.
            CorpusSrc::Gutenberg(id) => {
                download(client, format!("https://www.gutenberg.org/cache/epub/{0}/pg{0}.txt", id).as_str())
                    .and_then(simplify_gutenberg)
                    .and_then(|content| save(content, &target_path))
            },
            // For paths, copy the file as-is.
            CorpusSrc::Path(path) => {
                // If the filepath doesn't exist, make all the directories that
                // lead up to that path.
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

/// Obtain a corpus file from its source, apply custom handling functions, and
/// place it in the configuration directory. Note that this function is used
/// when the `http` feature is disabled, and will return an error if it attempts
/// to download a corpus file from the internet.
fn get_nohttp(src: &CorpusSrc, target_dir: &PathBuf) -> Result<(), BGError> {
    let target_path = gen_path(src, target_dir);
    // Check if a file already exists and skip downloading / generating if so.
    if target_path.exists() { 
        log::debug!("{:?} already exists. Skipping...", src);
        Ok(()) 
    } else {
        match src {
            // For paths, copy the file as-is.
            CorpusSrc::Path(path) => {
                // If the filepath doesn't exist, make all the directories that
                // lead up to that path.
                if let Some(p) = target_path.parent() { 
                    log::debug!("Constructing path {}...", &p.to_str().unwrap());
                    create_dir_all(p)?
                };
                copy(path, &target_path)?;
                Ok(())
            }
            _ => {
                Err(BGError::AppError("Please enable the http feature and try again.".to_string()))
            },
        }
    }
}

#[cfg(feature = "http")]
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

/// Remove header/footer from downloaded plaintext format Project Gutenberg
/// books, as this will affect training data.
#[cfg(feature = "http")]
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

/// Save the contents of the string (processed corpus material) to the target
/// filepath.
#[cfg(feature = "http")]
fn save(content: String, path: &PathBuf) -> Result<(), BGError> {
    // If the filepath doesn't exist, make all the directories that lead up to
    // that path.
    if let Some(p) = path.parent() { 
        log::debug!("Constructing path {}...", &p.to_str().unwrap());
        create_dir_all(p)?
    };
    let mut f = File::create(path)?;
    Ok(f.write_all(content.as_bytes())?)
}

/// Generate the target filepath for a corpus file.
pub fn gen_path(src: &CorpusSrc, target_path: &PathBuf) -> PathBuf {
    match src {
        CorpusSrc::Url(path) => {
            // reqwest doesn't really give me a way to guess the filename and I
            // don't feel like deconstructing URLs. so just hash the URL and
            // create a filename out of that.
            let digest = md5::compute(path.as_bytes());
            target_path.join(format!("url_{:x}.txt", digest))
        },
        CorpusSrc::Gutenberg(id) => target_path.join(format!("gutenberg_{}.txt", id)),
        CorpusSrc::Path(src) => target_path.join(PathBuf::from(src).file_name().unwrap())
    }
}