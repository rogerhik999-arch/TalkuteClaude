//! Tests for TranscriptionHistory model

use crate::storage::models::TranscriptionHistory;

#[test]
fn test_transcription_history_creation() {
    let history = TranscriptionHistory::new(
        "history-123".to_string(),
        "session-456".to_string(),
        "RAW_SPEECH".to_string(),
        "Hello world".to_string(),
    );

    assert_eq!(history.history_id, "history-123");
    assert_eq!(history.session_id, "session-456");
    assert_eq!(history.stage, "RAW_SPEECH");
    assert_eq!(history.text_content, "Hello world");
    assert!(history.timestamp <= chrono::Utc::now());
}

#[test]
fn test_transcription_history_fields() {
    let history = TranscriptionHistory::new(
        "history-789".to_string(),
        "session-789".to_string(),
        "TRANSCRIBED".to_string(),
        "Test content".to_string(),
    );

    assert!(!history.history_id.is_empty());
    assert!(!history.session_id.is_empty());
    assert!(history.metadata.is_none());
}

#[test]
fn test_transcription_history_metadata() {
    let history = TranscriptionHistory::new(
        "history-meta".to_string(),
        "session-meta".to_string(),
        "AI_POLISHED".to_string(),
        "Final text".to_string(),
    );
    let history_with_metadata = history.clone();

    // Metadata should be None by default
    assert!(history_with_metadata.metadata.is_none());
}

#[test]
fn test_transcription_history_various_stages() {
    let stages = vec![
        "RAW_SPEECH",
        "TRANSCRIBED",
        "FILLER_REMOVED",
        "SELF_CORRECTED",
        "AI_POLISHED",
    ];

    for (i, stage) in stages.iter().enumerate() {
        let history = TranscriptionHistory::new(
            format!("history-{}", i),
            format!("session-{}", i),
            stage.to_string(),
            format!("Content for {}", stage),
        );

        assert_eq!(history.stage, *stage);
    }
}
