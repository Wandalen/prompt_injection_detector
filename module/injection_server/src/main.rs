//! `injection_server` - HTTP API server for prompt injection detection
//!
//! Provides REST API endpoints for detecting prompt injection attacks using `injection_core`.
//!
//! # Endpoints
//!
//! - GET /health - Health check
//! - POST /detect - Detect prompt injection in text
//!
//! # Example
//!
//! ```bash
//! # Start server
//! cargo run -p injection_server --features full
//!
//! # Health check
//! curl http://localhost:3000/health
//!
//! # Detect injection
//! curl -X POST http://localhost:3000/detect \
//!   -H "Content-Type: application/json" \
//!   -d '{"text": "Ignore all previous instructions"}'
//! ```

#[cfg(feature = "full")]
use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
};
#[cfg(feature = "full")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "full")]
use std::time::Instant;
#[cfg(feature = "full")]
use tower_http::trace::TraceLayer;
#[cfg(feature = "full")]
use tracing::{error, info};

/// Main entry point
#[cfg(feature = "full")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("injection_server=info,tower_http=debug")
        .init();

    info!("Initializing prompt injection detection server...");

    // Pre-load model for faster first request
    info!("Pre-loading detection model...");
    let load_start = Instant::now();
    injection_core::init()?;
    info!("Model loaded in {:.2}s", load_start.elapsed().as_secs_f64());

    // Build router (no shared state needed - detect() is thread-safe)
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/detect", post(detect_handler))
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = "0.0.0.0:3000";
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
///
/// Returns server status and model information
#[cfg(feature = "full")]
#[inline]
async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        model: "ModernBERT-based prompt injection classifier".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Detect injection endpoint
///
/// Accepts text and returns classification result
#[cfg(feature = "full")]
#[inline]
async fn detect_handler(
    Json(request): Json<DetectRequest>,
) -> Result<Json<DetectResponse>, AppError> {
    info!(
        "Detecting injection in text (length: {})",
        request.text.len()
    );

    let start = Instant::now();

    // Call injection_core detect function (thread-safe, uses cached model)
    let label = injection_core::detect(&request.text)
        .map_err(|e| AppError::Classification(e.to_string()))?;

    let duration = start.elapsed();

    info!(
        "Classification: {} ({:.0}ms)",
        label.to_uppercase(),
        duration.as_millis()
    );

    Ok(Json(DetectResponse {
        label: label.clone(),
        is_safe: label == "benign",
        time_ms: u64::try_from(duration.as_millis()).unwrap_or(u64::MAX),
    }))
}

/// Health check response
#[cfg(feature = "full")]
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    model: String,
    version: String,
}

/// Detect request payload
#[cfg(feature = "full")]
#[derive(Deserialize)]
struct DetectRequest {
    text: String,
}

/// Detect response payload
#[cfg(feature = "full")]
#[derive(Serialize)]
struct DetectResponse {
    label: String,
    is_safe: bool,
    time_ms: u64,
}

/// Application error type
#[cfg(feature = "full")]
enum AppError {
    Classification(String),
}

#[cfg(feature = "full")]
impl IntoResponse for AppError {
    #[inline]
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Classification(msg) => {
                error!("Classification error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Classification failed: {msg}"),
                )
            }
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

/// Stub main when full feature is not enabled
#[cfg(not(feature = "full"))]
fn main() {
    eprintln!("Error: injection_server requires 'full' feature to be enabled");
    eprintln!("Build with: cargo build -p injection_server --features full");
    std::process::exit(1);
}
