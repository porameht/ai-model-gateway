use reqwest::Client;
use crate::config::Config;
use crate::domain::models::{ ChatRequest, ChatResponse, ApiResponse };

pub struct OpenRouterClient {
    client: Client,
    config: Config,
}

impl OpenRouterClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn send_chat_request(&self, request: ChatRequest) -> Result<ApiResponse, String> {
        let response = self.client
            .post(&self.config.openrouter_api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.openrouter_api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let chat_response = response.json::<ChatResponse>()
            .await
            .map_err(|e| e.to_string())?;

        // Extract the first choice's message
        let message = chat_response.choices
            .first()
            .ok_or_else(|| "No response choices available".to_string())?
            .message.clone();

        Ok(ApiResponse { message })
    }
} 