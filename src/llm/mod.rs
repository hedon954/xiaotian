//! LLM 交互模块，封装各种大语言模型的访问接口

mod error;
mod ollama;
mod prompt;

pub use error::LLMError;
pub use ollama::{OllamaClient, OllamaConfig};
pub use prompt::PromptBuilder;

use async_trait::async_trait;

/// LLM 客户端接口，定义了与大语言模型交互的通用方法
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// 生成内容
    ///
    /// # 参数
    ///
    /// * `prompt` - 输入提示词
    /// * `dry_run` - 是否为干运行模式。在干运行模式下，只返回处理后的提示词，不实际请求 LLM
    ///
    /// # 返回
    ///
    /// 成功时返回生成的内容，失败时返回错误
    async fn generate(&self, prompt: &str, dry_run: bool) -> Result<String, LLMError>;

    /// 获取客户端类型名称
    fn get_name(&self) -> &str;
}
