use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub targets: TargetPaths,
    pub endpoints: EndpointConfig,
    #[serde(default)]
    pub honeypot: Option<HoneypotConfig>,
    #[serde(default)]
    pub ratelimit: Option<RatelimitConfig>,
    pub labyrinth: LabyrinthConfig,
    pub metrics: MetricsConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetPaths {
    pub nginx: PathBuf,
    pub anubis: PathBuf,
    pub iocaine: PathBuf,
    pub prometheus: PathBuf,
    pub supervisord: PathBuf
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndpointConfig {
    pub target: String,
    pub iocaine: String,
    pub anubis: String,
    pub internal: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoneypotConfig {
    pub endpoints: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RatelimitConfig {
    pub rules: Vec<RatelimitRule>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabyrinthConfig {
    pub banish_threshold: u32,
    #[serde(flatten)]
    pub iocaine: IocaineMixins
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IocaineMixins {
    pub corpus: Vec<CorpusSrc>,
    pub words: CorpusSrc
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum CorpusSrc {
    Url(String),
    Gutenberg(u32),
    #[serde(untagged)]
    Path(PathBuf)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RatelimitRule {
    AnyRequests {
        amount: u32,
        seconds: u32
    },
    UniqueRequests {
        total: u32,
        unique: u32,
        seconds: u32
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsConfig {
    pub enabled: bool
}