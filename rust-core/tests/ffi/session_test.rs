//! Tests for session management FFI

use talkute_core::ffi::bridge::{
    start_voice_session, stop_voice_session, cancel_voice_session, get_session_status,
    SessionStatus,
};
use talkute_core::ffi::session_manager::SessionManager;

#[tokio::test]
async fn test_start_voice_session() {
    let manager = SessionManager::global();

    // Start a new session
    let result = start_voice_session("test-device", None).await;
    assert!(result.is_ok());

    let session_id = result.unwrap();
    assert!(!session_id.is_empty());

    // Verify session exists
    assert!(manager.session_exists(&session_id).await);
}

#[tokio::test]
async fn test_start_voice_session_with_context() {
    let context_id = Some("email-context".to_string());

    // Start a new session with context
    let result = start_voice_session("test-device", context_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stop_voice_session() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Stop the session
    let result = stop_voice_session(&session_id).await;
    assert!(result.is_ok());

    let info = result.unwrap();
    assert_eq!(info.session_id, session_id);
    assert_eq!(info.status, SessionStatus::Completed);
}

#[tokio::test]
async fn test_cancel_voice_session() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Cancel the session
    let result = cancel_voice_session(&session_id, Some("user cancelled")).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_session_status() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Get status
    let result = get_session_status(&session_id).await;
    assert!(result.is_ok());

    let status = result.unwrap();
    assert_eq!(status, SessionStatus::Idle);
}

#[tokio::test]
async fn test_stop_nonexistent_session() {
    // Try to stop a session that doesn't exist
    let result = stop_voice_session("nonexistent-session").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_cancel_nonexistent_session() {
    // Try to cancel a session that doesn't exist
    let result = cancel_voice_session("nonexistent-session", None).await;
    assert!(result.is_err());
}