use anyhow::{Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::phi3::{Config as Phi3Config, Model as Phi3};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

pub struct LocalLlm {
    model: Phi3,
    tokenizer: Tokenizer,
    device: Device,
}

impl LocalLlm {
    pub fn new() -> Result<Self> {
        let device = Device::Cpu; // Use Metal if available on Mac, but start with CPU for safety
        // TODO: Add Metal support detection
        
        let api = Api::new()?;
        let repo = api.repo(Repo::new("microsoft/Phi-3.5-mini-instruct".to_string(), RepoType::Model));

        let tokenizer_filename = repo.get("tokenizer.json")?;
        let config_filename = repo.get("config.json")?;
        let model_filenames = vec![
            repo.get("model-00001-of-00002.safetensors")?,
            repo.get("model-00002-of-00002.safetensors")?,
        ];

        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
        let config: Phi3Config = serde_json::from_slice(&std::fs::read(config_filename)?)?;
        
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&model_filenames, DType::F32, &device)? };
        let model = Phi3::new(&config, vb)?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> Result<String> {
        let tokens = self.tokenizer.encode(prompt, true).map_err(E::msg)?;
        let mut tokens = tokens.get_ids().to_vec();
        let mut generated_tokens = Vec::new();
        let mut logits_processor = LogitsProcessor::new(299792458, Some(0.7), Some(0.9)); // Seed, Temp, TopP

        for _ in 0..max_tokens {
            let input = Tensor::new(&tokens[tokens.len().saturating_sub(2048)..], &self.device)?.unsqueeze(0)?;
            let logits = self.model.forward(&input)?;
            let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
            
            let next_token = logits_processor.sample(&logits)?;
            tokens.push(next_token);
            generated_tokens.push(next_token);

            if next_token == self.tokenizer.token_to_id("<|endoftext|>").unwrap_or(32000) {
                break;
            }
        }

        let decoded = self.tokenizer.decode(&generated_tokens, true).map_err(E::msg)?;
        Ok(decoded)
    }
}
