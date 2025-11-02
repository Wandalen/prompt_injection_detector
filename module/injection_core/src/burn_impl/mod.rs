//! Burn implementation module
//!
//! Contains the Burn-based model inference code

pub mod config;
pub mod data;
pub mod inference;
pub mod model;

// Re-export main types
pub use config::ExperimentConfig;
pub use data::{BertCasedTokenizer, TextBatch, TextClassificationBatcher};
pub use inference::{InferenceContext, Prediction, infer};
pub use model::DebertaModel;
