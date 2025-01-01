mod config;
mod domain;
mod infrastructure;
mod application;
mod presentation;

use axum::{
    routing::{post, get},
    Router,
};
use dotenv::dotenv;
use tower_http::cors::{CorsLayer, Any};
use crate::config::Config;

// Basic health check handler
async fn health_check() -> &'static str {
    "AI API Server is running"
}

#[tokio::main]
async fn main() {
    // Initialize environment variables
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::new().expect("Failed to load configuration");
    let host = config.server_host.clone();
    let port = config.server_port;

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes and CORS
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/chat", post(presentation::handlers::chat_handler))
        .route("/api/prompt", post(presentation::handlers::prompt_handler))
        .layer(cors)
        .with_state(config);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .expect("Failed to bind server");

    tracing::info!("Server running on http://{}", listener.local_addr().unwrap());

    // Run our app with hyper
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
