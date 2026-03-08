//! Tests for Azure Speech API client

use crate::speech::client::{AzureSpeechConfig, TranscriptionResult};

#[test]
fn test_azure_config_creation() {
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

    let endpoint = config.get_speech_endpoint();
    assert!(endpoint.contains("sts/v1.0/issueToken"));
}

#[test]
fn test_transcription_result_parsing() {
    let result = TranscriptionResult {
        recognition_status: "Success".to_string(),
        display_text: Some("Hello world".to_string()),
        offset: Some(0),
        duration: Some(1000),
    };

    assert_eq!(result.recognition_status, "Success");
    assert_eq!(result.display_text, Some("Hello world".to_string()));
}

#[test]
fn test_transcription_result_no_text() {
    let result = TranscriptionResult {
        recognition_status: "Success".to_string(),
        display_text: None,
        offset: None,
        duration: None,
    };

    assert_eq!(result.display_text, None);
}

#[test]
fn test_default_format_creation() {
    let format = crate::speech::audio_capture::AudioFormat::default();

    assert_eq!(format.sample_rate, 16000);
    assert_eq!(format.channels, 1);
}

#[test]
fn test_high_fidelity_format() {
    let format = crate::speech::audio_capture::AudioFormat::high_fidelity();

    assert_eq!(format.sample_rate, 44100);
    assert_eq!(format.channels, 2);
}

#[test]
fn test_audio_capture_state() {
    let capture = crate::speech::audio_capture::AudioCapture::new("test-device");

    assert!(!capture.is_recording());
}
