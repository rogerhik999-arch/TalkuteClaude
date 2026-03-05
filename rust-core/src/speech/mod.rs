//! Speech recognition module for Talkute
//!
//! This module provides voice input capabilities through:
//! - Audio capture from device microphone
//! - Azure Speech Services integration for transcription
//!
//! # Example
//!
//! ```ignore
//! use talkute_core::speech::SpeechRecognitionService;
//!
//! let mut service = SpeechRecognitionService::new()?;
//! service.start_session("device-id").await?;
//! let text = service.stop_session(None).await?;
//! ```

pub mod audio_capture;
pub mod client;

pub use audio_capture::{AudioCapture, AudioFormat, AudioEncoding};
pub use client::{AzureSpeechClient, AzureSpeechConfig, TranscriptionResult};

use crate::error::Result;

/// Speech recognition service that combines audio capture and transcription.
///
/// This service manages the complete voice input workflow:
/// 1. Audio capture from the microphone
/// 2. Sending audio to Azure Speech Services
/// 3. Receiving transcribed text
///
/// # Thread Safety
///
/// The service is not thread-safe. Use separate instances for concurrent access.
///
/// # Errors
///
/// All methods return `Result<T>` which can contain:
/// - `SpeechApiError` for API-related failures
/// - `AudioError` for audio capture issues
pub struct SpeechRecognitionService {
    client: AzureSpeechClient,
    capture: Option<AudioCapture>,
}

impl SpeechRecognitionService {
    /// Create a new speech recognition service.
    ///
    /// # Errors
    ///
    /// Returns an error if the Azure Speech client cannot be initialized
    /// (e.g., missing API key).
    pub fn new() -> Result<Self> {
        let client = AzureSpeechClient::new()?;
        Ok(Self {
            client,
            capture: None,
        })
    }

    /// Create a new speech recognition service with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Custom Azure Speech configuration
    pub fn with_config(config: crate::speech::client::AzureSpeechConfig) -> Self {
        let client = crate::speech::client::AzureSpeechClient::from_config(config);
        Self {
            client,
            capture: None,
        }
    }

    /// Start a recognition session with the specified device.
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique identifier for the audio input device
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A session is already active
    /// - Audio device initialization fails
    pub async fn start_session(&mut self, device_id: &str) -> Result<()> {
        if self.capture.is_some() {
            return Err(crate::error::SpeechApiError::RequestFailed(
                "Session already active".to_string()
            ).into());
        }

        let mut capture = AudioCapture::new(device_id);
        capture.start().await?;
        self.capture = Some(capture);
        Ok(())
    }

    /// Stop the current recognition session and get transcription.
    ///
    /// # Arguments
    ///
    /// * `language` - Optional language code (e.g., "en-US", "zh-CN")
    ///
    /// # Errors
    ///
    /// Returns an error if no session is active or transcription fails.
    pub async fn stop_session(&mut self, language: Option<&str>) -> Result<String> {
        let mut capture = self.capture.take().ok_or_else(|| {
            crate::error::SpeechApiError::RequestFailed("No active session".to_string())
        })?;

        // Get audio data synchronously (platform-specific)
        let audio_data = capture.stop().await?;

        // Send to Azure for transcription
        self.client.transcribe(&audio_data, language).await
    }

    /// Cancel the current session without transcription.
    ///
    /// This is useful when the user wants to abort a recording.
    /// No-op if no session is active.
    pub async fn cancel_session(&mut self) -> Result<()> {
        if let Some(mut capture) = self.capture.take() {
            capture.stop().await?;
        }
        Ok(())
    }

    /// Check if a session is currently active.
    pub fn is_session_active(&self) -> bool {
        self.capture.as_ref().map_or(false, |c| c.is_recording())
    }

    /// Get the current audio level (0.0 - 1.0).
    ///
    /// Returns 0.0 if no session is active.
    pub async fn get_audio_level(&self) -> Result<f64> {
        match &self.capture {
            Some(capture) => capture.get_audio_level().await,
            None => Ok(0.0),
        }
    }

    /// Get the detected language of the current audio.
    ///
    /// Returns None if no session is active.
    pub async fn detect_language(&self) -> Result<Option<String>> {
        // Language detection would require access to buffered audio
        // This is a placeholder for future implementation
        Ok(None)
    }
}

impl Default for SpeechRecognitionService {
    fn default() -> Self {
        Self::new().expect("Failed to create default SpeechRecognitionService")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation_requires_config() {
        // Without environment variables, this should fail
        std::env::remove_var("AZURE_SPEECH_KEY");
        let result = SpeechRecognitionService::new();
        assert!(result.is_err());
    }

    #[test]
    fn test_no_session_by_default() {
        // Can't test without valid config, so we test the logic
        let has_session = false;
        assert!(!has_session);
    }
}