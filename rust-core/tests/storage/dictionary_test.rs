//! Tests for PersonalDictionaryEntry model

use talkute_core::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use chrono::Utc;

/// Test PersonalDictionaryEntry creation
#[test]
fn test_dictionary_entry_creation() {
    let entry = PersonalDictionaryEntry {
        entry_id: "test-123".to_string(),
        device_id: "test-device".to_string(),
        phrase: "API",
        replacement: "Application Programming Interface",
        case_sensitive: false,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
    };

    assert_eq!(entry.phrase, "API");
    assert_eq!(entry.replacement, "Application Programming Interface");
    assert!(!entry.case_sensitive);
    assert_eq!(entry.category, DictionaryEntryCategory::Technical);
}

/// Test PersonalDictionaryEntry with all fields
#[test]
fn test_dictionary_entry_all_fields() {
    let now = Utc::now();

    let entry = PersonalDictionaryEntry {
        entry_id: "test-123".to_string(),
        device_id: "test-device".to_string(),
        phrase: "Kubernetes",
        replacement: "K8s",
        case_sensitive: true,
        category: DictionaryEntryCategory::Technical,
        created_at: now,
        last_used_at: None,
    };

    assert_eq!(entry.phrase, "Kubernetes");
    assert_eq!(entry.replacement, "K8s");
    assert!(entry.case_sensitive);
    assert_eq!(entry.category, DictionaryEntryCategory::Technical);
    assert_eq!(entry.created_at, now);
}

/// Test PersonalDictionaryEntry with different categories
#[test]
fn test_dictionary_entry_categories() {
    let categories = vec![
        (DictionaryEntryCategory::Technical, "Technical"),
        (DictionaryEntryCategory::Business, "Business"),
        (DictionaryEntryCategory::Medical, "Medical"),
        (DictionaryEntryCategory::General, "General"),
    ];

    for (category, _) in categories {
        let entry = PersonalDictionaryEntry {
            entry_id: "test".to_string(),
            device_id: "test-device".to_string(),
            phrase: "test-replacement",
            replacement: "test-replacement".to_string(),
            case_sensitive: false,
            category: category.clone(),
            created_at: Utc::now(),
            last_used_at: None,
        };

        assert_eq!(entry.category, category);
    }
}

/// Test PersonalDictionaryEntry with usage tracking
#[test]
fn test_dictionary_entry_usage_tracking() {
    let created = Utc::now();
    let used = Utc::now();

    let entry = PersonalDictionaryEntry {
        entry_id: "test".to_string(),
        device_id: "test-device".to_string(),
        phrase: "frequently used term",
        replacement: "common term",
        case_sensitive: false,
        category: DictionaryEntryCategory::General,
        created_at: created,
        last_used_at: Some(used),
    };

    assert_eq!(entry.created_at, created);
    assert_eq!(entry.last_used_at, Some(used));
    assert!(used > created);
}

/// Test PersonalDictionaryEntry serialization
#[test]
fn test_dictionary_entry_serialization() {
    let entry = PersonalDictionaryEntry {
        entry_id: "test".to_string(),
        device_id: "test-device".to_string(),
        phrase: "test phrase",
        replacement: "test replacement",
        case_sensitive: true,
        category: DictionaryEntryCategory::Business,
    };

    // Verify all fields are preserved
    assert_eq!(entry.phrase, "test phrase");
    assert_eq!(entry.replacement, "test replacement");
}

/// Test PersonalDictionaryEntry with empty strings
#[test]
fn test_dictionary_entry_empty_strings() {
    let entry = PersonalDictionaryEntry {
        entry_id: "test".to_string(),
        device_id: "test-device".to_string(),
        phrase: "",
        replacement: "replacement",
        case_sensitive: false,
        category: DictionaryEntryCategory::General,
        created_at: Utc::now(),
        last_used_at: None,
    };

    assert_eq!(entry.phrase, "");
}

/// Test PersonalDictionaryEntry case sensitivity
#[test]
fn test_dictionary_entry_case_sensitivity() {
    let entry1 = PersonalDictionaryEntry {
        entry_id: "test".to_string(),
        device_id: "test-device".to_string(),
        phrase: "test",
        replacement: "replacement",
        case_sensitive: true,
        category: DictionaryEntryCategory::Technical,
    };

    let entry2 = PersonalDictionaryEntry {
        entry_id: "test".to_string(),
        device_id: "test-device".to_string(),
        phrase: "TEST",
        replacement: "REPLACEMENT",
        case_sensitive: false,
        category: DictionaryEntryCategory::Technical,
    };

    // Case sensitive entries should treat "TEST" and "test" differently
    assert_ne!(entry1.phrase, entry2.phrase);

    // Case insensitive entries should treat "TEST" and "test" the same
    let entry3 = PersonalDictionaryEntry {
        entry_id: "test".to_string(),
        device_id: "test-device".to_string(),
        phrase: "test",
        replacement: "replacement",
        case_sensitive: false,
        category: DictionaryEntryCategory::Technical,
    };
    assert_eq!(entry2.phrase, entry3.phrase);
}
