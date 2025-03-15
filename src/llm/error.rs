//! LLM 交互错误模块

use ollama_rs::error::OllamaError;
use thiserror::Error;

/// LLM 操作可能出现的错误
#[derive(Debug, Error)]
pub enum LLMError {
    #[error("Ollama error: {0}")]
    OllamaError(#[from] OllamaError),
}
