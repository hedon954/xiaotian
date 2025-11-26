mod config;
mod domain;
mod shared;

use axum::{
    extract::DefaultBodyLimit,
    http::{Method, StatusCode},
    response::Response,
    routing::{delete, get, post, put},
    Router,
};
use config::Config;
use shared::{ApiResponse, AppError};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn health_check() -> Result<ApiResponse<serde_json::Value>, AppError> {
    Ok(ApiResponse::success(
        serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now(),
            "version": "0.1.0",
            "uptime": 0
        }),
        "Service is healthy",
    ))
}

async fn not_found() -> Result<ApiResponse<serde_json::Value>, AppError> {
    Err(AppError::NotFound {
        entity: "Endpoint".to_string(),
        id: "unknown".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load()?;

    // Initialize tracing
    init_tracing(&config);

    info!("Starting {} v{}", config.app.name, config.app.version);
    info!("Environment: {}", config.app.environment);

    // Build the application router
    let app = create_app(&config).await?;

    // Create the server address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    info!("Server listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_app(config: &Config) -> Result<Router, Box<dyn std::error::Error>> {
    // Create base router
    let app = Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // API v1 routes
        .nest("/api/v1", create_api_routes())
        // Fallback handler for 404
        .fallback(not_found);

    // Add middleware layers
    let app = app.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(Any),
            )
            .layer(DefaultBodyLimit::max(config.server.max_request_size)),
    );

    Ok(app)
}

fn create_api_routes() -> Router {
    Router::new()
        // Placeholder routes - these would be implemented in the interfaces layer
        .route("/feeds", get(|| async { ApiResponse::success(vec![], "Feeds retrieved successfully") }))
        .route("/feeds", post(|| async { ApiResponse::success(serde_json::json!({"id": 1}), "Feed created successfully") }))
        .route("/feeds/:id", get(|| async { ApiResponse::success(serde_json::json!({"id": 1}), "Feed retrieved successfully") }))
        .route("/feeds/:id", put(|| async { ApiResponse::success(serde_json::json!({"id": 1}), "Feed updated successfully") }))
        .route("/feeds/:id", delete(|| async { ApiResponse::success(serde_json::Value::Null, "Feed deleted successfully") }))

        .route("/summaries", get(|| async { ApiResponse::success(vec![], "Summaries retrieved successfully") }))
        .route("/summaries/:id", get(|| async { ApiResponse::success(serde_json::json!({"id": 1}), "Summary retrieved successfully") }))

        .route("/chat/sessions", get(|| async { ApiResponse::success(vec![], "Chat sessions retrieved successfully") }))
        .route("/chat/sessions", post(|| async { ApiResponse::success(serde_json::json!({"id": 1}), "Chat session created successfully") }))
        .route("/chat/sessions/:id", get(|| async { ApiResponse::success(serde_json::json!({"id": 1}), "Chat session retrieved successfully") }))
        .route("/chat/sessions/:id/messages", post(|| async { ApiResponse::success(serde_json::json!({}), "Message sent successfully") }))

        .route("/sync/manual", post(|| async { ApiResponse::success(serde_json::json!({"syncId": "sync-123"}), "Sync started") }))
        .route("/sync/status", get(|| async { ApiResponse::success(serde_json::json!({"isRunning": false}), "Sync status retrieved") }))

        .route("/tasks/scheduled", get(|| async { ApiResponse::success(vec![], "Tasks retrieved successfully") }))
        .route("/tasks/scheduled", post(|| async { ApiResponse::success(serde_json::json!({"id": "task-123"}), "Task created successfully") }))

        .route("/email/config", get(|| async { ApiResponse::success(serde_json::json!({"enabled": false}), "Email config retrieved") }))
        .route("/email/config", put(|| async { ApiResponse::success(serde_json::json!({"enabled": false}), "Email config updated") }))

        .route("/system/stats", get(|| async { ApiResponse::success(serde_json::json!({"feeds": {"total": 0}}), "System stats retrieved") }))
        .route("/system/health", get(|| async { ApiResponse::success(serde_json::json!({"status": "healthy"}), "System health retrieved") }));
}

fn init_tracing(config: &Config) {
    let filter = match config.app.log_level.as_str() {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter.to_string())),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_not_found() {
        let response = not_found().await;
        assert!(response.is_err());
    }

    #[test]
    fn test_config_loading() {
        let config = Config::load();
        assert!(config.is_ok());
    }
}