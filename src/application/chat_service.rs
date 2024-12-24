use crate::domain::models::{ChatRequest, ApiResponse};
use crate::infrastructure::openrouter_client::OpenRouterClient;

const SUPPORTED_MODELS: [&str; 5] = [
    "openai/gpt-3.5-turbo",
    "openai/gpt-4",
    "anthropic/claude-3-5-sonnet",
    "meta-llama/llama-3.2-3b-instruct:free",
    "meta-llama/llama-3.2-1b-instruct:free",
];

pub struct ChatService {
    client: OpenRouterClient,
}

impl ChatService {
    pub fn new(client: OpenRouterClient) -> Self {
        Self { client }
    }

    pub async fn process_chat(&self, request: ChatRequest) -> Result<ApiResponse, String> {
        // Validate model
        if !SUPPORTED_MODELS.contains(&request.model.as_str()) {
            return Err(format!(
                "Unsupported model: {}. Supported models are: {}",
                request.model,
                SUPPORTED_MODELS.join(", ")
            ));
        }

        self.client.send_chat_request(request).await
    }
} 