// Speech module
pub mod audio_capture;
pub mod client;

pub use audio_capture::{AudioCapture, AudioFormat, AudioEncoding};
pub use client::{AzureSpeechClient, AzureSpeechConfig, TranscriptionResult};

use crate::error::Result;

/// Speech recognition service that combines audio capture and transcription
pub struct SpeechRecognitionService {
    client: AzureSpeechClient,
    capture: Option<AudioCapture>,
}

impl SpeechRecognitionService {
    /// Create a new speech recognition service
    pub fn new() -> Result<Self> {
        let client = AzureSpeechClient::new()?;
        Ok(Self {
            client,
            capture: None,
        })
    }

    /// Start a recognition session with the specified device
    pub async fn start_session(&mut self, device_id: &str) -> Result<()> {
        let mut capture = AudioCapture::new(device_id);
        capture.start().await?;
        self.capture = Some(capture);
        Ok(())
    }

    /// Stop the current recognition session and get transcription
    pub async fn stop_session(&mut self, language: Option<&str>) -> Result<String> {
        let mut capture = self.capture.take().ok_or_else(|| {
            crate::error::SpeechApiError::RequestFailed("No active session".to_string())
        })?;

        let audio_data = capture.stop().await?;
        self.client.transcribe(&audio_data, language).await
    }

    /// Cancel the current session without transcription
    pub async fn cancel_session(&mut self) -> Result<()> {
        if let Some(mut capture) = self.capture.take() {
            capture.stop().await?;
        }
        Ok(())
    }

    /// Check if a session is active
    pub fn is_session_active(&self) -> bool {
        self.capture.as_ref().map_or(false, |c| c.is_recording())
    }

    /// Get the current audio level (0.0 - 1.0)
    pub async fn get_audio_level(&self) -> Result<f64> {
        if let Some(capture) = &self.capture {
            capture.get_audio_level().await
        } else {
            Ok(0.0)
        }
    }
}
