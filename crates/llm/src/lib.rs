use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::sampling::LlamaSampler;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Backend initialization failed: {0}")]
    BackendError(String),
    #[error("Model load failed: {0}")]
    ModelLoadError(String),
    #[error("Context creation failed: {0}")]
    ContextError(String),
    #[error("Tokenization failed: {0}")]
    TokenizationError(String),
    #[error("Generation failed: {0}")]
    GenerationError(String),
    #[error("Invalid input")]
    InvalidInput,
}

/// Global llama backend (initialized once)
static BACKEND: OnceLock<LlamaBackend> = OnceLock::new();

fn get_backend() -> Result<&'static LlamaBackend, LLMError> {
    if let Some(backend) = BACKEND.get() {
        return Ok(backend);
    }
    
    let backend = LlamaBackend::init()
        .map_err(|e| LLMError::BackendError(format!("{:?}", e)))?;
    
    // Ignore the error if another thread initialized it first
    let _ = BACKEND.set(backend);
    
    BACKEND.get().ok_or(LLMError::BackendError("Failed to get backend".to_string()))
}

pub struct SmolLM2 {
    model: LlamaModel,
    n_ctx: u32,
}

impl SmolLM2 {
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, LLMError> {
        let path = model_path.as_ref();
        if !path.exists() {
            return Err(LLMError::ModelNotFound(path.to_string_lossy().to_string()));
        }

        let backend = get_backend()?;
        
        let model_params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(
            backend,
            path,
            &model_params,
        ).map_err(|e| LLMError::ModelLoadError(format!("{:?}", e)))?;

        Ok(Self { 
            model,
            n_ctx: 512,
        })
    }

    pub fn polish(&self, text: &str) -> Result<String, LLMError> {
        if text.is_empty() {
            return Err(LLMError::InvalidInput);
        }

        // Format prompt for SmolLM2 (ChatML format)
        let prompt = format!(
            "<|im_start|>system\nYou are a helpful assistant that polishes and improves text. Keep the meaning but make it clearer and more natural.\n<|im_end|>\n<|im_start|>user\nPlease polish this text: {}\n<|im_end|>\n<|im_start|>assistant\n",
            text
        );

        let backend = get_backend()?;

        // Create context
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(std::num::NonZeroU32::new(self.n_ctx));
        
        let mut ctx = self.model.new_context(backend, ctx_params)
            .map_err(|e| LLMError::ContextError(format!("{:?}", e)))?;

        // Tokenize prompt
        let tokens = self.model.str_to_token(&prompt, llama_cpp_2::model::AddBos::Always)
            .map_err(|e| LLMError::TokenizationError(format!("{:?}", e)))?;

        // Create batch - allocate enough space for prompt + generation
        let n_tokens = tokens.len();
        let mut batch = llama_cpp_2::llama_batch::LlamaBatch::new(512, 1);
        
        for (i, &token) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            batch.add(token, i as i32, &[0], is_last).map_err(|e| {
                LLMError::GenerationError(format!("Batch add failed: {:?}", e))
            })?;
        }

        // Decode prompt
        ctx.decode(&mut batch)
            .map_err(|e| LLMError::GenerationError(format!("Decode failed: {:?}", e)))?;

        // Create sampler chain
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::temp(0.7),
            LlamaSampler::penalties(64, 1.1, 0.0, 0.0),  // repeat penalty
            LlamaSampler::dist(0),
        ]);

        // Track position for batch
        let mut n_cur = batch.n_tokens() as i32;
        
        // Generate tokens
        let mut generated_tokens = Vec::new();
        let max_tokens = 256;
        let eos_token = self.model.token_eos();
        
        for _ in 0..max_tokens {
            // Sample from the last token's logits
            let token = sampler.sample(&ctx, batch.n_tokens() - 1);
            
            if token == eos_token {
                break;
            }
            
            generated_tokens.push(token);

            // Prepare batch for next token
            batch.clear();
            batch.add(token, n_cur, &[0], true)
                .map_err(|e| LLMError::GenerationError(format!("Batch add failed: {:?}", e)))?;
            
            n_cur += 1;
            
            ctx.decode(&mut batch)
                .map_err(|e| LLMError::GenerationError(format!("Decode failed: {:?}", e)))?;
            
            sampler.accept(token);
        }

        // Convert tokens to string
        let mut result = String::new();
        for token in generated_tokens {
            let bytes = self.model.token_to_piece_bytes(token, 32, true, None)
                .unwrap_or_default();
            if let Ok(s) = String::from_utf8(bytes) {
                result.push_str(&s);
            }
        }

        Ok(result.trim().to_string())
    }

    pub fn model_path(&self) -> &str {
        "loaded"
    }
}

pub trait LLMEngine {
    fn polish(&self, text: &str) -> Result<String, LLMError>;
}

impl LLMEngine for SmolLM2 {
    fn polish(&self, text: &str) -> Result<String, LLMError> {
        self.polish(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_creation() {
        let result = SmolLM2::new("/nonexistent/model.gguf");
        assert!(result.is_err());
    }

    #[test]
    fn test_polish_empty() {
        let model_path = std::path::Path::new("../../models/smollm2-360m-q8.gguf");
        if !model_path.exists() {
            return;
        }
        
        let llm = SmolLM2::new(model_path).unwrap();
        let result = llm.polish("");
        assert!(result.is_err());
    }

    #[test]
    fn test_polish_text() {
        let model_path = std::path::Path::new("../../models/smollm2-360m-q8.gguf");
        if !model_path.exists() {
            return;
        }
        
        let llm = SmolLM2::new(model_path).unwrap();
        let result = llm.polish("Hello world");
        assert!(result.is_ok());
        println!("Polished: {:?}", result);
    }
}