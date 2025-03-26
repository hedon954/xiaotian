//! Ollama LLM 客户端实现

use crate::llm::{LLMClient, LLMError};
use async_trait::async_trait;
use ollama_rs::{
    Ollama,
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

const DEFAULT_TEMPERATURE: f32 = 0.7;
const DEFAULT_TOP_P: f32 = 0.9;
const DEFAULT_MODEL: &str = "llama3";
const DEFAULT_HOST: &str = "http://localhost";
const DEFAULT_PORT: u16 = 11434;

/// Ollama 客户端配置
#[derive(Debug, Clone)]
pub struct OllamaConfig {
    /// the host of the ollama server
    pub host: String,
    /// the port of the ollama server
    pub port: u16,
    /// the model to use
    pub model: String,
    /// the temperature of the model
    pub temperature: f32,
    /// the top-p of the model
    pub top_p: f32,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            model: DEFAULT_MODEL.to_string(),
            temperature: DEFAULT_TEMPERATURE,
            top_p: DEFAULT_TOP_P,
        }
    }
}

/// Ollama LLM 客户端
#[derive(Debug, Clone)]
pub struct OllamaClient {
    config: OllamaConfig,
    client: Arc<Mutex<Ollama>>,
}

impl OllamaClient {
    pub async fn new() -> Result<Self, LLMError> {
        Self::with_config(OllamaConfig::default()).await
    }

    pub async fn with_config(config: OllamaConfig) -> Result<Self, LLMError> {
        let client = Ollama::new(config.host.clone(), config.port);
        let _ = client.list_local_models().await?;
        Ok(Self {
            config,
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn with_params(
        host: impl Into<String>,
        port: impl Into<u16>,
        model: impl Into<String>,
        temperature: f32,
        top_p: f32,
    ) -> Result<Self, LLMError> {
        let config = OllamaConfig {
            host: host.into(),
            port: port.into(),
            model: model.into(),
            temperature,
            top_p,
        };
        Self::with_config(config).await
    }
}

#[async_trait]
impl LLMClient for OllamaClient {
    async fn generate(&self, prompt: &str, dry_run: bool) -> Result<String, LLMError> {
        if dry_run {
            debug!("Dry run mode enabled, returning prompt");
            return Ok(format!("DRY RUN MODE\n\nPROMPT:\n{}", prompt));
        }

        info!(
            "Generating content with Ollama model '{}', temperature={}, top_p={}",
            self.config.model, self.config.temperature, self.config.top_p
        );
        debug!("Prompt: {}", prompt);

        let options = GenerationOptions::default()
            .temperature(self.config.temperature)
            .top_p(self.config.top_p);

        let request =
            GenerationRequest::new(self.config.model.clone(), prompt.to_string()).options(options);

        let client = self.client.lock().await;
        let response = client.generate(request).await?;

        Ok(response.response)
    }

    fn get_name(&self) -> &str {
        &self.config.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "This test is slow and requires local ollama server"]
    async fn test_dry_run() {
        let client = OllamaClient::with_config(OllamaConfig {
            host: "http://localhost".to_string(),
            port: 11434,
            model: "llama3".to_string(),
            temperature: 0.7,
            top_p: 0.9,
        })
        .await;

        if let Ok(client) = client {
            let result = client.generate("Test prompt", true).await.unwrap();
            assert!(result.contains("DRY RUN MODE"));
            assert!(result.contains("Test prompt"));
        }
    }

    #[tokio::test]
    #[ignore = "This test is slow and requires local ollama server"]
    async fn test_generate() -> anyhow::Result<()> {
        let client = OllamaClient::with_config(OllamaConfig {
            host: "http://localhost".to_string(),
            port: 11434,
            model: "llama3.2".to_string(),
            temperature: 0.7,
            top_p: 0.9,
        })
        .await?;

        let result = client
            .generate(
                "Please write a poem about a cat, start with 'Once upon a time'",
                false,
            )
            .await
            .unwrap();
        assert!(result.contains("Once upon a time"));
        Ok(())
    }
}
