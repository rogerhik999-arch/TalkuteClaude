//! Claude API client for text polishing

use crate::error::{Result, AIError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// Claude API configuration
#[derive(Debug, Clone)]
pub struct ClaudeConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl ClaudeConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| AIError::AuthenticationFailed)?;
        let model = env::var("CLAUDE_MODEL")
            .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());

        Ok(Self {
            api_key,
            model,
            max_tokens: 4096,
            temperature: 0.3,
        })
    }
}

/// Claude API client
pub struct ClaudeClient {
    client: Client,
    config: ClaudeConfig,
}

impl ClaudeClient {
    /// Create a new Claude client
    pub fn new() -> Result<Self> {
        let config = ClaudeConfig::from_env()?;
        let client = Client::new();

        Ok(Self { client, config })
    }

    /// Send a completion request to Claude
    pub async fn complete(&self, system_prompt: &str, user_message: &str) -> Result<String> {
        let endpoint = "https://api.anthropic.com/v1/messages";

        let request = ClaudeRequest {
            model: &self.config.model,
            max_tokens: self.config.max_tokens,
            system: system_prompt,
            messages: vec![ClaudeMessage {
                role: "user",
                content: user_message,
            }],
        };

        let response = self.client
            .post(endpoint)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AIError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AIError::RequestFailed(
                format!("API request failed: {} - {}", status, body)
            ).into());
        }

        let result: ClaudeResponse = response.json()
            .await
            .map_err(|e| AIError::InvalidResponse(format!("Failed to parse response: {}", e)))?;

        // Extract text from the first content block
        if let Some(block) = result.content.first() {
            if block.block_type == "text" {
                return Ok(block.text.clone());
            }
        }

        Err(AIError::InvalidResponse("No text content in response".to_string()).into())
    }
}

#[derive(Serialize)]
struct ClaudeRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    system: &'a str,
    messages: Vec<ClaudeMessage<'a>>,
}

#[derive(Serialize)]
struct ClaudeMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContentBlock>,
}

#[derive(Deserialize)]
struct ClaudeContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = ClaudeConfig {
            api_key: "test-key".to_string(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 4096,
            temperature: 0.3,
        };

        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.model, "claude-3-5-sonnet-20241022");
    }
}
