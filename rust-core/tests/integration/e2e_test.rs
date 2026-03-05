//! End-to-end integration test: Complete voice-to-text flow
//!
//! This test validates the complete workflow:
//! 1. Session creation and activation
//! 2. Audio capture (simulated)
//! 3. Transcription (mocked)
//! 4. Text processing pipeline
//! 5. AI polishing
//! 6. Result display via FFI

use std::time::Duration;
use talkute_core::ffi::bridge::{
    start_voice_session, stop_voice_session, get_raw_transcription, get_polished_text,
    SessionStatus,
};
use talkute_core::ffi::session_manager::SessionManager;
use talkute_core::processing::TextProcessingPipeline;
use talkute_core::quota::tracker::QuotaTracker;
use talkute_core::storage::database::Database;

/// Test the complete voice-to-text flow
#[tokio::test]
async fn test_complete_voice_to_text_flow() {
    // Step 1: Create a voice session
    let session_id = start_voice_session("test-device", None).await.unwrap();
    assert!(!session_id.is_empty());

    // Verify session was created with correct initial state
    let manager = SessionManager::global();
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Idle);
    assert!(session.raw_transcription.is_empty());
    assert!(session.polished_text.is_empty());

    // Step 2: Simulate recording (session status update)
    manager.update_status(&session_id, SessionStatus::Recording).await.unwrap();
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Recording);

    // Step 3: Simulate audio level updates during recording
    for _ in [0.3, 0.5, 0.8, 0.6, 0.4] {
        // In a real implementation, this would come from audio capture
        // For testing, we just verify the session handles state correctly
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    // Step 4: Simulate transcription result
    let raw_transcription = "I wanted to um schedule a meeting for tomorrow";
    manager.set_raw_transcription(&session_id, raw_transcription).await.unwrap();

    // Step 5: Apply text processing pipeline
    let pipeline = TextProcessingPipeline::new();
    let processed = pipeline.process(raw_transcription);

    // Step 6: Set polished text
    manager.set_polished_text(&session_id, &processed).await.unwrap();

    // Step 7: Verify the complete flow results
    let result = get_raw_transcription(&session_id).await.unwrap();
    assert_eq!(result, raw_transcription);

    let polished = get_polished_text(&session_id).await.unwrap();
    assert!(!polished.contains("um")); // Filler should be removed

    // Step 8: Verify word count
    let session = manager.get_session(&session_id).await.unwrap();
    assert!(session.word_count > 0);

    // Step 9: Clean up session
    stop_voice_session(&session_id).await.unwrap();
}

/// Test session lifecycle with context
#[tokio::test]
async fn test_session_with_context() {
    // Create session with context
    let session_id = start_voice_session("test-device", Some("email".to_string())).await.unwrap();

    let manager = SessionManager::global();
    let session = manager.get_session(&session_id).await.unwrap();

    // Verify context was set
    assert_eq!(session.context_id, Some("email".to_string()));
}

/// Test quota checking before session start
#[tokio::test]
async fn test_quota_check_before_session() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Check quota is available
    let available = tracker.check_quota(100).await.expect("Failed to check quota");
    assert!(available, "Should have quota available");

    // Add words up to limit
    tracker.add_words(4000).await.expect("Failed to add words");

    // Now quota should not be available
    let available = tracker.check_quota(100).await.expect("Failed to check quota");
    assert!(!available, "Should not have quota available after using limit");
}

/// Test concurrent sessions are handled correctly
#[tokio::test]
async fn test_concurrent_session_handling() {
    // Start multiple sessions
    let session1 = start_voice_session("device-1", None).await.unwrap();
    let session2 = start_voice_session("device-2", None).await.unwrap();

    // Verify both sessions exist and are independent
    assert_ne!(session1, session2);

    let manager = SessionManager::global();

    // Update sessions independently
    manager.set_raw_transcription(&session1, "Session 1 text").await.unwrap();
    manager.set_raw_transcription(&session2, "Session 2 text").await.unwrap();

    // Verify isolation
    let text1 = get_raw_transcription(&session1).await.unwrap();
    let text2 = get_raw_transcription(&session2).await.unwrap();

    assert_eq!(text1, "Session 1 text");
    assert_eq!(text2, "Session 2 text");
}

/// Test error recovery during processing
#[tokio::test]
async fn test_error_recovery() {
    let session_id = start_voice_session("test-device", None).await.unwrap();
    let manager = SessionManager::global();

    // Simulate an error state
    manager.update_status(&session_id, SessionStatus::Failed).await.unwrap();

    // Verify error state
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Failed);

    // Recovery: start a new session
    let new_session_id = start_voice_session("test-device", None).await.unwrap();
    assert_ne!(session_id, new_session_id);

    // New session should be in idle state
    let session = manager.get_session(&new_session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Idle);
}

/// Test session timeout handling
#[tokio::test]
async fn test_session_timeout() {
    let session_id = start_voice_session("test-device", None).await.unwrap();
    let manager = SessionManager::global();

    // Start recording
    manager.update_status(&session_id, SessionStatus::Recording).await.unwrap();

    // Verify session is active
    let session = manager.get_session(&session_id).await.unwrap();
    assert!(!session.is_timed_out(Duration::from_secs(300)));

    // Simulate time passing (by checking with a very short timeout)
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Session should timeout with very short duration
    let session = manager.get_session(&session_id).await.unwrap();
    assert!(session.is_timed_out(Duration::from_millis(1)));
}

/// Test text processing pipeline integration
#[tokio::test]
async fn test_text_processing_pipeline_integration() {
    let pipeline = TextProcessingPipeline::new();

    // Test various input types
    let test_cases = vec![
        // (input, should_not_contain)
        ("I wanted to um schedule a meeting", "um"),
        ("I will be there like tomorrow", "like"),
        ("I need to uh call them", "uh"),
    ];

    for (input, filler) in test_cases {
        let result = pipeline.process(input);
        assert!(!result.contains(filler), "Should remove '{}' from '{}'", filler, input);
        assert!(!result.is_empty(), "Result should not be empty");
    }
}

/// Test complete workflow with quota tracking
#[tokio::test]
async fn test_complete_workflow_with_quota() {
    let db = Database::in_memory().expect("Failed to create database");
    let tracker = QuotaTracker::new(db).expect("Failed to create tracker");

    // Start session
    let session_id = start_voice_session("test-device", None).await.unwrap();
    let manager = SessionManager::global();

    // Simulate transcription with known word count
    let text = "one two three four five six seven eight nine ten";
    manager.set_raw_transcription(&session_id, text).await.unwrap();

    // Process and polish
    let pipeline = TextProcessingPipeline::new();
    let polished = pipeline.process(text);
    manager.set_polished_text(&session_id, &polished).await.unwrap();

    // Get word count from session
    let session = manager.get_session(&session_id).await.unwrap();
    let word_count = session.word_count;

    // Record usage in quota tracker
    tracker.add_words(word_count).await.expect("Failed to add words");

    // Verify quota was updated
    let usage = tracker.get_weekly_usage().await.expect("Failed to get usage");
    assert_eq!(usage, word_count);
}

/// Test FFI boundary correctness
#[tokio::test]
async fn test_ffi_boundary() {
    // Test that FFI functions handle all edge cases correctly

    // 1. Non-existent session should return error
    let result = get_raw_transcription("non-existent-session").await;
    assert!(result.is_err());

    // 2. Valid session should work
    let session_id = start_voice_session("test-device", None).await.unwrap();
    let result = get_raw_transcription(&session_id).await;
    assert!(result.is_ok());

    // 3. Empty transcription should return empty string
    let text = result.unwrap();
    assert!(text.is_empty());
}

/// Test session status transitions
#[tokio::test]
async fn test_session_status_transitions() {
    let session_id = start_voice_session("test-device", None).await.unwrap();
    let manager = SessionManager::global();

    // Verify initial state
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Idle);

    // Transition to recording
    manager.update_status(&session_id, SessionStatus::Recording).await.unwrap();
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Recording);

    // Transition to transcribing
    manager.update_status(&session_id, SessionStatus::Transcribing).await.unwrap();
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Transcribing);

    // Transition to polishing
    manager.update_status(&session_id, SessionStatus::Polishing).await.unwrap();
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Polishing);

    // Transition to completed
    manager.update_status(&session_id, SessionStatus::Completed).await.unwrap();
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Completed);
}