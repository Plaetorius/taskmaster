use crate::models::Config;
use std::fs::File;
use std::io::{self, Read};
use toml;
use thiserror::Error; // TODO might be illegal

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
    
    #[error("TOML Deserialization Error: {0}")]
    Toml(#[from] toml::de::Error),
}

/// TODO write me
pub fn parse_config(file_path: &str) -> Result<Config, ConfigError> {
	let mut file = File::open(file_path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	let config: Config = toml::from_str(&contents)?;
	Ok(config)
}