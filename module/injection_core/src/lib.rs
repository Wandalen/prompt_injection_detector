//! injection_core library
//!
//! Prompt injection detection engine using ONNX Runtime and DeBERTa model.
//!
//! # Architecture
//!
//! - `model` - Model loading with ONNX Runtime
//! - `classify` - Binary classification (benign/injection)
//! - `error` - Error types using error_tools
//!
//! # Model
//!
//! Uses ProtectAI's deberta-v3-base-prompt-injection-v2:
//! - Binary classification (0=benign, 1=injection)
//! - 200M parameters
//! - 512 token max length
//! - 95%+ accuracy on detection
//!
//! # Milestone
//!
//! **MVP:** Detect prompt injection attacks (5 days)
//! - Load DeBERTa model via ONNX Runtime
//! - Classify text as benign or injection
//! - Return confidence scores

#[cfg(feature = "full")]
pub mod error;
#[cfg(feature = "full")]
pub mod classify;
#[cfg(feature = "full")]
pub mod model;

// Re-export main types for convenience
#[cfg(feature = "full")]
pub use error::DetectionError;
#[cfg(feature = "full")]
pub use classify::Classifier;
#[cfg(feature = "full")]
pub use model::ModelLoader;
