//! LLM 交互模块，封装各种大语言模型的访问接口

mod error;
mod ollama;
mod prompt;

pub use error::LLMError;
pub use ollama::{OllamaClient, OllamaConfig};
pub use prompt::PromptBuilder;

use async_trait::async_trait;

/// LLM client interface, defines common methods for interacting with large language models
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// Generate content
    ///
    /// # Parameters
    ///
    /// * `prompt` - input prompt
    /// * `dry_run` - whether to run in dry run mode. In dry run mode, only return the processed prompt, not the actual LLM request
    ///
    /// # Returns
    ///
    /// Returns the generated content, or an error if it fails
    async fn generate(&self, prompt: &str, dry_run: bool) -> Result<String, LLMError>;

    /// Get the client type name
    fn get_name(&self) -> &str;
}
