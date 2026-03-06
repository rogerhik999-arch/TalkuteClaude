//! Translation service using Claude API
//!
//! Provides real-time translation with natural phrasing.

use crate::error::{Result, AiServiceError};
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

/// Translation service using Claude API
pub struct TranslationService {
    client: Client,
    api_key: Option<String>,
}

/// Supported language information
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    pub code: String,
    pub name: String,
    pub native_name: String,
}

impl TranslationService {
    /// Create a new translation service
    pub fn new() -> Self {
        let api_key = std::env::var("ANTHROPIC_API_KEY").ok();
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Check if the service is available (has API key)
    pub fn is_available(&self) -> bool {
        self.api_key.is_some()
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
            "en".to_string(),    // English
            "zh".to_string(),    // Chinese
            "es".to_string(),    // Spanish
            "ja".to_string(),    // Japanese
            "de".to_string(),    // German
            "fr".to_string(),    // French
            "ko".to_string(),    // Korean
            "pt".to_string(),    // Portuguese
            "it".to_string(),    // Italian
            "ru".to_string(),    // Russian
            "ar".to_string(),    // Arabic
            "hi".to_string(),    // Hindi
            "th".to_string(),    // Thai
            "vi".to_string(),    // Vietnamese
            "nl".to_string(),    // Dutch
            "pl".to_string(),    // Polish
            "sv".to_string(),    // Swedish
            "tr".to_string(),    // Turkish
        ]
    }

    /// Get language name from code
    pub fn get_language_name(&self, code: &str) -> &str {
        match code {
            "en" => "English",
            "zh" => "Chinese",
            "es" => "Spanish",
            "ja" => "Japanese",
            "de" => "German",
            "fr" => "French",
            "ko" => "Korean",
            "pt" => "Portuguese",
            "it" => "Italian",
            "ru" => "Russian",
            "ar" => "Arabic",
            "hi" => "Hindi",
            "th" => "Thai",
            "vi" => "Vietnamese",
            "nl" => "Dutch",
            "pl" => "Polish",
            "sv" => "Swedish",
            "tr" => "Turkish",
            _ => "Unknown",
        }
    }

    /// Translate text
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

        // Check API availability
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| AiServiceError::AuthenticationFailed)?;

        // Build prompt for translation
        let source_lang = if request.source_language == "auto" {
            "the detected language".to_string()
        } else {
            self.get_language_name(&request.source_language).to_string()
        };
        let target_lang = self.get_language_name(&request.target_language);

        let prompt = if request.preserve_formatting {
            format!(
                "Translate the following text from {} to {}. Preserve any placeholders like {{name}} or {{place}} exactly as they are. Provide a natural, {} translation.\n\nText: {}\n\nProvide only the translation, no explanation.",
                source_lang, target_lang, request.tone, request.text
            )
        } else {
            format!(
                "Translate the following text from {} to {}. Provide a natural, {} translation.\n\nText: {}\n\nProvide only the translation, no explanation.",
                source_lang, target_lang, request.tone, request.text
            )
        };

        // Call Claude API
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
            .map_err(|e| AiServiceError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AiServiceError::RequestFailed(
                format!("Translation failed: {}", response.status())
            ).into());
        }

        let result: ClaudeResponse = response.json()
            .await
            .map_err(|_e| AiServiceError::InvalidResponse)?;

        // Extract translated text
        let translated_text = result.content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_default();

        Ok(TranslationResult {
            translated_text,
            source_language: request.source_language.clone(),
            target_language: request.target_language.clone(),
            confidence: 0.9, // Default confidence
        })
    }

    /// Translate multiple texts in batch
    pub async fn translate_batch(
        &self,
        texts: &[String],
        source_language: &str,
        target_language: &str,
    ) -> Result<Vec<String>> {
        let mut results = Vec::new();

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
}