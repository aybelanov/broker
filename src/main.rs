mod app;
mod config;
mod db;
mod models;

use anyhow::Result;
use log::info;
use crate::db::sqlite;
//use crate::config::{get_config, ConfigError};

fn main()->Result<()>  {
    
    // 1. enables logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting broker application");
    
    // 2. initializes app configuration
    let cfg = config::get_config();
    info!("App configuration has been read successfully.");
     
    // 3. initializes SQLite data base
    sqlite::init_db().unwrap();
    info!("Data base has been initialized successfully.");

    if !cfg.enabled {
        info!("Application is disabled, exiting.");
        return Ok(());
    }


    Ok(())
    // println!("Application started with config:");
    // println!("Enabled: {}", config.enabled);
    // println!("System Name: {}", config.system_name);
    // println!("Client ID: {}", config.client_id);
    // println!("Hub Endpoint: {}", config.hub_endpoint);
    // println!("Secret: {}", "*".repeat(config.secret.len()));
}