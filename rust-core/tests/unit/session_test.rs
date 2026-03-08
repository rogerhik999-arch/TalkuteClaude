//! Session state machine unit tests

use talkute_core::state::{SessionState, TranscriptionSession, SessionManager};

#[tokio::test]
async fn test_session_creation() {
    let session = TranscriptionSession::new();
    assert_eq!(session.state, SessionState::Idle);
}

#[tokio::test]
async fn test_session_state_transitions() {
    let mut session = TranscriptionSession::new();

    // Idle -> Recording
    session.start_recording();
    assert_eq!(session.state, SessionState::Recording);

    // Recording -> Processing
    session.stop_recording(5);
    assert_eq!(session.state, SessionState::Processing);

    // Processing -> Idle (complete)
    session.complete("Test text".to_string());
    assert_eq!(session.state, SessionState::Idle);
}

#[tokio::test]
async fn test_session_cancel() {
    let mut session = TranscriptionSession::new();
    session.start_recording();
    session.cancel();
    assert_eq!(session.state, SessionState::Idle);
}

#[tokio::test]
async fn test_session_fail() {
    let mut session = TranscriptionSession::new();
    session.start_recording();
    session.fail("Test error".to_string());
    assert_eq!(session.state, SessionState::Error);
    assert_eq!(session.error_message, Some("Test error".to_string()));
}

#[tokio::test]
async fn test_session_manager_start() {
    let manager = SessionManager::new();
    let result = manager.start_session().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_session_manager_double_start_fails() {
    let manager = SessionManager::new();
    let _ = manager.start_session().await;
    let result = manager.start_session().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_manager_stop() {
    let manager = SessionManager::new();
    let session_id = manager.start_session().await.unwrap();
    let result = manager.stop_recording(&session_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_session_manager_cancel() {
    let manager = SessionManager::new();
    let session_id = manager.start_session().await.unwrap();
    let result = manager.cancel_session(&session_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_session_manager_complete() {
    let manager = SessionManager::new();
    let session_id = manager.start_session().await.unwrap();
    manager.stop_recording(&session_id).await.unwrap();
    let result = manager.complete_session(&session_id, "Test".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_session_manager_fail() {
    let manager = SessionManager::new();
    let session_id = manager.start_session().await.unwrap();
    let result = manager.fail_session(&session_id, "Error".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_session_state_event_broadcast() {
    let manager = SessionManager::new();
    let mut receiver = manager.subscribe();

    let _ = manager.start_session().await;

    // Should receive a state change event
    let event = receiver.recv().await;
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.state, SessionState::Recording);
}