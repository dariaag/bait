use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub webhook_endpoints: Vec<WebhookEndpoint>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEndpoint {
    pub id: String,
    pub url: String,
    pub token: Option<String>, // Optional authentication token
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::new();
        settings
            .merge(config::File::with_name("Config"))? // Load `Config.toml` or another format
            .merge(config::Environment::with_prefix("APP"))?; // Override with environment variables prefixed with APP_
        settings.try_into()
    }
}
