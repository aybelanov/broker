use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),
    
    #[error("Invalid token response")]
    InvalidTokenResponse,
}

#[derive(Debug, Clone)]
pub struct TokenManager {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug)]
struct Inner {
    current_token: Option<String>,
    expiry_time: Option<Instant>,
    config: Config,
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

impl TokenManager {
    pub fn new(config: Config) -> Self {
        TokenManager {
            inner: Arc::new(Mutex::new(Inner {
                current_token: None,
                expiry_time: None,
                config,
                client: reqwest::Client::new(),
            })),
        }
    }

    pub async fn start(self) -> Result<(), AuthError> {
        self.refresh_token().await?;
        let manager = self.clone();
        tokio::spawn(async move {
            manager.run_periodic_refresh().await;
        });
        Ok(())
    }

    async fn run_periodic_refresh(self) {
        loop {
            let sleep_duration = self.calculate_sleep_duration().await;
            tokio::time::sleep(sleep_duration).await;
            
            if let Err(e) = self.refresh_token().await {
                eprintln!("Failed to refresh token: {}", e);
            }
        }
    }

    async fn calculate_sleep_duration(&self) -> Duration {
        let inner = self.inner.lock().await;
        match inner.expiry_time {
            Some(expiry) => {
                let now = Instant::now();
                let five_min = Duration::from_secs(300);
                
                if expiry > now + five_min {
                    expiry - now - five_min
                } else {
                    Duration::from_secs(1)
                }
            }
            None => Duration::from_secs(30),
        }
    }

    async fn refresh_token(&self) -> Result<(), AuthError> {
        let mut inner = self.inner.lock().await;
        let token = Self::fetch_new_token(&inner.config, &inner.client).await?;
        
        inner.current_token = Some(token.access_token.clone());
        inner.expiry_time = Some(Instant::now() + Duration::from_secs(token.expires_in));
        
        Ok(())
    }

    async fn fetch_new_token(config: &Config, client: &reqwest::Client) -> Result<TokenResponse, AuthError> {
        let response = client
            .post(&config.hub_endpoint)
            .json(&serde_json::json!({
                "client_id": config.client_id,
                "secret": config.secret
            }))
            .send()
            .await?;

        let token: TokenResponse = response.json().await.map_err(|_| AuthError::InvalidTokenResponse)?;
        
        Ok(token)
    }

    pub async fn get_token(&self) -> Option<String> {
        let inner = self.inner.lock().await;
        inner.current_token.clone()
    }
}