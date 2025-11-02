//! Burn backend with CUDA support
//!
//! Uses the burn framework with CUDA acceleration for inference

use anyhow::{Context, Result};
use burn::backend::cuda::{Cuda, CudaDevice};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::burn_impl::InferenceContext;

/// Internal detector holding Burn inference context
struct Detector {
    ctx: InferenceContext<Cuda<f32>>,
}

impl Detector {
    /// Create new detector by loading model and tokenizer
    fn new() -> Result<Self> {
        eprintln!("[Burn Backend] Initializing with CUDA device");

        let device = CudaDevice::default();
        eprintln!("[Burn] CUDA device: {:?}", device);

        // Determine artifact directory
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

        Ok(Self { ctx })
    }

    /// Perform inference on text
    fn detect_internal(&self, text: &str) -> Result<String> {
        // Run inference
        let predictions = self
            .ctx
            .infer(vec![text.to_string()])
            .context("Failed to run Burn inference")?;

        if predictions.is_empty() {
            anyhow::bail!("No predictions returned");
        }

        let prediction = &predictions[0];

        eprintln!("[Burn] Logits: {:?}", prediction.logits);
        eprintln!(
            "[Burn] Prediction: {} (class {})",
            prediction.class_name, prediction.class_index
        );

        // Map class names to expected format (benign/injection)
        // The model uses: 0=LEGITIMATE, 1=INJECTION
        let label = match prediction.class_name.to_lowercase().as_str() {
            "legitimate" => "benign",
            "injection" => "injection",
            _ => {
                // Fallback to class index
                if prediction.class_index == 0 {
                    "benign"
                } else {
                    "injection"
                }
            }
        };

        Ok(label.to_string())
    }
}

/// Global cached detector instance
/// Note: Using Mutex instead of RwLock because Burn's InferenceContext is not Sync
static DETECTOR: Lazy<Mutex<Option<Detector>>> = Lazy::new(|| Mutex::new(None));

/// Initialize the detector (pre-load model and tokenizer)
///
/// This is optional - the model will be loaded lazily on first `detect()` call if not called.
/// However, calling this explicitly at startup allows you to:
/// - Fail fast if model files are missing
/// - Avoid latency on first inference
/// - Pre-load in a controlled manner
pub fn init() -> Result<()> {
    let mut detector = DETECTOR.lock().unwrap();
    if detector.is_none() {
        *detector = Some(Detector::new()?);
    }
    Ok(())
}

/// Detect prompt injection using Burn backend with CUDA
///
/// The model is loaded lazily on first call, or eagerly if `init()` was called.
/// Subsequent calls reuse the cached model for fast inference.
///
/// Note: Uses Mutex for thread safety since Burn's tensors don't implement Sync
pub fn detect(text: &str) -> Result<String> {
    // Try to use existing detector first
    {
        let detector = DETECTOR.lock().unwrap();
        if let Some(d) = detector.as_ref() {
            return d.detect_internal(text);
        }
    }

    // Slow path: initialize detector (first call only)
    init()?;

    // Retry with initialized detector
    let detector = DETECTOR.lock().unwrap();
    detector
        .as_ref()
        .expect("Detector should be initialized after init()")
        .detect_internal(text)
}
