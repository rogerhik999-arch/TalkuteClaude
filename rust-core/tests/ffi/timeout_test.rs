//! Tests for session timeout handling in FFI

use std::time::Duration;
use talkute_core::ffi::bridge::{
    start_voice_session, stop_voice_session, SessionStatus,
};
use talkute_core::ffi::session_manager::{SessionManager, MAX_SESSION_DURATION};
use chrono::Utc;

#[tokio::test]
async fn test_session_has_timestamp() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Get session from manager
    let manager = SessionManager::global();
    let session = manager.get_session(&session_id).await.unwrap();

    // Should have a started_at timestamp within the last 5 seconds
    let now = Utc::now();
    let diff = now - session.started_at;
    assert!(diff.num_seconds() < 5);
}

#[tokio::test]
async fn test_session_duration_tracking() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Wait a bit (use 500ms to ensure measurable duration)
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Get session from manager
    let manager = SessionManager::global();
    let session = manager.get_session(&session_id).await.unwrap();

    // Duration should be at least 400ms (allowing for some timing variance)
    assert!(session.duration() >= Duration::from_millis(400));
}

#[tokio::test]
async fn test_session_timeout_check() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Get session from manager
    let manager = SessionManager::global();
    let session = manager.get_session(&session_id).await.unwrap();

    // New session should not be timed out
    assert!(!session.is_timed_out(Duration::from_secs(300)));

    // Wait and check again with short timeout
    tokio::time::sleep(Duration::from_millis(100)).await;
    let session = manager.get_session(&session_id).await.unwrap();
    // Should be timed out with very short timeout (50ms)
    assert!(session.is_timed_out(Duration::from_millis(50)));
}

#[tokio::test]
async fn test_session_max_duration_five_minutes() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Get session from manager
    let manager = SessionManager::global();
    let session = manager.get_session(&session_id).await.unwrap();

    // Max duration should be 5 minutes (300 seconds)
    assert_eq!(session.max_duration, Duration::from_secs(300));
}

#[tokio::test]
async fn test_session_can_be_timed_out() {
    // Create a session manually with old timestamp
    let manager = SessionManager::global();
    let session_id = manager.create_session("test-device", None).await.unwrap();

    // Get the session and modify its started_at to simulate aging
    let mut session = manager.get_session(&session_id).await.unwrap();

    // Set started_at to 6 minutes ago (beyond 5-minute timeout)
    session.started_at = Utc::now() - chrono::Duration::seconds(360);

    // Update session in manager
    manager.update_session(&session_id, session.clone()).await.unwrap();

    // Get the updated session
    let updated_session = manager.get_session(&session_id).await.unwrap();

    // Should be timed out
    assert!(updated_session.is_timed_out(MAX_SESSION_DURATION));
}

#[tokio::test]
async fn test_stopping_session_records_duration() {
    // Start a session
    let session_id = start_voice_session("test-device", None).await.unwrap();

    // Wait a bit (use 500ms to ensure measurable duration)
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Stop the session
    let result = stop_voice_session(&session_id).await;
    assert!(result.is_ok());

    // Get session status
    let manager = SessionManager::global();
    let session = manager.get_session(&session_id).await.unwrap();

    // Status should be completed
    assert_eq!(session.status, SessionStatus::Completed);

    // Duration should be recorded (at least 400ms)
    assert!(session.duration() >= Duration::from_millis(400));
}

#[tokio::test]
async fn test_session_duration_calculation() {
    // Create a session
    let manager = SessionManager::global();
    let session_id = manager.create_session("test-device", None).await.unwrap();

    // Get session and verify duration starts near 0
    let session = manager.get_session(&session_id).await.unwrap();
    let initial_duration = session.duration();
    assert!(initial_duration < Duration::from_secs(1));

    // Wait 2 seconds
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Get session again and verify duration increased
    let session = manager.get_session(&session_id).await.unwrap();
    let later_duration = session.duration();
    assert!(later_duration >= Duration::from_secs(2));
}