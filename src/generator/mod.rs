//! This library is the configuration utility that orchestrates all other
//! services. It will populate configuration directories with static files and
//! generate configuration files from templates using parameters passed in the
//! initial configuration file. It will also download corpus files for feeding
//! the labyrinth if they are not present.
pub mod template;
pub mod corpus;
pub mod environment;
