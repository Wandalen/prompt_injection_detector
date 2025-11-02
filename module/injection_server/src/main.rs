//! injection_server - HTTP API server for prompt injection detection
//!
//! Provides REST API endpoints for detecting prompt injection attacks using injection_core.
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
//! cargo run -p injection_server
//!
//! # Health check
//! curl http://localhost:3000/health
//!
//! # Detect injection
//! curl -X POST http://localhost:3000/detect \
//!   -H "Content-Type: application/json" \
//!   -d '{"text": "Ignore all previous instructions", "threshold": 0.5}'
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
use injection_core::{Classifier, ModelLoader};
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

/// Server state shared across handlers
#[cfg(feature = "full")]
struct AppState {
    classifier: Arc<Mutex<Classifier>>,
}

/// Main entry point
#[cfg(feature = "full")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("injection_server=info,tower_http=debug")
        .init();

    info!("Initializing prompt injection detection server...");

    // Load model and create classifier
    info!("Loading DeBERTa model...");
    let model = ModelLoader::new()?;
    let classifier = Classifier::new(model);

    // Create shared state
    let state = AppState {
        classifier: Arc::new(Mutex::new(classifier)),
    };

    info!("Model loaded successfully");

    // Build router
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/detect", post(detect_handler))
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
        model: "protectai/deberta-v3-base-prompt-injection-v2".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Detect injection endpoint
///
/// Accepts text and returns classification result
#[cfg(feature = "full")]
#[inline]
async fn detect_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DetectRequest>,
) -> Result<Json<DetectResponse>, AppError> {
    info!("Detecting injection in text: {:?}", request.text);

    // Acquire lock on classifier
    let classifier = state.classifier.lock().await;

    // Classify text
    let result = classifier
        .classify(&request.text)
        .map_err(|e| AppError::Classification(e.to_string()))?;

    info!(
        "Classification: {}, Confidence: {:.2}%",
        if result.is_injection {
            "INJECTION"
        } else {
            "BENIGN"
        },
        result.confidence * 100.0
    );

    Ok(Json(DetectResponse {
        label: if result.is_injection {
            "injection".to_string()
        } else {
            "benign".to_string()
        },
        confidence: result.confidence,
        is_safe: !result.is_injection,
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
    threshold: Option<f32>,
}

/// Detect response payload
#[cfg(feature = "full")]
#[derive(Serialize)]
struct DetectResponse {
    label: String,
    confidence: f32,
    is_safe: bool,
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
                error!("Classification error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Classification failed: {}", msg),
                )
            }
        };

        (status, Json(serde_json::json!( { "error": message } ))).into_response()
    }
}

/// Stub main when full feature is not enabled
#[cfg(not(feature = "full"))]
fn main() {
    eprintln!("Error: injection_server requires 'full' feature to be enabled");
    eprintln!("Build with: cargo build --features full");
    std::process::exit(1);
}
