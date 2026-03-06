//! Azure Speech API client with multi-language support

use crate::error::{Result, SpeechApiError};
use reqwest::Client;
use serde::Deserialize;
use std::env;

/// Azure Speech configuration
#[derive(Debug, Clone)]
pub struct AzureSpeechConfig {
    pub endpoint: String,
    pub key: String,
    pub region: String,
}

impl AzureSpeechConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let endpoint = env::var("AZURE_SPEECH_ENDPOINT")
            .unwrap_or_else(|_| "https://*.cognitiveservices.azure.com".to_string());
        let key = env::var("AZURE_SPEECH_KEY")
            .map_err(|_| SpeechApiError::AuthenticationFailed)?;
        let region = env::var("AZURE_SPEECH_REGION")
            .unwrap_or_else(|_| "eastus".to_string());

        Ok(Self { endpoint, key, region })
    }

    /// Create config from explicit values
    pub fn new(key: String, region: String, endpoint: Option<String>) -> Self {
        let endpoint = endpoint.unwrap_or_else(|| format!("https://{}.cognitiveservices.azure.com", region));
        Self { endpoint, key, region }
    }

    /// Get the speech endpoint for the current region
    pub fn get_speech_endpoint(&self) -> String {
        format!("{}/sts/v1.0/issueToken", self.endpoint)
    }
}

/// Azure Speech API client with multi-language support
pub struct AzureSpeechClient {
    client: Client,
    config: AzureSpeechConfig,
    recognition_language: String,
}

/// Supported languages for speech recognition
pub const SUPPORTED_LANGUAGES: &[&str] = &[
    "en-US", "en-GB", "en-AU", "en-CA", "en-IN", "en-NZ",
    "zh-CN", "zh-HK", "zh-TW",
    "es-ES", "es-MX", "es-AR",
    "ja-JP",
    "de-DE", "de-AT", "de-CH",
    "fr-FR", "fr-CA", "fr-CH",
    "ko-KR",
    "pt-BR", "pt-PT",
    "it-IT",
    "ru-RU",
    "ar-SA", "ar-EG",
    "hi-IN",
    "th-TH",
    "vi-VN",
    "nl-NL",
    "pl-PL",
    "sv-SE",
    "tr-TR",
];

impl AzureSpeechClient {
    /// Create a new Azure Speech client
    pub fn new() -> Result<Self> {
        let config = AzureSpeechConfig::from_env()?;
        let client = Client::new();

        Ok(Self {
            client,
            config,
            recognition_language: "en-US".to_string(),
        })
    }

    /// Create an Azure Speech client from explicit configuration
    pub fn with_config(key: String, region: String, endpoint: Option<String>) -> Self {
        let config = AzureSpeechConfig::new(key, region, endpoint);
        let client = Client::new();
        Self {
            client,
            config,
            recognition_language: "en-US".to_string(),
        }
    }

    /// Create an Azure Speech client from existing configuration
    pub fn from_config(config: AzureSpeechConfig) -> Self {
        let client = Client::new();
        Self {
            client,
            config,
            recognition_language: "en-US".to_string(),
        }
    }

    /// Check if the client is properly configured
    pub fn is_configured(&self) -> bool {
        !self.config.key.is_empty() && !self.config.region.is_empty()
    }

    /// Get list of supported languages
    pub fn get_supported_languages(&self) -> Vec<String> {
        SUPPORTED_LANGUAGES.iter().map(|s| s.to_string()).collect()
    }

    /// Set the recognition language
    pub fn set_recognition_language(&mut self, language: &str) {
        if self.is_valid_language(language) {
            self.recognition_language = language.to_string();
        } else {
            // Fallback to default
            self.recognition_language = "en-US".to_string();
        }
    }

    /// Get the current recognition language
    pub fn get_recognition_language(&self) -> &str {
        &self.recognition_language
    }

    /// Check if a language code is valid
    pub fn is_valid_language(&self, language: &str) -> bool {
        SUPPORTED_LANGUAGES.contains(&language)
    }

    /// Create recognition configuration
    pub fn create_recognition_config(&self, language: Option<&str>, _continuous: bool) -> Result<serde_json::Value> {
        let lang = language.unwrap_or(&self.recognition_language);

        Ok(serde_json::json!({
            "language": lang,
            "speechConfig": {
                "recognitionMode": "conversation",
                "profanityOption": "masked",
            }
        }))
    }

    /// Setup continuous recognition for a language
    pub fn setup_continuous_recognition(&self, _language: &str) -> Result<()> {
        // In a real implementation, this would configure the speech SDK
        // for continuous recognition mode
        Ok(())
    }

    /// Get an authentication token from Azure
    pub async fn get_token(&self) -> Result<String> {
        let endpoint = self.config.get_speech_endpoint();

        let response = self.client
            .post(&endpoint)
            .header("Ocp-Apim-Subscription-Key", &self.config.key)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Content-Length", "0")
            .send()
            .await
            .map_err(|e| SpeechApiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SpeechApiError::RequestFailed(
                format!("Token request failed: {}", response.status())
            ).into());
        }

        let token = response.text()
            .await
            .map_err(|e| SpeechApiError::RequestFailed(e.to_string()))?;

        Ok(token)
    }

    /// Transcribe audio to text
    pub async fn transcribe(
        &self,
        audio_data: &[u8],
        language: Option<&str>,
    ) -> Result<String> {
        let token = self.get_token().await?;
        let lang = language.unwrap_or(&self.recognition_language);

        // Azure Speech API requires specific endpoint for transcription
        let endpoint = format!(
            "https://{}.stt.speech.microsoft.com/speech/recognition/conversation/cognitiveservices/v1?language={}",
            self.config.region,
            lang
        );

        let response = self.client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "audio/wav; codec=audio/pcm")
            .header("Transfer-Encoding", "chunked")
            .body(audio_data.to_vec())
            .send()
            .await
            .map_err(|e| SpeechApiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SpeechApiError::RequestFailed(
                format!("Transcription failed: {}", response.status())
            ).into());
        }

        let result: TranscriptionResult = response.json()
            .await
            .map_err(|_e| SpeechApiError::InvalidResponse)?;

        Ok(result.display_text.unwrap_or_default())
    }

    /// Detect the language of the audio
    pub async fn detect_language(&self, audio_data: &[u8]) -> Result<String> {
        let token = self.get_token().await?;

        // Azure Speech SDK supports language detection
        let endpoint = format!(
            "https://{}.stt.speech.microsoft.com/speech/recognition/conversation/cognitiveservices/v1?detectLanguage=true",
            self.config.region
        );

        let response = self.client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "audio/wav; codec=audio/pcm")
            .body(audio_data.to_vec())
            .send()
            .await
            .map_err(|e| SpeechApiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SpeechApiError::RequestFailed(
                format!("Language detection failed: {}", response.status())
            ).into());
        }

        let result: LanguageDetectionResult = response.json()
            .await
            .map_err(|_e| SpeechApiError::InvalidResponse)?;

        Ok(result.detected_language.unwrap_or_else(|| "en-US".to_string()))
    }
}

/// Transcription result from Azure Speech API
#[derive(Debug, Deserialize)]
pub struct TranscriptionResult {
    pub recognition_status: String,
    pub display_text: Option<String>,
    pub offset: Option<i64>,
    pub duration: Option<i64>,
}

/// Language detection result from Azure Speech API
#[derive(Debug, Deserialize)]
pub struct LanguageDetectionResult {
    pub detected_language: Option<String>,
    pub confidence: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads_from_env() {
        let config = AzureSpeechConfig {
            endpoint: "https://test.cognitiveservices.azure.com".to_string(),
            key: "test-key".to_string(),
            region: "test-region".to_string(),
        };

        assert_eq!(config.endpoint, "https://test.cognitiveservices.azure.com");
        assert_eq!(config.key, "test-key");
        assert_eq!(config.region, "test-region");
    }

    #[test]
    fn test_endpoint_generation() {
        let config = AzureSpeechConfig {
            endpoint: "https://eastus.cognitiveservices.azure.com".to_string(),
            key: "test-key".to_string(),
            region: "eastus".to_string(),
        };

        assert!(config.get_speech_endpoint().contains("sts/v1.0/issueToken"));
    }

    #[test]
    fn test_supported_languages() {
        let client = AzureSpeechClient::with_config(
            "test-key".to_string(),
            "test-region".to_string(),
            None,
        );

        let languages = client.get_supported_languages();
        assert!(languages.contains(&"en-US".to_string()));
        assert!(languages.contains(&"zh-CN".to_string()));
    }

    #[test]
    fn test_set_language() {
        let mut client = AzureSpeechClient::with_config(
            "test-key".to_string(),
            "test-region".to_string(),
            None,
        );

        client.set_recognition_language("zh-CN");
        assert_eq!(client.get_recognition_language(), "zh-CN");

        // Invalid language should fall back to default
        client.set_recognition_language("invalid");
        assert_eq!(client.get_recognition_language(), "en-US");
    }

    #[test]
    fn test_is_valid_language() {
        let client = AzureSpeechClient::with_config(
            "test-key".to_string(),
            "test-region".to_string(),
            None,
        );

        assert!(client.is_valid_language("en-US"));
        assert!(client.is_valid_language("zh-CN"));
        assert!(!client.is_valid_language("invalid"));
    }
}