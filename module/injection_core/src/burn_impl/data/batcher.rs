use super::BertCasedTokenizer;
use anyhow::Result;
use burn::prelude::*;
use burn::tensor::Tensor;
use burn::tensor::backend::Backend;
use std::sync::Arc;

/// Tensor pair used by transformer encoders (input ids + attention mask).
pub struct TextBatch<B: Backend> {
    pub input_ids: Tensor<B, 2, Int>,
    pub attention_mask: Tensor<B, 2, Int>,
}

/// Prepares mini-batches for text classification inference.
pub struct TextClassificationBatcher {
    tokenizer: Arc<BertCasedTokenizer>,
    seq_length: usize,
}

impl TextClassificationBatcher {
    pub fn new(tokenizer: Arc<BertCasedTokenizer>, seq_length: usize) -> Self {
        Self {
            tokenizer,
            seq_length,
        }
    }

    pub fn batch<B: Backend>(
        &self,
        texts: Vec<String>,
        device: &B::Device,
    ) -> Result<TextBatch<B>> {
        let encodings = self.tokenizer.encode_batch(&texts, self.seq_length)?;

        let batch_size = encodings.len();
        if batch_size == 0 {
            let zeros = Tensor::<B, 2, Int>::zeros([0, self.seq_length], device);
            return Ok(TextBatch {
                input_ids: zeros.clone(),
                attention_mask: zeros,
            });
        }

        let token_ids: Vec<i64> = encodings
            .iter()
            .flat_map(|encoding| encoding.get_ids().iter().map(|id| *id as i64))
            .collect();
        let attention_masks: Vec<i64> = encodings
            .iter()
            .flat_map(|encoding| {
                encoding
                    .get_attention_mask()
                    .iter()
                    .map(|value| *value as i64)
            })
            .collect();

        let input_ids_data = TensorData::new(token_ids, [batch_size, self.seq_length]);
        let attention_mask_data = TensorData::new(attention_masks, [batch_size, self.seq_length]);

        let input_ids = Tensor::<B, 2, Int>::from_data(input_ids_data, device);
        let attention_mask = Tensor::<B, 2, Int>::from_data(attention_mask_data, device);

        Ok(TextBatch {
            input_ids,
            attention_mask,
        })
    }
}
