//! Text polishing service using Claude API
//!
//! This module provides AI-powered text enhancement through:
//! - Default text polishing (filler removal, grammar correction)
//! - Context-aware tone adaptation
//! - Translation between languages
//!
//! # Error Resilience
//!
//! The service implements:
//! - Automatic retries with exponential backoff
//! - Request timeouts
//! - Graceful degradation on API failures
//!
//! # Example
//!
//! ```ignore
//! use talkute_core::ai::polisher::TextPolisher;
//!
//! let polisher = TextPolisher::new()?;
//! let polished = polisher.polish("I wanted to um schedule a meeting").await?;
//! ```

use crate::ai::client::ClaudeClient;
use crate::ai::prompts::AIPrompts;
use crate::error::{Result, AIError};
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// Polishing intensity levels for controlling AI modification depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PolishingIntensity {
    /// Light: Filler removal only, minimal changes
    Light,
    /// Standard: Grammar correction and logic improvements
    #[default]
    Standard,
    /// Deep: Full rewrite with comprehensive improvements
    Deep,
}

impl PolishingIntensity {
    /// Get the description for this intensity level
    pub fn description(&self) -> &'static str {
        match self {
            Self::Light => "Filler removal only",
            Self::Standard => "Grammar and logic improvements",
            Self::Deep => "Full rewrite with comprehensive improvements",
        }
    }

    /// Get the display name for this intensity level
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Standard => "Standard",
            Self::Deep => "Deep",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "light" => Some(Self::Light),
            "standard" => Some(Self::Standard),
            "deep" => Some(Self::Deep),
            _ => None,
        }
    }
}

/// Configuration for the text polisher.
#[derive(Debug, Clone)]
pub struct PolisherConfig {
    /// Maximum number of retry attempts for failed API calls
    pub max_retries: u32,
    /// Initial delay between retries (doubles each attempt)
    pub retry_delay: Duration,
    /// Maximum timeout for API requests
    pub request_timeout: Duration,
    /// Whether to fall back to original text on failure
    pub fallback_on_error: bool,
    /// Current polishing intensity level
    pub intensity: PolishingIntensity,
}

impl Default for PolisherConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            request_timeout: Duration::from_secs(30),
            fallback_on_error: true,
            intensity: PolishingIntensity::Standard,
        }
    }
}

/// Text polishing service with error resilience.
///
/// Provides AI-powered text enhancement through Claude API.
pub struct TextPolisher {
    client: ClaudeClient,
    prompts: AIPrompts,
    config: PolisherConfig,
}

impl TextPolisher {
    /// Create a new text polisher with default configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the Claude API client cannot be initialized
    /// (e.g., missing API key).
    pub fn new() -> Result<Self> {
        let client = ClaudeClient::new()?;
        let prompts = AIPrompts::new();
        let config = PolisherConfig::default();

        Ok(Self { client, prompts, config })
    }

    /// Create a text polisher with custom configuration.
    pub fn with_config(config: PolisherConfig) -> Result<Self> {
        let client = ClaudeClient::new()?;
        let prompts = AIPrompts::new();

        Ok(Self { client, prompts, config })
    }

    /// Polish text using the default prompt.
    ///
    /// Applies filler removal, grammar correction, and formatting.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails after all retries.
    pub async fn polish(&self, text: &str) -> Result<String> {
        self.polish_with_retry(text, None, None).await
    }

    /// Polish text with a specific intensity level.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to polish
    /// * `intensity` - The polishing intensity level
    pub async fn polish_with_intensity(&self, text: &str, intensity: PolishingIntensity) -> Result<String> {
        self.polish_with_retry(text, None, Some(intensity)).await
    }

    /// Polish text with context-aware tone.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to polish
    /// * `context` - Context identifier (e.g., "email", "chat", "code")
    pub async fn polish_with_context(&self, text: &str, context: &str) -> Result<String> {
        self.polish_with_retry(text, Some(context), None).await
    }

    /// Polish text with context and intensity.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to polish
    /// * `context` - Context identifier (e.g., "email", "chat", "code")
    /// * `intensity` - The polishing intensity level
    pub async fn polish_with_context_and_intensity(
        &self,
        text: &str,
        context: &str,
        intensity: PolishingIntensity,
    ) -> Result<String> {
        self.polish_with_retry(text, Some(context), Some(intensity)).await
    }

    /// Set the polishing intensity level.
    pub fn set_intensity(&mut self, intensity: PolishingIntensity) {
        self.config.intensity = intensity;
    }

    /// Get the current polishing intensity level.
    pub fn intensity(&self) -> PolishingIntensity {
        self.config.intensity
    }

    /// Translate text to target language.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to translate
    /// * `target_language` - Target language name (e.g., "Spanish", "Chinese")
    pub async fn translate(&self, text: &str, target_language: &str) -> Result<String> {
        let system_prompt = self.prompts.translation(target_language);
        let user_message = self.prompts.format(&system_prompt, text);

        self.call_with_retry(&system_prompt, &user_message).await
    }

    /// Internal method to polish with retry logic.
    async fn polish_with_retry(
        &self,
        text: &str,
        context: Option<&str>,
        intensity: Option<PolishingIntensity>,
    ) -> Result<String> {
        let effective_intensity = intensity.unwrap_or(self.config.intensity);

        let system_prompt = match context {
            Some(ctx) => self.prompts.for_context(ctx),
            None => self.prompts.default_polishing(),
        };

        // Adjust prompt based on intensity
        let intensity_prompt = match effective_intensity {
            PolishingIntensity::Light => self.prompts.light_polishing(),
            PolishingIntensity::Standard => system_prompt.clone(),
            PolishingIntensity::Deep => self.prompts.deep_polishing(),
        };

        let user_message = self.prompts.format(&intensity_prompt, text);

        match self.call_with_retry(&intensity_prompt, &user_message).await {
            Ok(result) => Ok(result),
            Err(e) if self.config.fallback_on_error => {
                // Log the error but return the original text
                log::warn!("Polishing failed, returning original text: {}", e);
                Ok(text.to_string())
            }
            Err(e) => Err(e),
        }
    }

    /// Call the API with exponential backoff retry logic.
    async fn call_with_retry(&self, system_prompt: &str, user_message: &str) -> Result<String> {
        let mut last_error = None;
        let mut delay = self.config.retry_delay;

        for attempt in 0..=self.config.max_retries {
            match self.client.complete(system_prompt, user_message).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    let is_retryable = self.is_retryable_error(&e);

                    if !is_retryable || attempt == self.config.max_retries {
                        return Err(e);
                    }

                    log::warn!(
                        "API call failed (attempt {}/{}), retrying in {:?}: {}",
                        attempt + 1,
                        self.config.max_retries,
                        delay,
                        e
                    );

                    last_error = Some(e);
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AIError::RequestFailed("Max retries exceeded".to_string()).into()))
    }

    /// Determine if an error is retryable.
    fn is_retryable_error(&self, error: &crate::error::Error) -> bool {
        match error {
            crate::error::Error::AiService(ai_error) => {
                matches!(
                    ai_error,
                    AIError::RequestFailed(_) | AIError::Timeout
                )
            }
            _ => false,
        }
    }

    /// Get the current configuration.
    pub fn config(&self) -> &PolisherConfig {
        &self.config
    }

    /// Check if the service is properly configured.
    pub fn is_configured(&self) -> bool {
        // The client would fail to create if not configured
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polisher_creation() {
        // Without API key, this should fail
        std::env::remove_var("ANTHROPIC_API_KEY");
        let result = TextPolisher::new();
        assert!(result.is_err());
    }

    #[test]
    fn test_default_config() {
        let config = PolisherConfig::default();

        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay, Duration::from_millis(500));
        assert!(config.fallback_on_error);
    }

    #[test]
    fn test_custom_config() {
        let config = PolisherConfig {
            max_retries: 5,
            retry_delay: Duration::from_secs(1),
            request_timeout: Duration::from_secs(60),
            fallback_on_error: false,
            intensity: PolishingIntensity::Deep,
        };

        assert_eq!(config.max_retries, 5);
        assert!(!config.fallback_on_error);
        assert_eq!(config.intensity, PolishingIntensity::Deep);
    }
}