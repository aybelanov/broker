use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::panic;
use thiserror::Error;

pub mod validation;

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
    pub listen_port: u16
}

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn get_config(cfg_file_path:&str) -> &'static Config {
    CONFIG.get_or_init(|| {
        load_config(cfg_file_path).unwrap_or_else(|e| {
            panic!("Configuration error: {}", e);
        })
    })
}

fn load_config(cfg_file_path:&str) -> Result<Config, ConfigError> {
    let config_data = std::fs::read_to_string(cfg_file_path)?;
    let config: Config = serde_json::from_str(&config_data)?;
    validation::validate(&config)?;
    Ok(config)
}