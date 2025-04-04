//! DeepSeek LLM client implementation

use crate::llm::{LLMClient, LLMError};
use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

const DEFAULT_TEMPERATURE: f32 = 0.7;
const DEFAULT_TOP_P: f32 = 0.9;
const DEFAULT_MODEL: &str = "deepseek-coder";
const DEFAULT_API_BASE: &str = "https://api.deepseek.com/v1";
const DEFAULT_MAX_TOKENS: u32 = 4096;

/// DeepSeek client configuration
#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for the API
    pub api_base: String,
    /// The model to use
    pub model: String,
    /// The temperature of the model (0.0 to 1.0)
    pub temperature: f32,
    /// The top-p of the model (0.0 to 1.0)
    pub top_p: f32,
    /// Maximum tokens to generate
    pub max_tokens: u32,
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            api_base: DEFAULT_API_BASE.to_string(),
            model: DEFAULT_MODEL.to_string(),
            temperature: DEFAULT_TEMPERATURE,
            top_p: DEFAULT_TOP_P,
            max_tokens: DEFAULT_MAX_TOKENS,
        }
    }
}

/// Request payload for the DeepSeek API
#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
}

/// Message format for the DeepSeek API
#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

/// Response format from the DeepSeek API
#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    id: String,
    object: String,
    created: u64,
    choices: Vec<Choice>,
    usage: Usage,
}

/// Choice in the DeepSeek API response
#[derive(Debug, Deserialize)]
struct Choice {
    index: u32,
    message: Message,
    finish_reason: String,
}

/// Token usage information
#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct DeepSeekClient {
    config: DeepSeekConfig,
    client: Arc<Mutex<reqwest::Client>>,
}

impl DeepSeekClient {
    /// Create a new DeepSeek client with the given API key
    pub async fn new(api_key: impl Into<String>) -> Result<Self, LLMError> {
        let mut config = DeepSeekConfig::default();
        config.api_key = api_key.into();
        Self::with_config(config).await
    }

    /// Create a new DeepSeek client with custom configuration
    pub async fn with_config(config: DeepSeekConfig) -> Result<Self, LLMError> {
        if config.api_key.is_empty() {
            return Err(LLMError::ConfigurationError(
                "DeepSeek API key is required".to_string(),
            ));
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let auth_value = format!("Bearer {}", config.api_key);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).map_err(|e| {
                LLMError::ConfigurationError(format!("Invalid API key format: {}", e))
            })?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| {
                LLMError::ConfigurationError(format!("Failed to build HTTP client: {}", e))
            })?;

        // Verify API credentials and connectivity
        Self::verify_credentials(&client, &config).await?;

        Ok(Self {
            config,
            client: Arc::new(Mutex::new(client)),
        })
    }

    /// Verify API credentials and connectivity
    async fn verify_credentials(
        client: &reqwest::Client,
        config: &DeepSeekConfig,
    ) -> Result<(), LLMError> {
        let url = format!("{}/models", config.api_base);

        let response = client.get(&url).send().await.map_err(|e| {
            LLMError::ConnectionError(format!("Failed to connect to DeepSeek API: {}", e))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response".to_string());

            return Err(LLMError::AuthenticationError(format!(
                "Failed to authenticate with DeepSeek API: {} - {}",
                status, text
            )));
        }

        Ok(())
    }

    /// Create parameters for the DeepSeek API
    fn create_params(&self, prompt: &str) -> DeepSeekRequest {
        DeepSeekRequest {
            model: self.config.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            max_tokens: self.config.max_tokens,
        }
    }
}

#[async_trait]
impl LLMClient for DeepSeekClient {
    async fn generate(&self, prompt: &str, dry_run: bool) -> Result<String, LLMError> {
        if dry_run {
            debug!("Dry run mode enabled, returning prompt");
            return Ok(format!("DRY RUN MODE\n\nPROMPT:\n{}", prompt));
        }

        info!(
            "Generating content with DeepSeek model '{}', temperature={}, top_p={}",
            self.config.model, self.config.temperature, self.config.top_p
        );
        debug!("Prompt: {}", prompt);

        let request = self.create_params(prompt);
        let url = format!("{}/chat/completions", self.config.api_base);

        let client = self.client.lock().await;
        let response = client.post(&url).json(&request).send().await.map_err(|e| {
            LLMError::ConnectionError(format!("Failed to connect to DeepSeek API: {}", e))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response".to_string());

            return Err(LLMError::ApiError(format!(
                "DeepSeek API error: {} - {}",
                status, text
            )));
        }

        let deepseek_response: DeepSeekResponse = response.json().await.map_err(|e| {
            LLMError::DeserializationError(format!("Failed to parse DeepSeek response: {}", e))
        })?;

        if deepseek_response.choices.is_empty() {
            return Err(LLMError::EmptyResponseError(
                "DeepSeek API returned no choices".to_string(),
            ));
        }

        let content = deepseek_response.choices[0].message.content.clone();
        Ok(content)
    }

    fn get_name(&self) -> &str {
        &self.config.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    #[ignore = "Requires DeepSeek API key"]
    async fn test_deepseek_dry_run() {
        dotenv().ok(); // Load .env file if available

        let api_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set");
        let client = DeepSeekClient::new(api_key).await.unwrap();

        let result = client.generate("Test prompt", true).await.unwrap();
        assert!(result.contains("DRY RUN MODE"));
        assert!(result.contains("Test prompt"));
    }

    #[tokio::test]
    #[ignore = "Requires DeepSeek API key and costs API credits"]
    async fn test_deepseek_generate() -> anyhow::Result<()> {
        dotenv().ok(); // Load .env file if available

        let api_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY must be set");
        let config = DeepSeekConfig {
            api_key,
            model: "deepseek-coder".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            ..Default::default()
        };

        let client = DeepSeekClient::with_config(config).await?;

        let result = client
            .generate(
                "Please write a short function to calculate fibonacci numbers in Python. Include a brief comment explaining how it works.",
                false,
            )
            .await?;

        assert!(result.contains("def"));
        assert!(result.contains("fibonacci"));
        Ok(())
    }
}
