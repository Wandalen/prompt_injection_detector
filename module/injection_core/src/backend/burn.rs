//! Burn backend with CUDA support
//!
//! Uses the burn framework with CUDA acceleration for inference

use anyhow::{Context, Result};
use burn::backend::cuda::{Cuda, CudaDevice};

// Re-export the inference functionality from burn_implementation

/// Detect prompt injection using Burn backend with CUDA
pub fn detect(text: &str) -> Result<String> {
    eprintln!("[Burn Backend] Using CUDA device");

    // Use the InferenceContext from burn_impl module
    use crate::burn_impl::InferenceContext;

    let device = CudaDevice::default();
    eprintln!("[Burn] CUDA device: {:?}", device);

    // Determine artifact directory (check multiple possible locations)
    let artifact_dir = if std::path::Path::new("artifacts").exists() {
        "artifacts"
    } else if std::path::Path::new("../artifacts").exists() {
        "../artifacts"
    } else if std::path::Path::new("../../artifacts").exists() {
        "../../artifacts"
    } else {
        return Err(anyhow::anyhow!("Could not find artifacts directory"));
    };

    let ctx = InferenceContext::<Cuda<f32>>::new(device, artifact_dir)
        .context("Failed to create Burn inference context")?;

    eprintln!("[Burn] Model loaded successfully");

    // Run inference
    let predictions = ctx.infer(vec![text.to_string()])
        .context("Failed to run Burn inference")?;

    if predictions.is_empty() {
        anyhow::bail!("No predictions returned");
    }

    let prediction = &predictions[0];

    eprintln!("[Burn] Logits: {:?}", prediction.logits);
    eprintln!("[Burn] Prediction: {} (class {})", prediction.class_name, prediction.class_index);

    // Map class names to expected format (benign/injection)
    // The model uses: 0=LEGITIMATE, 1=INJECTION
    let label = match prediction.class_name.to_lowercase().as_str() {
        "legitimate" => "benign",
        "injection" => "injection",
        _ => {
            // Fallback to class index
            if prediction.class_index == 0 { "benign" } else { "injection" }
        }
    };

    Ok(label.to_string())
}
