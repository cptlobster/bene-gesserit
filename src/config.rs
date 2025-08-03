use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    endpoints: EndpointConfig,
    honeypot: Option<HoneypotConfig>,
    ratelimit: Option<RatelimitConfig>,
    labyrinth: LabyrinthConfig
}

#[derive(Serialize, Deserialize)]
pub struct EndpointConfig {
    target: String,
    iocaine: String,
    anubis: String,
    internal: String
}

#[derive(Serialize, Deserialize)]
pub struct HoneypotConfig {
    endpoints: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RatelimitConfig {
    rules: Vec<RatelimitRule>
}

#[derive(Serialize, Deserialize)]
pub struct LabyrinthConfig {
    banish_threshold: u32,
    #[serde(flatten)]
    iocaine: IocaineMixins
}

#[derive(Serialize, Deserialize)]
pub struct IocaineMixins {
    corpus: Vec<String>,
    words: String
}

#[derive(Serialize, Deserialize)]
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