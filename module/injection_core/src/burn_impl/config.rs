use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct ExperimentConfig {
    #[serde(default = "ExperimentConfig::default_seq_length")]
    pub seq_length: usize,
    #[serde(default)]
    pub transformer: TransformerConfig,
    #[serde(default)]
    pub tokenizer: TokenizerConfig,
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub id2label: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct TransformerConfig {
    #[serde(default = "TransformerConfig::default_weights")]
    pub weights: String,
}

impl Default for TransformerConfig {
    fn default() -> Self {
        Self {
            weights: Self::default_weights(),
        }
    }
}

impl TransformerConfig {
    fn default_weights() -> String {
        "model.mpk".to_string()
    }

    pub fn resolve_path<P: AsRef<Path>>(&self, artifact_dir: P) -> PathBuf {
        let mut path = artifact_dir.as_ref().to_path_buf();
        path.push(&self.weights);
        path
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenizerConfig {
    #[serde(default = "TokenizerConfig::default_dir")]
    pub dir: String,
    #[serde(default = "TokenizerConfig::default_repo")]
    pub repo: Option<String>,
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            dir: Self::default_dir(),
            repo: Self::default_repo(),
        }
    }
}

impl TokenizerConfig {
    fn default_dir() -> String {
        ".".to_string()
    }

    fn default_repo() -> Option<String> {
        Some("answerdotai/ModernBERT-base".to_string())
    }

    pub fn resolve_path<P: AsRef<Path>>(&self, artifact_dir: P) -> PathBuf {
        let mut path = artifact_dir.as_ref().to_path_buf();
        path.push(&self.dir);
        path
    }

    pub fn repo(&self) -> Option<&str> {
        self.repo.as_deref()
    }
}

impl ExperimentConfig {
    fn default_seq_length() -> usize {
        128
    }

    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        let mut config: ExperimentConfig = serde_json::from_str(&data)?;
        if config.seq_length == 0 {
            config.seq_length = Self::default_seq_length();
        }
        Ok(config)
    }

    /// Resolve class labels from the config. When the explicit `classes` field is empty we try to
    /// fall back to `id2label` from HuggingFace-style configs.
    pub fn class_labels(&self, num_classes: usize) -> Vec<String> {
        if !self.classes.is_empty() {
            return self.classes.clone();
        }

        if !self.id2label.is_empty() {
            let mut labels: Vec<(usize, String)> = self
                .id2label
                .iter()
                .filter_map(|(idx, label)| idx.parse::<usize>().ok().map(|i| (i, label.clone())))
                .collect();
            labels.sort_by_key(|(idx, _)| *idx);
            return labels.into_iter().map(|(_, label)| label).collect();
        }

        (0..num_classes).map(|idx| format!("class_{idx}")).collect()
    }
}
