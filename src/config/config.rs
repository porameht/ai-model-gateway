use std::env;

#[derive(Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub openrouter_api_key: String,
    pub openrouter_api_url: String,
}

impl Config {
    pub fn new() -> Result<Self, String> {
        Ok(Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "80".to_string())
                .parse()
                .map_err(|_| "Invalid SERVER_PORT")?,
            openrouter_api_key: env::var("OPENROUTER_API_KEY")
                .map_err(|_| "OPENROUTER_API_KEY must be set")?,
            openrouter_api_url: env::var("OPENROUTER_API_URL")
                .unwrap_or_else(|_| "https://openrouter.ai/api/v1/chat/completions".to_string()),
        })
    }
}
