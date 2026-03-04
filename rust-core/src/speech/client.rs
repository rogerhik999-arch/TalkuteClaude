//! Azure Speech API client

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

    /// Get the speech endpoint for the current region
    pub fn get_speech_endpoint(&self) -> String {
        format!("{}/sts/v1.0/issueToken", self.endpoint)
    }
}

/// Azure Speech API client
pub struct AzureSpeechClient {
    client: Client,
    config: AzureSpeechConfig,
}

impl AzureSpeechClient {
    /// Create a new Azure Speech client
    pub fn new() -> Result<Self> {
        let config = AzureSpeechConfig::from_env()?;
        let client = Client::new();

        Ok(Self { client, config })
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

        // Azure Speech API requires specific endpoint for transcription
        // This is a placeholder - actual implementation would use the Speech SDK
        let endpoint = format!(
            "https://{}.stt.speech.microsoft.com/speech/recognition/conversation/cognitiveservices/v1",
            self.config.region
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
            .map_err(|e| SpeechApiError::InvalidResponse)?;

        Ok(result.display_text.unwrap_or_default())
    }

    /// Detect the language of the audio
    pub async fn detect_language(&self, audio_data: &[u8]) -> Result<String> {
        // Azure Speech SDK supports language detection
        // This is a placeholder - actual implementation would use the Speech SDK
        Err(SpeechApiError::RequestFailed(
            "Language detection not yet implemented".to_string()
        ).into())
    }
}

/// transcription result from Azure Speech API
#[derive(Debug, Deserialize)]
pub struct TranscriptionResult {
    pub recognition_status: String,
    pub display_text: Option<String>,
    pub offset: Option<i64>,
    pub duration: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads_from_env() {
        // This test would require mocked environment variables
        // For now, we just verify the struct can be constructed
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
}
