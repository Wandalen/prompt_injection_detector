use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tokenizers::tokenizer::{Encoding, Tokenizer};
use tokenizers::{
    PaddingDirection, PaddingParams, PaddingStrategy, TruncationParams, TruncationStrategy,
};

/// Wrapper around a HuggingFace tokenizer that is configured for BERT-style models.
pub struct BertCasedTokenizer {
    inner: Tokenizer,
    pad_id: u32,
    pad_token: String,
}

impl BertCasedTokenizer {
    /// Load a tokenizer from disk or download it from Hugging Face if not available locally.
    pub fn from_dir<P: AsRef<Path>>(dir: P, repo: Option<&str>) -> Result<Self> {
        let dir = dir.as_ref();
        let mut tokenizer = match load_local_tokenizer(dir) {
            Ok(tokenizer) => tokenizer,
            Err(err) => {
                if let Some(repo_id) = repo {
                    download_tokenizer(repo_id, dir)?
                } else {
                    return Err(err);
                }
            }
        };

        // Ensure BERT-like padding is enabled so that batching works deterministically.
        let pad_token = tokenizer
            .get_padding()
            .map(|params| params.pad_token.to_string())
            .filter(|token| !token.is_empty())
            .unwrap_or_else(|| "[PAD]".to_string());

        let pad_id = tokenizer
            .token_to_id(&pad_token)
            .or_else(|| tokenizer.token_to_id("[PAD]"))
            .unwrap_or(0);

        let _ = tokenizer.with_padding(Some(PaddingParams {
            strategy: PaddingStrategy::BatchLongest,
            direction: PaddingDirection::Right,
            pad_id,
            pad_type_id: 0,
            pad_token: pad_token.clone(),
            ..Default::default()
        }));

        Ok(Self {
            inner: tokenizer,
            pad_id,
            pad_token,
        })
    }

    /// Encode a batch of strings using the configured sequence length.
    pub fn encode_batch(&self, texts: &[String], seq_length: usize) -> Result<Vec<Encoding>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let mut tokenizer = self.inner.clone();
        tokenizer
            .with_truncation(Some(TruncationParams {
                max_length: seq_length,
                strategy: TruncationStrategy::LongestFirst,
                ..Default::default()
            }))
            .map_err(|err| anyhow::anyhow!("failed to configure truncation: {err}"))?;
        let _ = tokenizer.with_padding(Some(PaddingParams {
            strategy: PaddingStrategy::Fixed(seq_length),
            direction: PaddingDirection::Right,
            pad_id: self.pad_id,
            pad_type_id: 0,
            pad_token: self.pad_token.clone(),
            ..Default::default()
        }));

        tokenizer
            .encode_batch(
                texts.iter().map(|text| text.as_str()).collect::<Vec<_>>(),
                true,
            )
            .map_err(|err| anyhow::anyhow!("tokenization failed: {err}"))
    }

    /// Number of tokens in the tokenizer vocabulary.
    pub fn vocab_size(&self) -> usize {
        self.inner.get_vocab_size(false)
    }
}

fn resolve_tokenizer_json<P: AsRef<Path>>(dir: P) -> Result<PathBuf> {
    let mut path = dir.as_ref().to_path_buf();
    path.push("tokenizer.json");
    if path.exists() {
        return Ok(path);
    }

    Err(anyhow::anyhow!(
        "tokenizer.json not found in {}",
        dir.as_ref().display()
    ))
}

fn load_local_tokenizer(dir: &Path) -> Result<Tokenizer> {
    let path = resolve_tokenizer_json(dir)?;
    let json = fs::read_to_string(&path)?;
    Tokenizer::from_str(&json).map_err(|err| {
        anyhow::anyhow!(
            "failed to parse tokenizer JSON from {}: {err}",
            path.display()
        )
    })
}

fn download_tokenizer(_repo: &str, _dir: &Path) -> Result<Tokenizer> {
    // Note: from_pretrained removed in newer tokenizers versions
    // We expect tokenizer.json to already exist
    anyhow::bail!("Tokenizer download not supported. Tokenizer files must be present locally.")
}
