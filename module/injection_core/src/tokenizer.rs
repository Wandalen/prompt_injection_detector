use tokenizers::Tokenizer;
use anyhow::Result;
#[derive(Debug, Clone)]
pub struct TokenizedBatch {
    pub input_ids: Vec<Vec<u32>>,
    pub attention_mask: Vec<Vec<u32>>,
}

impl TokenizedBatch {
    pub fn batch_size(&self) -> usize {
        self.input_ids.len()
    }

    pub fn sequence_length(&self) -> usize {
        self.input_ids.get(0).map(|v| v.len()).unwrap_or(0)
    }

    /// Flatten to 1D array for frameworks that need it
    pub fn flatten_input_ids(&self) -> Vec<i64> {
        self.input_ids
            .iter()
            .flat_map(|ids| ids.iter().map(|&id| id as i64))
            .collect()
    }

    pub fn flatten_attention_mask(&self) -> Vec<i64> {
        self.attention_mask
            .iter()
            .flat_map(|mask| mask.iter().map(|&m| m as i64))
            .collect()
    }
}

pub fn tokenize_batch(
    tokenizer: &Tokenizer,
    texts: &[&str],
    max_length: usize,
) -> Result<TokenizedBatch> {
    let encoding = tokenizer
        .encode_batch(texts.to_vec(), true)
        .map_err(|e| anyhow::anyhow!("Tokenization failed: {}", e))?;

    let mut input_ids = Vec::new();
    let mut attention_mask = Vec::new();

    for enc in encoding {
        let ids = enc.get_ids();
        let mask = enc.get_attention_mask();

        // Pad or truncate to max_length
        let mut padded_ids = ids.to_vec();
        let mut padded_mask = mask.to_vec();

        padded_ids.resize(max_length, 0);
        padded_mask.resize(max_length, 0);

        input_ids.push(padded_ids);
        attention_mask.push(padded_mask);
    }

    Ok(TokenizedBatch {
        input_ids,
        attention_mask,
    })
}