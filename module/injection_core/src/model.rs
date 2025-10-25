//! Model loading and management
//!
//! Responsibilities:
//! - Download Phi model from HuggingFace Hub
//! - Load SafeTensors format to GPU memory
//! - Provide forward pass API
//!
//! # Milestone 1 Implementation
//!
//! 1. Load Phi model using Candle
//! 2. Verify GPU allocation
//! 3. Validate model architecture

use crate::error::{InferenceError, Result};
use candle_core::{Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::phi::{Config, Model};
use hf_hub::{api::sync::ApiBuilder, Repo, RepoType};
use tokenizers::Tokenizer;
use workspace_tools::Workspace;

/// Model loader for Phi
pub struct ModelLoader {
    model: Model,
    device: Device,
    tokenizer: Tokenizer,
}

impl ModelLoader {
    /// Create new model loader
    ///
    /// # Example
    /// ```ignore
    /// let loader = ModelLoader::new()?;
    /// ```
    #[inline]
    pub fn new() -> Result<Self> {
        eprintln!("Initializing GPU device...");

        // 1. Initialize CUDA device (prefer GPU, fallback to CPU for testing)
        let device = Device::cuda_if_available(0).map_err(|e| {
            InferenceError::ModelLoadingError(format!("Failed to initialize device: {}", e))
        })?;

        eprintln!("Device: {:?}", device);

        // 2. Load tokenizer from HuggingFace Hub
        eprintln!("Loading tokenizer...");
        let tokenizer = load_tokenizer()?;

        // 3. Load Phi model config and weights
        eprintln!("Loading Phi model configuration...");
        let (model, _config) = load_phi_model(&device)?;

        eprintln!("Model loaded successfully!");

        Ok(Self {
            model,
            device,
            tokenizer,
        })
    }

    /// Run forward pass on tokens
    ///
    /// # Arguments
    /// * `tokens` - Input token IDs
    ///
    /// # Returns
    /// Logits tensor for next token prediction
    #[inline]
    pub fn forward(&mut self, tokens: &[u32]) -> Result<Vec<f32>> {
        // Convert tokens to u32 vec then to tensor
        let input_ids = Tensor::new(tokens, &self.device)
            .map_err(|e| {
                InferenceError::GenerationError(format!("Failed to create tensor: {}", e))
            })?
            .unsqueeze(0) // Add batch dimension
            .map_err(|e| {
                InferenceError::GenerationError(format!("Failed to add batch dim: {}", e))
            })?;

        // Run forward pass
        let logits = self
            .model
            .forward(&input_ids)
            .map_err(|e| InferenceError::GenerationError(format!("Forward pass failed: {}", e)))?;

        // Get last token's logits - shape is [batch, seq_len, vocab_size]
        let seq_len = tokens.len();
        let logits_last = logits
            .i((0, seq_len - 1)) // Get batch 0, last position
            .map_err(|e| {
                InferenceError::GenerationError(format!("Failed to extract logits: {}", e))
            })?;

        // Convert to Vec<f32>
        let logits_vec = logits_last.to_vec1::<f32>().map_err(|e| {
            InferenceError::GenerationError(format!("Failed to convert logits: {}", e))
        })?;

        Ok(logits_vec)
    }

    /// Get reference to tokenizer
    #[inline]
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }
}

/// Load Phi tokenizer from HuggingFace Hub
#[inline]
fn load_tokenizer() -> Result<Tokenizer> {
    eprintln!("Downloading tokenizer from HuggingFace...");

    // Initialize HF Hub API with optional token from secrets
    let api = create_hf_api()?;

    // Get Phi-1.5 repository (smaller model for MVP)
    let repo = api.repo(Repo::new("microsoft/phi-1_5".to_string(), RepoType::Model));

    // Download tokenizer file
    let tokenizer_path = repo.get("tokenizer.json").map_err(|e| {
        InferenceError::TokenizationError(format!("Failed to download tokenizer: {}", e))
    })?;

    // Load tokenizer from file
    let tokenizer = Tokenizer::from_file(tokenizer_path).map_err(|e| {
        InferenceError::TokenizationError(format!("Failed to load tokenizer: {}", e))
    })?;

    Ok(tokenizer)
}

/// Create HuggingFace API client with optional token from secrets
#[inline]
fn create_hf_api() -> Result<hf_hub::api::sync::Api> {
    // Try to load HF token from workspace secrets
    let token = load_hf_token_from_secrets();

    let mut builder = ApiBuilder::new();

    if let Some(token) = token {
        eprintln!("Using HuggingFace token from workspace secrets");
        builder = builder.with_token(Some(token));
    } else {
        eprintln!("No HuggingFace token found, using anonymous access");
    }

    let api = builder.build().map_err(|e| {
        InferenceError::ModelLoadingError(format!("Failed to initialize HF API: {}", e))
    })?;

    Ok(api)
}

/// Load HuggingFace token from workspace secrets
///
/// Searches for token in:
/// 1. secret/hf_token.txt
/// 2. secret/huggingface_token.txt
/// 3. Environment variable HF_TOKEN
#[inline]
fn load_hf_token_from_secrets() -> Option<String> {
    // Try environment variable first
    if let Ok(token) = std::env::var("HF_TOKEN") {
        if !token.is_empty() {
            return Some(token);
        }
    }

    // Try to find workspace root and load from secret/ directory
    let workspace = Workspace::from_current_dir().ok()?;
    let workspace_root = workspace.root();

    // Try different secret file names
    let secret_paths = [
        workspace_root.join("secret/hf_token.txt"),
        workspace_root.join("secret/huggingface_token.txt"),
    ];

    for path in &secret_paths {
        if let Ok(token) = std::fs::read_to_string(path) {
            let token = token.trim().to_string();
            if !token.is_empty() {
                return Some(token);
            }
        }
    }

    None
}

/// Load Phi model from HuggingFace Hub
///
/// Returns the loaded model and config
#[inline]
fn load_phi_model(device: &Device) -> Result<(Model, Config)> {
    eprintln!("Downloading Phi model from HuggingFace...");

    // Initialize HF Hub API with secrets
    let api = create_hf_api()?;

    // Get Phi-1.5 repository
    let repo = api.repo(Repo::new("microsoft/phi-1_5".to_string(), RepoType::Model));

    // Download config and weights
    let config_path = repo.get("config.json").map_err(|e| {
        InferenceError::ModelLoadingError(format!("Failed to download config: {}", e))
    })?;

    let model_path = repo.get("model.safetensors").map_err(|e| {
        InferenceError::ModelLoadingError(format!("Failed to download model: {}", e))
    })?;

    eprintln!("Loading model configuration...");

    // Load config
    let config_str = std::fs::read_to_string(config_path)
        .map_err(|e| InferenceError::ModelLoadingError(format!("Failed to read config: {}", e)))?;

    let config: Config = serde_json::from_str(&config_str)
        .map_err(|e| InferenceError::ModelLoadingError(format!("Failed to parse config: {}", e)))?;

    eprintln!("Loading model weights...");

    // Load weights using VarBuilder
    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[model_path], candle_core::DType::F32, device)
    }
    .map_err(|e| InferenceError::ModelLoadingError(format!("Failed to load weights: {}", e)))?;

    // Create model
    let model = Model::new(&config, vb)
        .map_err(|e| InferenceError::ModelLoadingError(format!("Failed to create model: {}", e)))?;

    Ok((model, config))
}
