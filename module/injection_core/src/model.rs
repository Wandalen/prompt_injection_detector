use anyhow::{bail, Result};
use ort::execution_providers::CUDAExecutionProvider;
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use tokenizers::Tokenizer;
use ndarray::Array2;
use ort::value::Value;
use crate::tokenizer::TokenizedBatch;

pub struct Model {
    pub(crate) model: Session,
    pub(crate) tokenizer: Tokenizer
}

impl Model {
    pub fn new() -> Result<Self> {
        let model = Session::builder()?
            // .with_execution_providers([CUDAExecutionProvider::default()
            //     .with_device_id(0)
            //     .with_prefer_nhwc(true)
            //     .build()])?
            //.with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file("model.onnx")?;
        let tokenizer =
            match Tokenizer::from_file("tokenizer.json"){
                Ok(tokenizer) => tokenizer,
                Err(e)=> bail!(e)
            };
        Ok(Self{
            model,
            tokenizer,
        })
    }
    /// Run inference on a tokenized batch
    pub fn infer(&mut self, batch: &TokenizedBatch) -> Result<Vec<f32>> {
        let batch_size = batch.batch_size();
        let seq_length = batch.sequence_length();

        // Prepare input tensors as ndarrays
        let input_ids = Array2::from_shape_vec(
            (batch_size, seq_length),
            batch.flatten_input_ids(),
        )?;

        let attention_mask = Array2::from_shape_vec(
            (batch_size, seq_length),
            batch.flatten_attention_mask(),
        )?;

        // Create ORT Values from ndarrays
        let input_ids_value = Value::from_array(input_ids)?;
        let attention_mask_value = Value::from_array(attention_mask)?;

        // Run inference
        let outputs = self.model.run(ort::inputs![input_ids_value, attention_mask_value])?;

        // Extract logits (output shape: [batch_size, num_classes])
        let (_, logits_data) = outputs["logits"].try_extract_tensor::<f32>()?;

        Ok(logits_data.to_vec())
    }
    pub fn predict(&mut self, batch: &TokenizedBatch) -> Result<Vec<usize>> {
        let logits = self.infer(batch)?;
        let batch_size = batch.batch_size();
        let num_classes = logits.len() / batch_size;

        let predictions: Vec<usize> = (0..batch_size)
            .map(|i| {
                let start = i * num_classes;
                let end = start + num_classes;
                let sample_logits = &logits[start..end];

                sample_logits
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(idx, _)| idx)
                    .unwrap_or(0)
            })
            .collect();

        Ok(predictions)
    }
}
