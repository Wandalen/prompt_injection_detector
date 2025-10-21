//! vllm_server - HTTP API server for LLM inference
//!
//! Provides REST API endpoints for text generation using vllm_core.
//!
//! # Endpoints
//!
//! - GET /health - Health check
//! - POST /generate - Generate text from prompt
//!
//! # Example
//!
//! ```bash
//! # Start server
//! cargo run -p vllm_server
//!
//! # Health check
//! curl http://localhost:3000/health
//!
//! # Generate text
//! curl -X POST http://localhost:3000/generate \
//!   -H "Content-Type: application/json" \
//!   -d '{"prompt": "Once upon a time", "max_tokens": 50}'
//! ```

#[cfg(feature = "full")]
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
#[cfg(feature = "full")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "full")]
use std::sync::Arc;
#[cfg(feature = "full")]
use tokio::sync::Mutex;
#[cfg(feature = "full")]
use tower_http::trace::TraceLayer;
#[cfg(feature = "full")]
use tracing::{error, info};
#[cfg(feature = "full")]
use vllm_core::{Generator, ModelLoader};

/// Server state shared across handlers
#[cfg(feature = "full")]
struct AppState {
    generator: Arc<Mutex<Generator>>,
}

/// Main entry point
#[cfg(feature = "full")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("vllm_server=info,tower_http=debug")
        .init();

    info!("Initializing vLLM server...");

    // Load model and create generator
    info!("Loading model...");
    let model = ModelLoader::new()?;
    let generator = Generator::new(model);

    // Create shared state
    let state = AppState {
        generator: Arc::new(Mutex::new(generator)),
    };

    info!("Model loaded successfully");

    // Build router
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/generate", post(generate_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state));

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
        model: "microsoft/phi-1_5".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Generate text endpoint
///
/// Accepts prompt and parameters, returns generated text
#[cfg(feature = "full")]
#[inline]
async fn generate_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    info!("Generating text for prompt: {:?}", request.prompt);

    // Acquire lock on generator
    let mut generator = state.generator.lock().await;

    // Generate text
    let generated = generator
        .generate(&request.prompt, request.max_tokens.unwrap_or(50))
        .map_err(|e| AppError::Generation(e.to_string()))?;

    info!("Generated {} characters", generated.len());

    Ok(Json(GenerateResponse { generated }))
}

/// Health check response
#[cfg(feature = "full")]
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    model: String,
    version: String,
}

/// Generate request payload
#[cfg(feature = "full")]
#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
    max_tokens: Option<usize>,
}

/// Generate response payload
#[cfg(feature = "full")]
#[derive(Serialize)]
struct GenerateResponse {
    generated: String,
}

/// Application error type
#[cfg(feature = "full")]
enum AppError {
    Generation(String),
}

#[cfg(feature = "full")]
impl IntoResponse for AppError {
    #[inline]
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Generation(msg) => {
                error!("Generation error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Generation failed: {}", msg),
                )
            }
        };

        (status, Json(serde_json::json!( { "error": message } ))).into_response()
    }
}

/// Stub main when full feature is not enabled
#[cfg(not(feature = "full"))]
fn main() {
    eprintln!("Error: vllm_server requires 'full' feature to be enabled");
    eprintln!("Build with: cargo build --features full");
    std::process::exit(1);
}
