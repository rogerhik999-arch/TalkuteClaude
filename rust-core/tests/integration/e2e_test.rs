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

// ============================================================================
// Context-Aware Tone Adaptation Tests (User Story 2)
// ============================================================================

use talkute_core::context::detector::UnifiedContextDetector;
use talkute_core::ai::prompts::AIPrompts;

/// Test context-aware tone adaptation: same phrase produces different tones
#[tokio::test]
async fn test_context_aware_tone_adaptation() {
    let prompts = AIPrompts::new();

    // Get prompts for different contexts
    let email_prompt = prompts.for_context("email");
    let chat_prompt = prompts.for_context("chat");
    let code_prompt = prompts.for_context("code");

    // Verify prompts are different for different contexts
    assert_ne!(email_prompt, chat_prompt, "Email and chat prompts should differ");
    assert_ne!(chat_prompt, code_prompt, "Chat and code prompts should differ");

    // Verify email prompt emphasizes formal tone
    let email_lower = email_prompt.to_lowercase();
    assert!(
        email_lower.contains("formal") || email_lower.contains("professional"),
        "Email prompt should mention formal/professional tone"
    );

    // Verify chat prompt emphasizes casual tone
    let chat_lower = chat_prompt.to_lowercase();
    assert!(
        chat_lower.contains("casual") || chat_lower.contains("conversational"),
        "Chat prompt should mention casual/conversational tone"
    );

    // Verify code prompt is technical
    let code_lower = code_prompt.to_lowercase();
    assert!(
        code_lower.contains("technical") || code_lower.contains("code"),
        "Code prompt should mention technical aspects"
    );
}

/// Test context detection flow
#[tokio::test]
async fn test_context_detection_flow() {
    // Create context detector
    let detector = UnifiedContextDetector::new().await.expect("Failed to create detector");

    // Detect current context
    let result = detector.detect().await;
    assert!(result.is_ok(), "Context detection should succeed");

    let context = result.unwrap();

    // Verify context structure
    assert!(!context.context_id.is_empty(), "Context ID should not be empty");
    assert!(!context.application_name.is_empty(), "Application name should not be empty");
    assert!(!context.application_category.is_empty(), "Category should not be empty");

    // Verify category is one of the known categories
    let valid_categories = vec!["email", "chat", "document", "code", "browser", "general", "other"];
    assert!(
        valid_categories.contains(&context.application_category.as_str()),
        "Category '{}' should be a valid category",
        context.application_category
    );
}

/// Test session with detected context
#[tokio::test]
async fn test_session_with_detected_context() {
    // Detect context
    let detector = UnifiedContextDetector::new().await.expect("Failed to create detector");
    let context = detector.detect().await.expect("Failed to detect context");

    // Create session with detected context
    let session_id = start_voice_session("test-device", Some(context.application_category.clone())).await.unwrap();

    let manager = SessionManager::global();
    let session = manager.get_session(&session_id).await.unwrap();

    // Verify context was applied to session
    assert_eq!(session.context_id, Some(context.application_category));
}

/// Test tone selection based on application category
#[tokio::test]
async fn test_tone_selection_by_category() {
    let prompts = AIPrompts::new();

    // Test all major categories have appropriate prompts
    let categories = vec![
        ("email", vec!["formal", "professional"]),
        ("chat", vec!["casual", "conversational"]),
        ("document", vec!["professional", "document"]),
        ("code", vec!["technical", "code"]),
        ("browser", vec!["browser", "web"]),
    ];

    for (category, expected_keywords) in categories {
        let prompt = prompts.for_context(category);
        let prompt_lower = prompt.to_lowercase();

        let has_keyword = expected_keywords.iter().any(|kw| prompt_lower.contains(kw));
        assert!(
            has_keyword,
            "Category '{}' prompt should contain one of {:?}",
            category,
            expected_keywords
        );
    }
}

/// Test context detection categorization consistency
#[tokio::test]
async fn test_context_categorization_consistency() {
    let detector = UnifiedContextDetector::new().await.expect("Failed to create detector");

    // Test consistent categorization for known applications
    let test_cases = vec![
        ("Gmail", "email"),
        ("Outlook", "email"),
        ("Slack", "chat"),
        ("Discord", "chat"),
        ("VSCode", "code"),
        ("IntelliJ IDEA", "code"),
        ("Microsoft Word", "document"),
        ("Google Chrome", "browser"),
    ];

    for (app_name, expected_category) in test_cases {
        let category = detector.categorize_application(app_name);
        assert_eq!(
            category, expected_category,
            "Application '{}' should be categorized as '{}'",
            app_name, expected_category
        );
    }
}

/// Test complete context-aware workflow
#[tokio::test]
async fn test_complete_context_aware_workflow() {
    // Step 1: Detect context
    let detector = UnifiedContextDetector::new().await.expect("Failed to create detector");
    let context = detector.detect().await.expect("Failed to detect context");

    // Step 2: Get appropriate prompt for context
    let prompts = AIPrompts::new();
    let context_prompt = prompts.for_context(&context.application_category);
    assert!(!context_prompt.is_empty(), "Prompt should not be empty");

    // Step 3: Create session with context
    let session_id = start_voice_session("test-device", Some(context.application_category.clone())).await.unwrap();
    let manager = SessionManager::global();

    // Step 4: Simulate voice input
    let raw_text = "I wanted to um schedule a meeting for tomorrow";
    manager.set_raw_transcription(&session_id, raw_text).await.unwrap();

    // Step 5: Process text
    let pipeline = TextProcessingPipeline::new();
    let processed = pipeline.process(raw_text);
    assert!(!processed.contains("um"), "Filler words should be removed");

    // Step 6: Store processed text and mark completed
    manager.update_status(&session_id, SessionStatus::Completed).await.unwrap();
    manager.set_polished_text(&session_id, &processed).await.unwrap();

    // Step 7: Verify complete workflow
    let session = manager.get_session(&session_id).await.unwrap();
    assert_eq!(session.status, SessionStatus::Completed);
    assert!(session.word_count > 0);
}

// ============================================================================
// Personal Dictionary Tests (User Story 3)
// ============================================================================

use talkute_core::storage::dictionary::DictionaryStorage;
use talkute_core::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use talkute_core::storage::dictionary::apply_dictionary_to_text;
use chrono::Utc;

fn setup_test_device(db: &Database) {
    db.connection().execute(
        "INSERT OR IGNORE INTO device_profiles (device_id, created_at, last_active_at)
         VALUES ('test-device', datetime('now'), datetime('now'))",
        [],
    ).expect("Failed to create device profile");
}

fn create_test_entry(phrase: &str, replacement: &str, category: DictionaryEntryCategory) -> PersonalDictionaryEntry {
    PersonalDictionaryEntry {
        entry_id: format!("entry-{}", uuid::Uuid::new_v4()),
        device_id: "test-device".to_string(),
        phrase: phrase.to_string(),
        replacement: replacement.to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    }
}

/// Test complete personal dictionary workflow: add term → process text → term recognized
#[tokio::test]
async fn test_personal_dictionary_workflow() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_test_device(&db);
    let storage = DictionaryStorage::new(&db);

    // Step 1: Add custom term to dictionary
    let entry = create_test_entry(
        "API",
        "Application Programming Interface",
        DictionaryEntryCategory::Technical,
    );
    let entry_id = storage.add_entry(&entry).expect("Failed to add entry");
    assert!(!entry_id.is_empty());

    // Step 2: Verify entry was stored
    let entries = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].phrase, "API");

    // Step 3: Process text containing the custom term
    let text = "I need to use the API for my project";
    let processed = apply_dictionary_to_text(text, &entries);

    // Step 4: Verify the term was correctly replaced
    assert!(
        processed.contains("Application Programming Interface"),
        "Dictionary term should be replaced: got '{}'",
        processed
    );
    assert!(
        !processed.contains("API"),
        "Original term should be replaced"
    );
}

/// Test dictionary with multiple entries and priority
#[tokio::test]
async fn test_dictionary_multiple_entries_priority() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_test_device(&db);
    let storage = DictionaryStorage::new(&db);

    // Add entries - using non-overlapping phrases to test priority
    let entries = vec![
        create_test_entry("API", "Application Programming Interface", DictionaryEntryCategory::Technical),
        create_test_entry("CI/CD", "Continuous Integration/Continuous Deployment", DictionaryEntryCategory::Technical),
        create_test_entry("PR", "Pull Request", DictionaryEntryCategory::Technical),
    ];

    for entry in &entries {
        storage.add_entry(entry).expect("Failed to add entry");
    }

    // Get all entries
    let stored_entries = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(stored_entries.len(), 3);

    // Process text with multiple dictionary terms
    let text = "We need to review the PR and improve our CI/CD pipeline using the API";
    let processed = apply_dictionary_to_text(text, &stored_entries);

    // Verify all terms are replaced
    assert!(
        processed.contains("Application Programming Interface"),
        "Term 'API' should be replaced"
    );
    assert!(
        processed.contains("Continuous Integration/Continuous Deployment"),
        "Term 'CI/CD' should be replaced"
    );
    assert!(
        processed.contains("Pull Request"),
        "Term 'PR' should be replaced"
    );
}

/// Test dictionary entry categories
#[tokio::test]
async fn test_dictionary_categories() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_test_device(&db);
    let storage = DictionaryStorage::new(&db);

    // Add entries in different categories
    let entries = vec![
        create_test_entry("API", "Application Programming Interface", DictionaryEntryCategory::Technical),
        create_test_entry("ROI", "Return on Investment", DictionaryEntryCategory::Business),
        create_test_entry("MRI", "Magnetic Resonance Imaging", DictionaryEntryCategory::Medical),
        create_test_entry("FYI", "For Your Information", DictionaryEntryCategory::General),
    ];

    for entry in &entries {
        storage.add_entry(entry).expect("Failed to add entry");
    }

    // Verify all entries are stored with correct categories
    let stored = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(stored.len(), 4);

    let categories: Vec<_> = stored.iter().map(|e| e.category.clone()).collect();
    assert!(categories.contains(&DictionaryEntryCategory::Technical));
    assert!(categories.contains(&DictionaryEntryCategory::Business));
    assert!(categories.contains(&DictionaryEntryCategory::Medical));
    assert!(categories.contains(&DictionaryEntryCategory::General));
}

/// Test dictionary entry CRUD operations
#[tokio::test]
async fn test_dictionary_crud_operations() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_test_device(&db);
    let storage = DictionaryStorage::new(&db);

    // Create
    let entry = create_test_entry("API", "Application Programming Interface", DictionaryEntryCategory::Technical);
    let entry_id = storage.add_entry(&entry).expect("Failed to add entry");

    // Read
    let found = storage.get_entry_by_id(&entry_id, "test-device")
        .expect("Failed to get entry")
        .expect("Entry not found");
    assert_eq!(found.phrase, "API");

    // Update
    storage.update_entry(
        &entry_id,
        "test-device",
        Some("REST API"),
        Some("Representational State Transfer API"),
        Some(true),
        Some(DictionaryEntryCategory::General),
    ).expect("Failed to update entry");

    // Verify update
    let updated = storage.get_entry_by_id(&entry_id, "test-device")
        .expect("Failed to get entry")
        .expect("Entry not found");
    assert_eq!(updated.phrase, "REST API");
    assert_eq!(updated.replacement, "Representational State Transfer API");
    assert!(updated.case_sensitive);
    assert_eq!(updated.category, DictionaryEntryCategory::General);

    // Delete
    storage.remove_entry(&entry_id, "test-device").expect("Failed to remove entry");

    // Verify deletion
    let entries = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(entries.len(), 0);
}

/// Test dictionary import/export
#[tokio::test]
async fn test_dictionary_import_export() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_test_device(&db);
    let storage = DictionaryStorage::new(&db);

    // Add entries
    let entries = vec![
        create_test_entry("API", "Application Programming Interface", DictionaryEntryCategory::Technical),
        create_test_entry("ROI", "Return on Investment", DictionaryEntryCategory::Business),
    ];

    for entry in &entries {
        storage.add_entry(entry).expect("Failed to add entry");
    }

    // Export
    let json = storage.export_to_json("test-device").expect("Failed to export");
    assert!(json.contains("API"));
    assert!(json.contains("ROI"));

    // Clear
    for entry in storage.get_all_entries("test-device").unwrap() {
        storage.remove_entry(&entry.entry_id, "test-device").unwrap();
    }
    assert_eq!(storage.get_all_entries("test-device").unwrap().len(), 0);

    // Import
    let count = storage.import_from_json("test-device", &json).expect("Failed to import");
    assert_eq!(count, 2);

    // Verify
    let imported = storage.get_all_entries("test-device").expect("Failed to get entries");
    assert_eq!(imported.len(), 2);
}

/// Test dictionary with case sensitivity
#[tokio::test]
async fn test_dictionary_case_sensitivity() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_test_device(&db);
    let storage = DictionaryStorage::new(&db);

    // Add case-sensitive entry
    let mut entry = create_test_entry("API", "Application Programming Interface", DictionaryEntryCategory::Technical);
    entry.case_sensitive = true;
    storage.add_entry(&entry).expect("Failed to add entry");

    let entries = storage.get_all_entries("test-device").expect("Failed to get entries");

    // Test case-sensitive replacement
    let text = "The API and api are different things";
    let processed = apply_dictionary_to_text(text, &entries);

    assert!(processed.contains("Application Programming Interface"), "Uppercase API should be replaced");
    assert!(processed.contains("api"), "Lowercase api should NOT be replaced (case-sensitive)");
}

/// Test dictionary with whole-word-only option
#[tokio::test]
async fn test_dictionary_whole_word_only() {
    let db = Database::in_memory().expect("Failed to create database");
    setup_test_device(&db);
    let storage = DictionaryStorage::new(&db);

    // Add whole-word-only entry
    let entry = create_test_entry("app", "application", DictionaryEntryCategory::General);
    storage.add_entry(&entry).expect("Failed to add entry");

    let entries = storage.get_all_entries("test-device").expect("Failed to get entries");

    // Test whole-word replacement
    let text = "I opened the app but not the application";
    let processed = apply_dictionary_to_text(text, &entries);

    assert!(processed.contains("I opened the application"), "Whole word 'app' should be replaced");
    // "application" contains "app" but should not be modified due to word boundary
}

/// Test dictionary device isolation
#[tokio::test]
async fn test_dictionary_device_isolation() {
    let db = Database::in_memory().expect("Failed to create database");

    // Setup two devices
    db.connection().execute(
        "INSERT OR IGNORE INTO device_profiles (device_id, created_at, last_active_at)
         VALUES ('device-1', datetime('now'), datetime('now'))",
        [],
    ).expect("Failed to create device-1");

    db.connection().execute(
        "INSERT OR IGNORE INTO device_profiles (device_id, created_at, last_active_at)
         VALUES ('device-2', datetime('now'), datetime('now'))",
        [],
    ).expect("Failed to create device-2");

    let storage = DictionaryStorage::new(&db);

    // Add different entries for each device
    let entry1 = PersonalDictionaryEntry {
        entry_id: "entry-1".to_string(),
        device_id: "device-1".to_string(),
        phrase: "API".to_string(),
        replacement: "Application Programming Interface".to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    };

    let entry2 = PersonalDictionaryEntry {
        entry_id: "entry-2".to_string(),
        device_id: "device-2".to_string(),
        phrase: "API".to_string(),
        replacement: "Advanced Programming Interface".to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    };

    storage.add_entry(&entry1).expect("Failed to add entry1");
    storage.add_entry(&entry2).expect("Failed to add entry2");

    // Verify isolation
    let entries1 = storage.get_all_entries("device-1").expect("Failed to get entries");
    let entries2 = storage.get_all_entries("device-2").expect("Failed to get entries");

    assert_eq!(entries1.len(), 1);
    assert_eq!(entries2.len(), 1);
    assert_eq!(entries1[0].replacement, "Application Programming Interface");
    assert_eq!(entries2[0].replacement, "Advanced Programming Interface");
}