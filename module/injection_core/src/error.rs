//! Error types for vllm_inferencer
//!
//! All errors use `error_tools` crate per workspace standards.

use error_tools::dependency::thiserror;
use error_tools::dependency::thiserror::Error;

/// Main error type for inference operations
#[derive(Debug, Error)]
pub enum InferenceError {
    /// Model loading failed
    #[error("Model loading error: {0}")]
    ModelLoadingError(String),

    /// Tokenization failed
    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    /// Text generation failed
    #[error("Generation error: {0}")]
    GenerationError(String),

    /// HTTP server error (future)
    #[error("HTTP error: {0}")]
    HttpError(String),

    /// GPU/CUDA error
    #[error("GPU error: {0}")]
    GpuError(String),

    /// Generic error
    #[error("Inference error: {0}")]
    Generic(String),
}

/// Result type alias using InferenceError
pub type Result<T> = std::result::Result<T, InferenceError>;
