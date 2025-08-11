//! The config file representation for all of Bene Gesserit. This is the single
//! source of truth for all configuration parameters, and should be the only
//! place that you make configuration changes for the application. The generator
//! script will handle creating other configuration files.
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use crate::environment::EnvConfOpts;

/// The config object that contains all subconfiguration parameters.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The final target endpoint.
    pub target: String,
    /// Configuration for the specific environment.
    #[serde(rename = "environment")]
    pub env: EnvConfOpts,
    /// Configuration for honeypots. If not specified, the honeypot generator
    /// will not be used.
    #[serde(default)]
    pub honeypot: Option<HoneypotConfig>,
    /// Configuration for ratelimiting. If not specified, ratelimits will not be
    /// used.
    #[serde(default)]
    pub ratelimit: Option<RatelimitConfig>,
    /// Configuration parameters for the labyrinth, an endless tree of Markov
    /// chain generated nonsense. These configurations are used for Iocaine.
    pub labyrinth: LabyrinthConfig,
    /// Configuration for Prometheus metrics.
    pub metrics: MetricsConfig
}

/// This section configures "honeypot" endpoints; Any endpoints that match these
/// patterns will automatically be passed into the labyrinth.
#[derive(Serialize, Deserialize, Debug)]
pub struct HoneypotConfig {
    /// A set of regex patterns for paths to match.
    pub endpoints: Vec<String>,
    #[serde(default)]
    pub override_robots: bool
}

/// This section configures a set of rate limiting rules.
#[derive(Serialize, Deserialize, Debug)]
pub struct RatelimitConfig {
    pub rules: Vec<RatelimitRule>
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
    /// Configuration parameters passed through to Iocaine.
    #[serde(flatten)]
    pub iocaine: IocaineMixins
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "rule", rename_all = "snake_case")]
pub enum RatelimitRule {
    /// Trigger a ratelimit if users make {amount} requests in {seconds}
    /// seconds.
    #[serde(alias = "any")]
    AnyRequests {
        amount: u32,
        seconds: u32
    },
    /// Trigger a ratelimit if a user hits {unique} endpoints in {seconds} 
    /// seconds. If total is specified, then a user must make a minimum of
    /// {total} requests before the unique threshold is checked. 
    #[serde(alias = "unique")]
    UniqueRequests {
        #[serde(default)]
        total: Option<u32>,
        unique: u32,
        seconds: u32
    }
}

/// Configuration for Prometheus metrics.
#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsConfig {
    pub enabled: bool
}