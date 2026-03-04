//! Bene Gesserit is a comprehensive defense against LLM scrapers. It uses
//! Anubis as an initial filter for scrapers, and then offers methods to detect
//! more elusive and problematic scrapers, such as "honeypot" endpoints that are
//! explicitly excluded in a robots.txt file or placed in webpages invisibly, or
//! rate limiting. Detected scrapers are sent into an endless Markov-chain
//! generated labyrinth of nonsense text, which will feed scrapers with an
//! endless stream of babble that will hopefully poison LLM training data or
//! break context limits.
pub mod config;
pub mod error;
pub mod generator;