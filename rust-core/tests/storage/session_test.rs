//! Tests for VoiceSession model

use crate::storage::models::VoiceSession;

#[test]
fn test_voice_session_creation() {
    let session = VoiceSession::new("session-123".to_string(), "device-456".to_string());

    assert_eq!(session.session_id, "session-123");
    assert_eq!(session.device_id, "device-456");
    assert_eq!(session.context_id, None);
    assert_eq!(session.status, "RECORDING");
    assert_eq!(session.word_count, 0);
    assert!(session.started_at <= chrono::Utc::now());
}

#[test]
fn test_voice_session_with_context() {
    let session = VoiceSession::new("session-789".to_string(), "device-456".to_string());
    let session_with_context = session.clone();

    assert_eq!(session_with_context.session_id, "session-789");
}

#[test]
fn test_voice_session_fields() {
    let session = VoiceSession::new("session-test".to_string(), "device-test".to_string());

    // Test that all fields are accessible
    assert!(!session.session_id.is_empty());
    assert!(!session.device_id.is_empty());
    assert!(session.raw_transcription.is_none());
    assert!(session.polished_text.is_none());
    assert!(session.error_message.is_none());
}

#[test]
fn test_voice_session_default_status() {
    let session = VoiceSession::new("session-1".to_string(), "device-1".to_string());

    // Default status should be RECORDING
    assert_eq!(session.status, "RECORDING");
}

#[test]
fn test_voice_session_word_count_default() {
    let session = VoiceSession::new("session-1".to_string(), "device-1".to_string());

    // Default word count should be 0
    assert_eq!(session.word_count, 0);
}
