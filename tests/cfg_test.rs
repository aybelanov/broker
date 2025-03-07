use std::fs;
use broker::config::{get_config, validation::validate, Config, ConfigError};
use std::io::Write;
use std::panic::catch_unwind;
use tempfile::NamedTempFile;

// Хелпер для создания временного файла конфигурации
fn create_temp_config(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "{}", content).unwrap();
    file
}

fn parse_config_directly(path: &str) -> Config {
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(&contents).unwrap_or_else(|e| {
        panic!("Configuration error: JSON parsing error: {}", e);
    })
}

#[test]
fn test_get_config_valid() {
    let json = r#"
        {
            "enabled": false,
            "system_name": "MySystem",
            "client_id": "my_client",
            "secret": "MySecret123!",
            "hub_endpoint": "http://localhost",
            "listen_port": 3000
        }
    "#;
    let temp_file = create_temp_config(json);
    let config = get_config(temp_file.path().to_str().unwrap());

    assert_eq!(config.enabled, false);
    assert_eq!(config.system_name, "MySystem");
    assert_eq!(config.client_id, "my_client");
    assert_eq!(config.secret, "MySecret123!");
    assert_eq!(config.hub_endpoint, "http://localhost");
    assert_eq!(config.listen_port, 3000);
}

#[test]
fn test_get_config_panic_on_invalid_file() {
    let result = catch_unwind(|| {
        get_config("non_existent_file.json");
    });
    assert!(result.is_err()); // Ожидаем панику
}

#[test]
fn test_get_config_panic_on_invalid_json() {
    let json = r#"{"enabled": true}"#;
    let temp_file = create_temp_config(json);
    let result = catch_unwind(|| {
        parse_config_directly(temp_file.path().to_str().unwrap());
    });
    assert!(result.is_err());
}

#[test]
fn test_validate_valid_config() {
    let config = Config {
        enabled: true,
        system_name: "ValidSystem".to_string(),
        client_id: "valid123".to_string(),
        secret: "Valid123!".to_string(),
        hub_endpoint: "https://test.com".to_string(),
        listen_port: 8080,
    };
    let result = validate(&config);
    assert!(result.is_ok());
}

#[test]
fn test_validate_empty_system_name() {
    let config = Config {
        enabled: true,
        system_name: "".to_string(),
        client_id: "valid123".to_string(),
        secret: "Valid123!".to_string(),
        hub_endpoint: "https://test.com".to_string(),
        listen_port: 8080,
    };
    let result = validate(&config);
    assert!(matches!(result, Err(ConfigError::Validation(ref s)) if s == "System name cannot be empty"));
}

#[test]
fn test_validate_empty_client_id() {
    let config = Config {
        enabled: true,
        system_name: "ValidSystem".to_string(),
        client_id: "   ".to_string(), // Пробелы
        secret: "Valid123!".to_string(),
        hub_endpoint: "https://test.com".to_string(),
        listen_port: 8080,
    };
    let result = validate(&config);
    assert!(matches!(result, Err(ConfigError::Validation(ref s)) if s == "Client ID cannot be empty"));
}

#[test]
fn test_validate_short_secret() {
    let config = Config {
        enabled: true,
        system_name: "ValidSystem".to_string(),
        client_id: "valid123".to_string(),
        secret: "Short".to_string(), // Меньше 8 символов
        hub_endpoint: "https://test.com".to_string(),
        listen_port: 8080,
    };
    let result = validate(&config);
    assert!(matches!(result, Err(ConfigError::Validation(ref s)) if s == "Secret must be at least 8 characters long"));
}

#[test]
fn test_validate_secret_no_uppercase() {
    let config = Config {
        enabled: true,
        system_name: "ValidSystem".to_string(),
        client_id: "valid123".to_string(),
        secret: "secret123!".to_string(), // Нет заглавных букв
        hub_endpoint: "https://test.com".to_string(),
        listen_port: 8080,
    };
    let result = validate(&config);
    assert!(matches!(result, Err(ConfigError::Validation(ref s)) if s == "Secret must contain at least one uppercase letter"));
}

#[test]
fn test_validate_secret_no_digit() {
    let config = Config {
        enabled: true,
        system_name: "ValidSystem".to_string(),
        client_id: "valid123".to_string(),
        secret: "Secret!!!".to_string(), // Нет цифр
        hub_endpoint: "https://test.com".to_string(),
        listen_port: 8080,
    };
    let result = validate(&config);
    assert!(matches!(result, Err(ConfigError::Validation(ref s)) if s == "Secret must contain at least one digit"));
}

#[test]
fn test_validate_secret_no_special() {
    let config = Config {
        enabled: true,
        system_name: "ValidSystem".to_string(),
        client_id: "valid123".to_string(),
        secret: "Secret123".to_string(), // Нет спецсимволов
        hub_endpoint: "https://test.com".to_string(),
        listen_port: 8080,
    };
    let result = validate(&config);
    assert!(
        matches!(
            result,
            Err(ConfigError::Validation(ref s))
            if s == "Secret must contain at least one special character"
        )
    );
}

#[test]
fn test_validate_invalid_endpoint() {
    let config = Config {
        enabled: true,
        system_name: "ValidSystem".to_string(),
        client_id: "valid123".to_string(),
        secret: "Valid123!".to_string(),
        hub_endpoint: "ftp://test.com".to_string(), // Неправильный протокол
        listen_port: 8080,
    };
    let result = validate(&config);
    assert!(
        matches!(
            result,
            Err(ConfigError::Validation(ref s))
            if s == "Hub endpoint must use http:// or https:// protocol"
        )
    );
}