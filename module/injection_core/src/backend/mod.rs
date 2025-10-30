//! Backend selection module
//!
//! Provides unified `detect()` function that uses either ORT or Burn backend
//! based on compile-time feature flags.

#[cfg(feature = "backend-ort")]
mod ort;
#[cfg(feature = "backend-ort")]
pub use ort::detect;

#[cfg(feature = "backend-burn")]
mod burn;
#[cfg(feature = "backend-burn")]
pub use burn::detect;
