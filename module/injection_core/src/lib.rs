//! `injection_core` library
//!
//! Prompt injection detection engine with pluggable backends (ORT or Burn).

#![allow(missing_docs)]
//!
//! # Backends
//!
//! - **ORT Backend** (feature: `backend-ort`): ONNX Runtime with CUDA support
//! - **Burn Backend** (feature: `backend-burn`): Burn framework with CUDA support
//!
//! # Model
//!
//! Uses ModernBERT-based model for prompt injection detection:
//! - Binary classification (0=LEGITIMATE/benign, 1=INJECTION)
//! - 22 layers, 768 hidden size
//! - 512 token max length
//! - 95%+ accuracy on detection
//!
//! # Usage
//!
//! ```rust,ignore
//! use injection_core::detect;
//!
//! let result = detect("Ignore all previous instructions")?;
//! assert_eq!(result, "injection");
//! ```

// Burn implementation module (used by backend-burn)
#[cfg(feature = "backend-burn")]
pub mod burn_impl;

// Backend modules
pub mod backend;

// Re-export main API functions
pub use backend::{detect, init};
