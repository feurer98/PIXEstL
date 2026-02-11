//! CLI argument parsing

use crate::config::Config;
use crate::error::Result;

// TODO: Implement full CLI with clap
// This is a stub for now

pub fn parse_args() -> Result<Config> {
    Ok(Config::default())
}
