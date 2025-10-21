//! Text generation logic
//!
//! Implements:
//! - Greedy Sampling (MVP)
//! - KV Cache optimization (future)
//!
//! # Generation Loop
//!
//! 1. Tokenize prompt
//! 2. For each step until max_tokens:
//!    - Run forward pass (use KV Cache in future)
//!    - Greedy sample (argmax)
//!    - Append token to sequence
//! 3. Decode tokens to text

use crate::error::{InferenceError, Result};
use crate::model::ModelLoader;
use tokenizers::Encoding;

/// Text generator with optional KV Cache
pub struct Generator {
    model: ModelLoader,
    // TODO (future): Add KV Cache fields
    // kv_cache: Option<KVCache>,
}

impl Generator {
    /// Create new generator
    #[inline]
    pub fn new(model: ModelLoader) -> Self {
        Self { model }
    }

    /// Generate text from prompt
    ///
    /// # Arguments
    /// * `prompt` - Input text
    /// * `max_tokens` - Maximum tokens to generate
    ///
    /// # Returns
    /// Generated text string
    #[inline]
    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String> {
        eprintln!("Tokenizing prompt...");

        // 1. Tokenize prompt
        let encoding = self.tokenize(prompt)?;
        let mut tokens: Vec<u32> = encoding.get_ids().to_vec();

        eprintln!("Initial tokens: {:?}", tokens);
        eprintln!("Generating {} tokens...", max_tokens);

        // 2. Generation loop
        for step in 0..max_tokens {
            // Run forward pass
            let logits = self.model.forward(&tokens)?;

            // Greedy sampling (argmax)
            let next_token = self.greedy_sample(&logits);

            eprintln!("Step {}: Generated token {}", step, next_token);

            // Append to sequence
            tokens.push(next_token);

            // Optional: Check for EOS token (GPT-2 uses token 50256)
            if next_token == 50256 {
                eprintln!("EOS token reached");
                break;
            }
        }

        eprintln!("Decoding tokens...");

        // 3. Decode tokens to text
        let generated_text = self.decode(&tokens)?;

        Ok(generated_text)
    }

    /// Tokenize input text
    #[inline]
    fn tokenize(&self, text: &str) -> Result<Encoding> {
        let encoding =
            self.model.tokenizer().encode(text, false).map_err(|e| {
                InferenceError::TokenizationError(format!("Failed to tokenize: {}", e))
            })?;

        Ok(encoding)
    }

    /// Decode token IDs to text
    #[inline]
    fn decode(&self, tokens: &[u32]) -> Result<String> {
        let text =
            self.model.tokenizer().decode(tokens, true).map_err(|e| {
                InferenceError::TokenizationError(format!("Failed to decode: {}", e))
            })?;

        Ok(text)
    }

    /// Greedy sampling (select highest probability token)
    ///
    /// Finds the token with maximum logit value (argmax)
    #[inline]
    fn greedy_sample(&self, logits: &[f32]) -> u32 {
        let mut max_idx = 0;
        let mut max_val = logits[0];

        for (idx, &val) in logits.iter().enumerate().skip(1) {
            if val > max_val {
                max_val = val;
                max_idx = idx;
            }
        }

        max_idx as u32
    }
}
