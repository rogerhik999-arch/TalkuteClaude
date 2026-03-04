//! Tests for speech recognition service

use talkute_core::speech::{SpeechRecognitionService, AudioCapture, AudioFormat, AudioEncoding};

#[test]
fn test_audio_capture_creation() {
    let capture = AudioCapture::new("test-device");

    assert_eq!(capture.device_id(), "test-device");
    assert!(!capture.is_recording());
}

#[test]
fn test_audio_format_defaults() {
    let format = AudioFormat::default();

    assert_eq!(format.sample_rate, 16000);
    assert_eq!(format.channels, 1);
    assert_eq!(format.bits_per_sample, 16);
    assert!(matches!(format.encoding, AudioEncoding::Pcm));
}

#[test]
fn test_audio_format_high_fidelity() {
    let format = AudioFormat::high_fidelity();

    assert_eq!(format.sample_rate, 44100);
    assert_eq!(format.channels, 2);
}

#[tokio::test]
async fn test_audio_capture_start_stop() {
    let mut capture = AudioCapture::new("test-device");

    // Start capture
    let result = capture.start().await;
    assert!(result.is_ok());
    assert!(capture.is_recording());

    // Stop capture
    let result = capture.stop().await;
    assert!(result.is_ok());
    assert!(!capture.is_recording());
}

#[tokio::test]
async fn test_audio_capture_double_start() {
    let mut capture = AudioCapture::new("test-device");

    // First start should succeed
    let result = capture.start().await;
    assert!(result.is_ok());

    // Second start should fail
    let result = capture.start().await;
    assert!(result.is_err());

    // Clean up
    capture.stop().await.ok();
}

#[tokio::test]
async fn test_audio_capture_stop_without_start() {
    let mut capture = AudioCapture::new("test-device");

    // Stop without start should fail
    let result = capture.stop().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_audio_level_when_not_recording() {
    let capture = AudioCapture::new("test-device");

    let level = capture.get_audio_level().await;
    assert!(level.is_ok());
    assert_eq!(level.unwrap(), 0.0);
}

#[test]
fn test_speech_recognition_service_creation() {
    // This test requires AZURE_SPEECH_KEY env var
    // Skip if not available
    if std::env::var("AZURE_SPEECH_KEY").is_err() {
        return;
    }

    let result = SpeechRecognitionService::new();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_service_session_lifecycle() {
    // This test requires AZURE_SPEECH_KEY env var
    // Skip if not available
    if std::env::var("AZURE_SPEECH_KEY").is_err() {
        return;
    }

    let Ok(mut service) = SpeechRecognitionService::new() else {
        return;
    };

    // Initially no session
    assert!(!service.is_session_active());

    // Start session
    let result = service.start_session("test-device").await;
    assert!(result.is_ok());
    assert!(service.is_session_active());

    // Cancel session
    let result = service.cancel_session().await;
    assert!(result.is_ok());
    assert!(!service.is_session_active());
}

#[tokio::test]
async fn test_service_audio_level() {
    // This test requires AZURE_SPEECH_KEY env var
    // Skip if not available
    if std::env::var("AZURE_SPEECH_KEY").is_err() {
        return;
    }

    let Ok(mut service) = SpeechRecognitionService::new() else {
        return;
    };

    // No session - should return 0.0
    let level = service.get_audio_level().await;
    assert!(level.is_ok());
    assert_eq!(level.unwrap(), 0.0);

    // Start session
    service.start_session("test-device").await.ok();

    // With session - should return some value
    let level = service.get_audio_level().await;
    assert!(level.is_ok());

    // Clean up
    service.cancel_session().await.ok();
}
