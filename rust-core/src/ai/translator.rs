//! Translation service using Claude API
//!
//! Provides real-time translation with natural phrasing.
//! Includes retry logic, timeout handling, and error resilience.

use crate::error::{Result, AiServiceError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Translation request parameters
#[derive(Debug, Clone, Serialize)]
pub struct TranslationRequest {
    /// Text to translate
    pub text: String,
    /// Source language code (e.g., "en", "zh") or "auto" for auto-detection
    pub source_language: String,
    /// Target language code
    pub target_language: String,
    /// Whether to preserve formatting placeholders
    pub preserve_formatting: bool,
    /// Desired tone (e.g., "formal", "casual", "neutral")
    pub tone: String,
}

/// Translation result
#[derive(Debug, Clone, Deserialize)]
pub struct TranslationResult {
    /// Translated text
    pub translated_text: String,
    /// Detected or specified source language
    pub source_language: String,
    /// Target language
    pub target_language: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
}

/// Configuration for translation service
#[derive(Debug, Clone)]
pub struct TranslationConfig {
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}

/// Translation service using Claude API
pub struct TranslationService {
    client: Client,
    api_key: Option<String>,
    config: TranslationConfig,
}

/// Supported language information
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    pub code: String,
    pub name: String,
    pub native_name: String,
}

impl TranslationService {
    /// Create a new translation service with default configuration
    pub fn new() -> Self {
        Self::with_config(TranslationConfig::default())
    }

    /// Create a translation service with custom configuration
    pub fn with_config(config: TranslationConfig) -> Self {
        let api_key = std::env::var("ANTHROPIC_API_KEY").ok();
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client, api_key, config }
    }

    /// Check if the service is available (has API key)
    pub fn is_available(&self) -> bool {
        self.api_key.is_some()
    }

    /// Validate a translation request
    pub fn validate_request(&self, request: &TranslationRequest) -> Result<()> {
        // Validate text length
        if request.text.len() > 10000 {
            return Err(AiServiceError::InvalidRequest(
                "Text exceeds maximum length of 10000 characters".to_string()
            ).into());
        }

        // Validate target language
        if !self.is_supported_pair(&request.source_language, &request.target_language) {
            return Err(AiServiceError::InvalidRequest(
                format!("Unsupported language pair: {} -> {}", request.source_language, request.target_language)
            ).into());
        }

        // Validate tone
        let valid_tones = ["neutral", "formal", "casual", "professional", "friendly"];
        if !valid_tones.contains(&request.tone.as_str()) {
            return Err(AiServiceError::InvalidRequest(
                format!("Invalid tone: {}. Valid tones: {:?}", request.tone, valid_tones)
            ).into());
        }

        Ok(())
    }

    /// Check if a translation pair is supported
    pub fn is_supported_pair(&self, source: &str, target: &str) -> bool {
        let supported = self.get_supported_languages();
        let source_ok = source == "auto" || supported.contains(&source.to_string());
        let target_ok = supported.contains(&target.to_string());
        source_ok && target_ok
    }

    /// Get list of supported languages
    pub fn get_supported_languages(&self) -> Vec<String> {
        vec![
            "en".to_string(), "zh".to_string(), "es".to_string(), "ja".to_string(),
            "de".to_string(), "fr".to_string(), "ko".to_string(), "pt".to_string(),
            "it".to_string(), "ru".to_string(), "ar".to_string(), "hi".to_string(),
            "th".to_string(), "vi".to_string(), "nl".to_string(), "pl".to_string(),
            "sv".to_string(), "tr".to_string(),
        ]
    }

    /// Get language name from code
    pub fn get_language_name(&self, code: &str) -> &str {
        match code {
            "en" => "English", "zh" => "Chinese", "es" => "Spanish", "ja" => "Japanese",
            "de" => "German", "fr" => "French", "ko" => "Korean", "pt" => "Portuguese",
            "it" => "Italian", "ru" => "Russian", "ar" => "Arabic", "hi" => "Hindi",
            "th" => "Thai", "vi" => "Vietnamese", "nl" => "Dutch", "pl" => "Polish",
            "sv" => "Swedish", "tr" => "Turkish", _ => "Unknown",
        }
    }

    /// Translate text with retry logic
    pub async fn translate(&self, request: &TranslationRequest) -> Result<TranslationResult> {
        // Handle empty text
        if request.text.is_empty() {
            return Ok(TranslationResult {
                translated_text: "".to_string(),
                source_language: request.source_language.clone(),
                target_language: request.target_language.clone(),
                confidence: 1.0,
            });
        }

        // Validate request
        self.validate_request(request)?;

        // Attempt translation with retries
        for attempt in 0..=self.config.max_retries {
            match self.translate_once(request).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Don't retry on non-transient errors
                    if !Self::is_retryable_error(&e) {
                        return Err(e);
                    }

                    // Wait before retry (except on last attempt)
                    if attempt < self.config.max_retries {
                        tokio::time::sleep(Duration::from_millis(self.config.retry_delay_ms)).await;
                    }
                }
            }
        }

        Err(AiServiceError::RequestFailed("Translation failed after retries".to_string()).into())
    }

    /// Check if an error is retryable
    fn is_retryable_error(error: &crate::error::Error) -> bool {
        matches!(
            error,
            crate::error::Error::AiService(AiServiceError::RequestFailed(_))
        )
    }

    /// Single translation attempt
    async fn translate_once(&self, request: &TranslationRequest) -> Result<TranslationResult> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| AiServiceError::AuthenticationFailed)?;

        let source_lang = if request.source_language == "auto" {
            "the detected language".to_string()
        } else {
            self.get_language_name(&request.source_language).to_string()
        };
        let target_lang = self.get_language_name(&request.target_language);

        let prompt = self.build_prompt(&source_lang, target_lang, request);

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": "claude-3-haiku-20240307",
                "max_tokens": 4096,
                "messages": [{
                    "role": "user",
                    "content": prompt
                }]
            }))
            .send()
            .await
            .map_err(|e| AiServiceError::RequestFailed(format!("Network error: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(AiServiceError::RequestFailed(
                format!("API error ({}): {}", status, error_body)
            ).into());
        }

        let result: ClaudeResponse = response.json()
            .await
            .map_err(|e| AiServiceError::InvalidResponse(format!("Parse error: {}", e)))?;

        let translated_text = result.content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        Ok(TranslationResult {
            translated_text,
            source_language: request.source_language.clone(),
            target_language: request.target_language.clone(),
            confidence: 0.9,
        })
    }

    /// Build translation prompt
    fn build_prompt(&self, source_lang: &str, target_lang: &str, request: &TranslationRequest) -> String {
        if request.preserve_formatting {
            format!(
                "Translate the following text from {} to {}. Preserve any placeholders like {{name}} or {{place}} exactly as they are. Provide a natural, {} translation.\n\nText: {}\n\nProvide only the translation, no explanation.",
                source_lang, target_lang, request.tone, request.text
            )
        } else {
            format!(
                "Translate the following text from {} to {}. Provide a natural, {} translation.\n\nText: {}\n\nProvide only the translation, no explanation.",
                source_lang, target_lang, request.tone, request.text
            )
        }
    }

    /// Translate multiple texts in batch
    pub async fn translate_batch(
        &self,
        texts: &[String],
        source_language: &str,
        target_language: &str,
    ) -> Result<Vec<String>> {
        let mut results = Vec::with_capacity(texts.len());

        for text in texts {
            let request = TranslationRequest {
                text: text.clone(),
                source_language: source_language.to_string(),
                target_language: target_language.to_string(),
                preserve_formatting: false,
                tone: "neutral".to_string(),
            };

            let result = self.translate(&request).await?;
            results.push(result.translated_text);
        }

        Ok(results)
    }
}

impl Default for TranslationService {
    fn default() -> Self {
        Self::new()
    }
}

/// Claude API response structure
#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ClaudeContent>,
}

#[derive(Debug, Deserialize)]
struct ClaudeContent {
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_languages() {
        let service = TranslationService::new();
        let languages = service.get_supported_languages();

        assert!(languages.contains(&"en".to_string()));
        assert!(languages.contains(&"zh".to_string()));
        assert!(languages.contains(&"ja".to_string()));
    }

    #[test]
    fn test_supported_pairs() {
        let service = TranslationService::new();

        assert!(service.is_supported_pair("en", "zh"));
        assert!(service.is_supported_pair("auto", "en"));
    }

    #[test]
    fn test_language_names() {
        let service = TranslationService::new();

        assert_eq!(service.get_language_name("en"), "English");
        assert_eq!(service.get_language_name("zh"), "Chinese");
        assert_eq!(service.get_language_name("ja"), "Japanese");
    }

    #[test]
    fn test_validate_request_valid() {
        let service = TranslationService::new();
        let request = TranslationRequest {
            text: "Hello".to_string(),
            source_language: "en".to_string(),
            target_language: "zh".to_string(),
            preserve_formatting: false,
            tone: "neutral".to_string(),
        };

        assert!(service.validate_request(&request).is_ok());
    }

    #[test]
    fn test_validate_request_invalid_tone() {
        let service = TranslationService::new();
        let request = TranslationRequest {
            text: "Hello".to_string(),
            source_language: "en".to_string(),
            target_language: "zh".to_string(),
            preserve_formatting: false,
            tone: "invalid_tone".to_string(),
        };

        assert!(service.validate_request(&request).is_err());
    }

    #[test]
    fn test_validate_request_text_too_long() {
        let service = TranslationService::new();
        let request = TranslationRequest {
            text: "a".repeat(10001),
            source_language: "en".to_string(),
            target_language: "zh".to_string(),
            preserve_formatting: false,
            tone: "neutral".to_string(),
        };

        assert!(service.validate_request(&request).is_err());
    }

    #[test]
    fn test_default_config() {
        let config = TranslationConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay_ms, 1000);
    }
}