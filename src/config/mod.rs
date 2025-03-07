use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::panic;
use thiserror::Error;

mod validation;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub enabled: bool,
    pub system_name: String,
    pub client_id: String,
    pub secret: String,
    pub hub_endpoint: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        load_config().unwrap_or_else(|e| {
            panic!("Configuration error: {}", e);
        })
    })
}

fn load_config() -> Result<Config, ConfigError> {
    let config_path = "config.json";
    let config_data = std::fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_data)?;
    validation::validate(&config)?;
    Ok(config)
}