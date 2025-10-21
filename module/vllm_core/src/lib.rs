//! vllm_inferencer library
//!
//! Ultra-minimal LLM inference engine using Candle framework.
//!
//! # architecture
//!
//! - `model` - Model loading and forward pass
//! - `generate` - Text generation logic and KV Cache (future)
//! - `error` - Error types using error_tools
//!
//! # milestone
//!
//! **MVP:** Prove Candle works (3-5 days)
//! - Load GPT-2 Small to GPU
//! - Generate text from prompt
//! - Print to console

#[cfg(feature = "full")]
pub mod error;
#[cfg(feature = "full")]
pub mod generate;
#[cfg(feature = "full")]
pub mod model;

// Re-export main types for convenience
#[cfg(feature = "full")]
pub use error::InferenceError;
#[cfg(feature = "full")]
pub use generate::Generator;
#[cfg(feature = "full")]
pub use model::ModelLoader;
