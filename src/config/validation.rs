use super::{Config, ConfigError};

pub fn validate(config: &Config) -> Result<(), ConfigError> {
    validate_system_name(&config.system_name)?;
    validate_client_id(&config.client_id)?;
    validate_secret(&config.secret)?;
    validate_endpoint(&config.hub_endpoint)?;
    Ok(())
}

fn validate_system_name(value: &str) -> Result<(), ConfigError> {
    if value.trim().is_empty() {
        return Err(ConfigError::Validation("System name cannot be empty".into()));
    }
    Ok(())
}

fn validate_client_id(value: &str) -> Result<(), ConfigError> {
    if value.trim().is_empty() {
        return Err(ConfigError::Validation("Client ID cannot be empty".into()));
    }
    Ok(())
}

fn validate_secret(value: &str) -> Result<(), ConfigError> {
    if value.len() < 8 {
        return Err(ConfigError::Validation(
            "Secret must be at least 8 characters long".into(),
        ));
    }
    if !value.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ConfigError::Validation(
            "Secret must contain at least one uppercase letter".into(),
        ));
    }
    if !value.chars().any(|c| c.is_ascii_digit()) {
        return Err(ConfigError::Validation(
            "Secret must contain at least one digit".into(),
        ));
    }
    if !value.chars().any(|c| c.is_ascii_punctuation()) {
        return Err(ConfigError::Validation(
            "Secret must contain at least one special character".into(),
        ));
    }
    Ok(())
}

fn validate_endpoint(value: &str) -> Result<(), ConfigError> {
    if !value.starts_with("http://") && !value.starts_with("https://") {
        return Err(ConfigError::Validation(
            "Hub endpoint must use http:// or https:// protocol".into(),
        ));
    }
    Ok(())
}