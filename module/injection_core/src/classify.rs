use tokenizers::Tokenizer;
use crate::model::Model;
use anyhow::{anyhow, Result};
pub struct Classifier{
    model: Model,
    max_length: usize
}
use ndarray::Array;
use crate::tokenizer::tokenize_batch;

impl Classifier{
    pub fn new(model: Model) -> Self{
        Self{model , max_length:2048}
    }

    pub fn classify(&mut self, input: String) -> Result<String> {
        // Tokenize input text
        let batch = tokenize_batch(&self.model.tokenizer, &[&input], self.max_length)
            .map_err(|e| anyhow!("Tokenization failed: {}", e))?;

        // Run model prediction (returns Vec<usize>)
        let predictions = self.model.predict(&batch)
            .map_err(|e| anyhow!("Model prediction failed: {}", e))?;

        // Extract the first (and only) prediction
        let prediction = predictions
            .first()
            .ok_or_else(|| anyhow!("Empty prediction output"))?;

        Ok(prediction.to_string())
    }

}
