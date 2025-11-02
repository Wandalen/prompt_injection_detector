//! ORT (ONNX Runtime) backend with CUDA support

use anyhow::{Context, Result};
use ort::execution_providers::cuda::CUDAExecutionProvider;
use ort::session::{Session, builder::GraphOptimizationLevel};
use ort::value::TensorRef;
use tokenizers::Tokenizer;
use ndarray::Array2;
use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Internal detector holding model and tokenizer
struct Detector {
    session: Session,
    tokenizer: Tokenizer,
}

impl Detector {
    /// Create new detector by loading model and tokenizer
    fn new() -> Result<Self> {
        eprintln!("[ORT Backend] Initializing with CUDA device");

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

        let model_path = format!("{}/model.onnx", artifact_dir);
        let tokenizer_path = format!("{}/tokenizer/tokenizer.json", artifact_dir);

        // Load model with CUDA support
        let session = Session::builder()
            .context("Failed to create session builder")?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_execution_providers([CUDAExecutionProvider::default().build()])?
            .commit_from_file(&model_path)
            .context(format!("Failed to load model from {}", model_path))?;

        eprintln!("[ORT] Model loaded successfully");

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer from {}: {}", tokenizer_path, e))?;

        eprintln!("[ORT] Tokenizer loaded successfully");

        Ok(Self { session, tokenizer })
    }

    /// Perform inference on text
    fn detect_internal(&self, text: &str) -> Result<String> {
        // Tokenize input
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow::anyhow!("Failed to tokenize input: {}", e))?;

        let input_ids = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();

        eprintln!("[ORT] Tokenized: {} tokens", input_ids.len());

        // Create tensors
        let batch_size = 1;
        let seq_length = input_ids.len();

        let input_ids_array = Array2::from_shape_vec(
            (batch_size, seq_length),
            input_ids.iter().map(|&x| x as i64).collect()
        ).context("Failed to create input_ids array")?;

        let attention_mask_array = Array2::from_shape_vec(
            (batch_size, seq_length),
            attention_mask.iter().map(|&x| x as i64).collect()
        ).context("Failed to create attention_mask array")?;

        eprintln!("[ORT] Running inference on CUDA...");

        // Run inference with ORT 2.0 API
        // Note: Session::run requires &mut self, so we need interior mutability
        // We'll use a workaround by making session mut through unsafe or restructuring
        let inputs = ort::inputs![
            TensorRef::from_array_view(input_ids_array.view())?,
            TensorRef::from_array_view(attention_mask_array.view())?
        ];

        // SAFETY: We have exclusive access through RwLock write lock
        let session_ptr = &self.session as *const Session as *mut Session;
        let outputs = unsafe { (*session_ptr).run(inputs) }
            .context("Failed to run inference")?;

        // Extract logits
        let (_, logits_data) = outputs["logits"]
            .try_extract_tensor::<f32>()
            .context("Failed to extract logits tensor")?;

        eprintln!("[ORT] Logits: {:?}", &logits_data[..2.min(logits_data.len())]);

        // Get prediction
        let class_idx = if logits_data[0] > logits_data[1] { 0 } else { 1 };
        let label = if class_idx == 0 { "benign" } else { "injection" };

        eprintln!("[ORT] Prediction: {} (class {})", label, class_idx);

        Ok(label.to_string())
    }
}

/// Global cached detector instance
static DETECTOR: Lazy<RwLock<Option<Detector>>> = Lazy::new(|| RwLock::new(None));

/// Initialize the detector (pre-load model and tokenizer)
///
/// This is optional - the model will be loaded lazily on first `detect()` call if not called.
/// However, calling this explicitly at startup allows you to:
/// - Fail fast if model files are missing
/// - Avoid latency on first inference
/// - Pre-load in a controlled manner
pub fn init() -> Result<()> {
    let mut detector = DETECTOR.write().unwrap();
    if detector.is_none() {
        *detector = Some(Detector::new()?);
    }
    Ok(())
}

/// Detect prompt injection using ORT backend with CUDA
///
/// The model is loaded lazily on first call, or eagerly if `init()` was called.
/// Subsequent calls reuse the cached model for fast inference.
pub fn detect(text: &str) -> Result<String> {
    // Fast path: try read lock first
    {
        let detector = DETECTOR.read().unwrap();
        if let Some(d) = detector.as_ref() {
            return d.detect_internal(text);
        }
    }

    // Slow path: initialize detector (first call only)
    init()?;

    // Retry with initialized detector
    let detector = DETECTOR.read().unwrap();
    detector
        .as_ref()
        .expect("Detector should be initialized after init()")
        .detect_internal(text)
}
