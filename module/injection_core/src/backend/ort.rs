//! ORT (ONNX Runtime) backend with CUDA support

use anyhow::{Context, Result};
use ort::execution_providers::cuda::CUDAExecutionProvider;
use ort::session::{Session, builder::GraphOptimizationLevel};
use ort::value::TensorRef;
use tokenizers::Tokenizer;
use ndarray::Array2;

/// Detect prompt injection using ORT backend with CUDA
pub fn detect(text: &str) -> Result<String> {
    eprintln!("[ORT Backend] Using CUDA device");

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

    let model_path = format!("{}/model.onnx", artifact_dir);
    let tokenizer_path = format!("{}/tokenizer/tokenizer.json", artifact_dir);

    // Load model with CUDA support
    let mut session = Session::builder()
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

    // Tokenize input
    let encoding = tokenizer.encode(text, true)
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
    let inputs = ort::inputs![
        TensorRef::from_array_view(input_ids_array.view())?,
        TensorRef::from_array_view(attention_mask_array.view())?
    ];
    let outputs = session.run(inputs)
        .context("Failed to run inference")?;

    // Extract logits (output shape: [batch_size, num_classes])
    let (_, logits_data) = outputs["logits"]
        .try_extract_tensor::<f32>()
        .context("Failed to extract logits tensor")?;

    eprintln!("[ORT] Logits: {:?}", &logits_data[..2.min(logits_data.len())]);

    // Get prediction (class 0 = LEGITIMATE/benign, class 1 = INJECTION)
    let class_idx = if logits_data[0] > logits_data[1] { 0 } else { 1 };

    let label = if class_idx == 0 { "benign" } else { "injection" };

    eprintln!("[ORT] Prediction: {} (class {})", label, class_idx);

    Ok(label.to_string())
}
