//! DeepSeek LLM client implementation (using async-openai)

use crate::llm::{LLMClient, LLMError};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info};

const DEFAULT_MODEL: &str = "deepseek-chat";
const DEFAULT_API_BASE: &str = "https://api.deepseek.com/v1";
const DEFAULT_MAX_TOKENS: u32 = 4096;

/// DeepSeek client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekConfig {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for the API
    pub api_base: String,
    /// The model to use
    pub model: String,
    /// The temperature of the model (0.0 to 1.0)
    pub temperature: Option<f32>,
    /// The top-p of the model (0.0 to 1.0)
    pub top_p: Option<f32>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            api_base: DEFAULT_API_BASE.to_string(),
            model: DEFAULT_MODEL.to_string(),
            temperature: None,
            top_p: None,
            max_tokens: Some(DEFAULT_MAX_TOKENS),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeepSeekClient {
    config: DeepSeekConfig,
    client: Arc<Client<OpenAIConfig>>,
}

impl DeepSeekClient {
    /// Create a new DeepSeek client with the given API key
    pub async fn new(api_key: impl Into<String>) -> Result<Self, LLMError> {
        let config = DeepSeekConfig {
            api_key: api_key.into(),
            ..Default::default()
        };
        Self::with_config(config).await
    }

    /// Create a new DeepSeek client with custom configuration
    pub async fn with_config(config: DeepSeekConfig) -> Result<Self, LLMError> {
        if config.api_key.is_empty() {
            return Err(LLMError::DeepSeekError(
                "DeepSeek API key is required".to_string(),
            ));
        }

        let openai_config = OpenAIConfig::new()
            .with_api_key(&config.api_key)
            .with_api_base(&config.api_base);

        let client = Client::with_config(openai_config);

        Ok(Self {
            config,
            client: Arc::new(client),
        })
    }
}

#[async_trait]
impl LLMClient for DeepSeekClient {
    async fn generate(&self, prompt: &str, dry_run: bool) -> Result<String, LLMError> {
        const SYSTEM_PROMPT: &str =
            "You are a helpful assistant to summarize the content of the user's message.";

        if dry_run {
            debug!("Dry run mode enabled, returning prompt");
            return Ok(format!("DRY RUN MODE\n\nPROMPT:\n{}", prompt));
        }

        info!(
            "Generating content with DeepSeek model '{}', temperature={:?}, top_p={:?}",
            self.config.model, self.config.temperature, self.config.top_p
        );
        debug!("Prompt: {}", prompt);

        let request = CreateChatCompletionRequest {
            model: self.config.model.clone(),
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            max_completion_tokens: self.config.max_tokens,
            messages: vec![
                ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                    content: ChatCompletionRequestSystemMessageContent::Text(
                        SYSTEM_PROMPT.to_string(),
                    ),
                    ..Default::default()
                }),
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    content: ChatCompletionRequestUserMessageContent::Text(prompt.to_string()),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };

        // 发送请求并处理响应
        match self.client.chat().create(request).await {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.message.content {
                        return Ok(content.clone());
                    }
                }
                Err(LLMError::DeepSeekError(
                    "DeepSeek API returned empty response".to_string(),
                ))
            }
            Err(e) => Err(LLMError::DeepSeekError(format!(
                "DeepSeek API error: {}",
                e
            ))),
        }
    }

    fn get_name(&self) -> &str {
        &self.config.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    #[ignore = "Requires DeepSeek API key"]
    async fn test_deepseek_dry_run() {
        let api_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set");
        let client = DeepSeekClient::new(api_key).await.unwrap();

        let result = client.generate("Test prompt", true).await.unwrap();
        assert!(result.contains("DRY RUN MODE"));
        assert!(result.contains("Test prompt"));
    }

    #[tokio::test]
    #[ignore = "Requires DeepSeek API key and costs API credits"]
    async fn test_deepseek_generate() -> anyhow::Result<()> {
        let api_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set");
        let config = DeepSeekConfig {
            api_key,
            model: "deepseek-chat".to_string(),
            temperature: Some(0.7),
            top_p: Some(0.9),
            max_tokens: Some(1000),
            ..Default::default()
        };

        let client = DeepSeekClient::with_config(config).await?;

        let result = client
            .generate(
                "Please write a short function to calculate fibonacci numbers in Python. Include a brief comment explaining how it works.",
                false,
            )
            .await?;

        println!("result: {}", result);
        assert!(result.contains("def"));
        assert!(result.contains("fibonacci"));
        Ok(())
    }
}
