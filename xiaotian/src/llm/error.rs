//! LLM 交互错误模块

use ollama_rs::error::OllamaError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LLMError {
    #[error("Ollama error: {0}")]
    OllamaError(#[from] OllamaError),
}
