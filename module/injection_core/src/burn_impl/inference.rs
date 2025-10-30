use super::config::ExperimentConfig;
use super::data::{TextBatch, TextClassificationBatcher};
use super::model::DebertaModel;
use anyhow::{Context, Result};
use burn::tensor::backend::Backend;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Prediction {
    pub text: String,
    pub logits: Vec<f32>,
    pub class_index: usize,
    pub class_name: String,
}

pub fn infer<B: Backend>(
    device: &B::Device,
    artifact_dir: impl AsRef<Path>,
    samples: Vec<String>,
) -> Result<Vec<Prediction>>
where
    B::Device: Clone,
{
    let ctx = InferenceContext::<B>::new(device.clone(), artifact_dir)?;
    ctx.infer(samples)
}

pub struct InferenceContext<B: Backend>
where
    B::Device: Clone,
{
    model: DebertaModel<B>,
    batcher: TextClassificationBatcher,
    config: Arc<ExperimentConfig>,
    device: B::Device,
}

impl<B: Backend> InferenceContext<B>
where
    B::Device: Clone,
{
    pub fn new(device: B::Device, artifact_dir: impl AsRef<Path>) -> Result<Self> {
        let artifact_dir = artifact_dir.as_ref();
        let config_path = artifact_dir.join("config.json");
        let config = Arc::new(
            ExperimentConfig::load(&config_path)
                .with_context(|| format!("failed to load config at {}", config_path.display()))?,
        );

        let tokenizer_dir = config.tokenizer.resolve_path(artifact_dir);
        let tokenizer = Arc::new(super::data::BertCasedTokenizer::from_dir(
            &tokenizer_dir,
            config.tokenizer.repo(),
        )?);
        let batcher = TextClassificationBatcher::new(tokenizer, config.seq_length);

        let weights_path = config.transformer.resolve_path(artifact_dir);
        let weights_str = weights_path
            .to_str()
            .context("weights path contains invalid UTF-8")?;
        let model = DebertaModel::<B>::from_file(weights_str, &device);

        Ok(Self {
            model,
            batcher,
            config,
            device,
        })
    }

    pub fn infer(&self, samples: Vec<String>) -> Result<Vec<Prediction>> {
        if samples.is_empty() {
            return Ok(Vec::new());
        }

        let batch = self.batcher.batch(samples.clone(), &self.device)?;
        let logits = self.model.forward(batch.input_ids, batch.attention_mask);
        let logits_data = logits.into_data();
        let shape = logits_data.shape.clone();

        if shape.len() != 2 {
            anyhow::bail!("unexpected logits shape: {:?}", shape);
        }
        let batch_size = shape[0];
        let num_classes = shape[1];

        let values = logits_data
            .into_vec::<f32>()
            .map_err(|err| anyhow::anyhow!("failed to convert logits: {:?}", err))?;
        if values.len() != batch_size * num_classes {
            anyhow::bail!(
                "logits length mismatch (values={}, expected={})",
                values.len(),
                batch_size * num_classes
            );
        }

        let class_names = self.config.class_labels(num_classes);

        let mut predictions = Vec::with_capacity(samples.len());
        for (row, text) in samples.into_iter().enumerate() {
            let start = row * num_classes;
            let end = start + num_classes;
            let logits_slice = &values[start..end];
            let (class_index, _score) = logits_slice
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or((0, &0.0));
            let class_name = class_names
                .get(class_index)
                .cloned()
                .unwrap_or_else(|| format!("class_{class_index}"));
            predictions.push(Prediction {
                text,
                logits: logits_slice.to_vec(),
                class_index,
                class_name,
            });
        }

        Ok(predictions)
    }

    pub fn batch_inputs(&self, samples: Vec<String>) -> Result<TextBatch<B>> {
        self.batcher.batch(samples, &self.device)
    }

    pub fn config(&self) -> Arc<ExperimentConfig> {
        Arc::clone(&self.config)
    }
}
