use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use crate::domain::models::{ChatRequest, ApiResponse};
use crate::application::chat_service::ChatService;
use crate::infrastructure::openrouter_client::OpenRouterClient;
use crate::config::Config;

pub async fn chat_handler(
    State(config): State<Config>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    // Validate request
    if request.messages.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Messages array cannot be empty".to_string(),
        ));
    }

    // Validate model
    if request.model.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Model must be specified".to_string(),
        ));
    }

    let client = OpenRouterClient::new(config);
    let service = ChatService::new(client);

    service
        .process_chat(request)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
} 