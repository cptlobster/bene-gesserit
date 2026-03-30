//! The config file representation for all of Bene Gesserit. This is the single
//! source of truth for all configuration parameters, and should be the only
//! place that you make configuration changes for the application. The generator
//! script will handle creating other configuration files.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use crate::generator::environment::EnvConfOpts;

/// The config object that contains all subconfiguration parameters.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Whether extra debugging headers are enabled.
    #[serde(default)]
    pub debug: bool,
    /// The final target endpoint.
    pub target: String,
    /// Configuration for the specific environment.
    #[serde(rename = "environment")]
    pub env: EnvConfOpts,
    /// Configuration for Anubis.
    #[serde(default)]
    pub anubis: AnubisOpts,
    /// Configuration for honeypots. If not specified, the honeypot generator
    /// will not be used.
    #[serde(default)]
    pub honeypot: Option<HoneypotConfig>,
    /// Configuration for ratelimiting. If not specified, ratelimits will not be
    /// used.
    #[serde(default)]
    pub ratelimit: Option<RatelimitConfig>,
    /// Configuration for IP address banning.
    #[serde(default)]
    pub ipban: IpBanConfig,
    /// Configuration parameters for the labyrinth, an endless tree of Markov
    /// chain generated nonsense. These configurations are used for Iocaine, or for NGINX rules
    /// related to Iocaine.
    pub labyrinth: LabyrinthConfig,
    /// Configuration for Prometheus metrics.
    #[serde(default)]
    pub metrics: MetricsConfig
}

/// This section configures Anubis settings.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AnubisOpts {
    #[serde(default)]
    predef_rules: AnubisPredefRules
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AnubisPredefRules {
    #[serde(default)]
    block_cf_bots: bool
}

/// This section configures "honeypot" endpoints; Any endpoints that match these
/// patterns will automatically be passed into the labyrinth.
#[derive(Serialize, Deserialize, Debug)]
pub struct HoneypotConfig {
    /// A set of regex patterns for paths to match.
    pub endpoints: Vec<String>,
    #[serde(default)]
    pub robots: RobotsConfig
}

/// This section defines the generation of the robots.txt file.
/// At the moment it is very manual; due to how the patterns work, we need to
/// either figure out how to automatically generate paths to use in the robots
/// file or we need to write them ourselves. We'll handle the former in a later
/// update.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RobotsConfig {
    /// Whether to use the service defined robots.txt file. If disabled, passes through to the
    /// backend's robots.txt.
    #[serde(default)]
    pub generate: bool,
    /// Define the contents of the robots.txt file. This consists of a dictionary where the keys are
    /// user agent strings and the values are lists of URI paths, like so:
    /// ```toml
    /// [honeypot.robots.contents]
    /// "*" = ["/sicily", "/asia"] # disallow any user agent
    /// "GoogleBot" = ["/sicily", "/asia"] # disallow GoogleBot specifically
    /// ```
    /// This example generates the following robots.txt file:
    /// ```text
    /// User-Agent: *
    /// Disallow: /sicily
    /// Disallow: /asia
    ///
    /// User-Agent: GoogleBot
    /// Disallow: /sicily
    /// Disallow: /asia
    /// ```
    #[serde(default)]
    pub contents: HashMap<String, Vec<String>>
}

/// This section configures a set of rate limiting rules.
#[derive(Serialize, Deserialize, Debug)]
pub struct RatelimitConfig {
    #[serde(default)]
    pub rules: Vec<RatelimitRule>
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct IpBanConfig {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    limits: IpBanLimits
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct IpBanLimits {
    #[serde(default = "default_client_ipban")]
    client: u32,
    #[serde(default = "default_region_ipban")]
    region: u32
}

/// Configuration parameters for the labyrinth, an endless tree of Markov chain
/// generated nonsense. These configurations are used for Iocaine.
#[derive(Serialize, Deserialize, Debug)]
pub struct LabyrinthConfig {
    /// After n amount of violations, clients will be permanently sent into the
    /// labyrinth on all future requests (even to legitimate endpoints). A
    /// "client" is determined by the cookie set by Anubis on their initial
    /// connection. Setting this to 0 disables this functionality, and clients
    /// will not be permanently sent into the labyrinth (just when they trigger
    /// violations).
    pub banish_threshold: u32,
    /// After a violation, the application should wait this many seconds before recording another
    /// violation, for banish purposes. Setting to 0 disables this.
    #[serde(default = "default_grace_seconds")]
    pub violation_delay: u32,
    /// If enabled, limits the speed of the iocaine pipe to 64 bits per second.
    #[serde(default)]
    pub slowmode: SlowModeConfig,
    /// Configuration parameters passed through to Iocaine.
    #[serde(flatten)]
    pub iocaine: IocaineMixins
}

/// Configuration for labyrinth slow mode. If enabled, limits the speed of labyrinth traffic.
/// Meant to slow down traffic for bots and scrapers in order to waste their time.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SlowModeConfig {
    /// Enable or disable the rate limit for the labyrinth.
    pub enable: bool,
    /// Apply the rate limit after this amount of bytes has been transferred.
    #[serde(alias="limit_rate_after", default="default_limit_rate_after")]
    pub after: Option<String>,
    /// After the initial data is transferred, limit rate at this speed.
    #[serde(alias="limit_rate", default="default_limit_rate")]
    pub rate: Option<String>
}

/// Configuration for Iocaine.
#[derive(Serialize, Deserialize, Debug)]
pub struct IocaineMixins {
    /// The file(s) used to train the Markov chain generator. These should be
    /// long text files that contain a lot of words. You can select files on
    /// your computer or arbitrary URLs, or use one of the pre-configured text
    /// file services (such as Project Gutenberg). See [CorpusSrc] for more
    /// information on available services.
    pub corpus: Vec<CorpusSrc>,
    /// A word list file. This should be a file containing a list of words,
    /// separated by newlines. These will be used for generating URL paths to
    /// link to.
    pub words: CorpusSrc
}

/// The various sources for a document used in the Markov chain training. These
/// can be files hosted locally or on the Internet, and will be downloaded on
/// launch of the application. If using Docker, these files can be persisted by
/// volume mounting the directory, and subsequent restarts will use the already
/// downloaded copies.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum CorpusSrc {
    /// Download a file from a URL directly. This will utilize the file as-is,
    /// make sure that you provide a plain text file.
    Url(String),
    /// Download a book from Project Gutenberg. This will download a plaintext
    /// version of the book with the ID you provide, and strip the header and
    /// footer from the file when training the Markov generator. These files
    /// should not be redistributed in this form.
    Gutenberg(u32),
    /// Use a file from a path on your filesystem.
    #[serde(untagged)]
    Path(PathBuf)
}

/// Rules for how rate limiting should be triggered.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "rule", rename_all = "snake_case")]
pub enum RatelimitRule {
    /// Trigger a ratelimit if users make {amount} requests in {seconds}
    /// seconds.
    #[serde(alias = "any")]
    AnyRequests {
        amount: u32,
        seconds: u32,
        #[serde(default)]
        include: Vec<String>,
        #[serde(default)]
        exclude: Vec<String>
    },
    /// Trigger a ratelimit if a user hits {unique} endpoints in {seconds} 
    /// seconds. If total is specified, then a user must make a minimum of
    /// {total} requests before the unique threshold is checked. 
    #[serde(alias = "unique")]
    UniqueRequests {
        #[serde(default)]
        total: Option<u32>,
        unique: u32,
        seconds: u32,
        #[serde(default)]
        include: Vec<String>,
        #[serde(default)]
        exclude: Vec<String>
    }
}

/// Configuration for Prometheus metrics.
#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsConfig {
    pub enabled: bool
}

impl Default for MetricsConfig {
    fn default() -> Self {
        MetricsConfig { enabled: false }
    }
}

/// The default grace period is 5 seconds.
fn default_grace_seconds() -> u32 { 5 }
fn default_true() -> bool { true }
fn default_limit_rate_after() -> Option<String> { Some("256".to_string()) }
fn default_limit_rate() -> Option<String> { Some("128".to_string()) }
fn default_client_ipban() -> u32 { 10 }
fn default_region_ipban() -> u32 { 100 }