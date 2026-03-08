//! Tests for transcription FFI

use talkute_core::ffi::bridge::{
    start_voice_session, stop_voice_session, get_raw_transcription, get_polished_text,
    SessionStatus,
};
use talkute_core::ffi::session_manager::SessionManager;

#[tokio::test]
async fn test_get_raw_transcription_empty() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Get raw transcription (should be empty)
    let result = get_raw_transcription(&session_id).await;
    assert!(result.is_ok());

    let text = result.unwrap();
    assert!(text.is_empty());
}

#[tokio::test]
async fn test_get_polished_text_empty() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Get polished text (should be empty)
    let result = get_polished_text(&session_id).await;
    assert!(result.is_ok());

    let text = result.unwrap();
    assert!(text.is_empty());
}

#[tokio::test]
async fn test_get_raw_transcription_nonexistent() {
    // Try to get transcription for nonexistent session
    let result = get_raw_transcription("nonexistent-session").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_polished_text_nonexistent() {
    // Try to get polished text for nonexistent session
    let result = get_polished_text("nonexistent-session").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_set_raw_transcription() {
    let manager = SessionManager::global();

    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Set raw transcription
    manager.set_raw_transcription(&session_id, "Hello world").await.unwrap();

    // Get raw transcription
    let result = get_raw_transcription(&session_id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello world");
}

#[tokio::test]
async fn test_set_polished_text() {
    let manager = SessionManager::global();

    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Set polished text
    manager.set_polished_text(&session_id, "Hello, world.").await.unwrap();

    // Get polished text
    let result = get_polished_text(&session_id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, world.");
}

#[tokio::test]
async fn test_word_count() {
    let manager = SessionManager::global();

    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Set raw transcription
    manager.set_raw_transcription(&session_id, "One two three four five").await.unwrap();

    // Check session info
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.word_count, 5);
}