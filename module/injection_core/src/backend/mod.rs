//! Backend selection module
//!
//! Provides unified `detect()` and `init()` functions that use either ORT or Burn backend
//! based on compile-time feature flags.

// ORT backend (priority if both features enabled)
#[cfg(all(feature = "backend-ort", not(feature = "backend-burn")))]
mod ort;
#[cfg(all(feature = "backend-ort", not(feature = "backend-burn")))]
pub use ort::{detect, init};

// Burn backend (only if ORT not enabled)
#[cfg(all(feature = "backend-burn", not(feature = "backend-ort")))]
mod burn;
#[cfg(all(feature = "backend-burn", not(feature = "backend-ort")))]
pub use burn::{detect, init};

// ORT takes priority if both enabled
#[cfg(all(feature = "backend-ort", feature = "backend-burn"))]
mod ort;
#[cfg(all(feature = "backend-ort", feature = "backend-burn"))]
pub use ort::{detect, init};
