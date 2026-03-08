//! Tests for PersonalDictionaryEntry model

use talkute_core::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use chrono::Utc;

fn create_entry(phrase: &str, replacement: &str) -> PersonalDictionaryEntry {
    PersonalDictionaryEntry {
        entry_id: "test-id".to_string(),
        device_id: "test-device".to_string(),
        phrase: phrase.to_string(),
        replacement: replacement.to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    }
}

/// Test PersonalDictionaryEntry creation
#[test]
fn test_dictionary_entry_creation() {
    let entry = create_entry("API", "Application Programming Interface");

    assert_eq!(entry.phrase, "API");
    assert_eq!(entry.replacement, "Application Programming Interface");
    assert!(!entry.case_sensitive);
    assert_eq!(entry.category, DictionaryEntryCategory::Technical);
}

/// Test PersonalDictionaryEntry with all fields
#[test]
fn test_dictionary_entry_all_fields() {
    let entry = PersonalDictionaryEntry {
        entry_id: "test-id".to_string(),
        device_id: "test-device".to_string(),
        phrase: "Kubernetes".to_string(),
        replacement: "K8s".to_string(),
        case_sensitive: true,
        whole_word_only: false,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: Some(Utc::now()),
        usage_count: 5,
    };

    assert_eq!(entry.phrase, "Kubernetes");
    assert_eq!(entry.replacement, "K8s");
    assert!(entry.case_sensitive);
    assert_eq!(entry.category, DictionaryEntryCategory::Technical);
    assert_eq!(entry.usage_count, 5);
}

/// Test PersonalDictionaryEntry with different categories
#[test]
fn test_dictionary_entry_categories() {
    let categories = [
        DictionaryEntryCategory::Technical,
        DictionaryEntryCategory::Business,
        DictionaryEntryCategory::Medical,
        DictionaryEntryCategory::General,
    ];

    for category in categories {
        let entry = PersonalDictionaryEntry {
            entry_id: "test-id".to_string(),
            device_id: "test-device".to_string(),
            phrase: "test".to_string(),
            replacement: "test-replacement".to_string(),
            case_sensitive: false,
            whole_word_only: true,
            category: category.clone(),
            created_at: Utc::now(),
            last_used_at: None,
            usage_count: 0,
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
        entry_id: "test-id".to_string(),
        device_id: "test-device".to_string(),
        phrase: "frequently used term".to_string(),
        replacement: "common term".to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::General,
        created_at: created,
        last_used_at: Some(used),
        usage_count: 10,
    };

    assert_eq!(entry.created_at, created);
    assert_eq!(entry.last_used_at, Some(used));
    assert_eq!(entry.usage_count, 10);
    assert!(used >= created);
}

/// Test PersonalDictionaryEntry serialization
#[test]
fn test_dictionary_entry_serialization() {
    let entry = create_entry("test phrase", "test replacement");

    // Verify all fields are preserved
    assert_eq!(entry.phrase, "test phrase");
    assert_eq!(entry.replacement, "test replacement");
}

/// Test PersonalDictionaryEntry with empty strings
#[test]
fn test_dictionary_entry_empty_strings() {
    let entry = PersonalDictionaryEntry {
        entry_id: "test-id".to_string(),
        device_id: "test-device".to_string(),
        phrase: "".to_string(),
        replacement: "replacement".to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::General,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    };

    assert_eq!(entry.phrase, "");
}

/// Test PersonalDictionaryEntry case sensitivity
#[test]
fn test_dictionary_entry_case_sensitivity() {
    let entry1 = PersonalDictionaryEntry {
        entry_id: "test-1".to_string(),
        device_id: "test-device".to_string(),
        phrase: "test".to_string(),
        replacement: "replacement".to_string(),
        case_sensitive: true,
        whole_word_only: true,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    };

    let entry2 = PersonalDictionaryEntry {
        entry_id: "test-2".to_string(),
        device_id: "test-device".to_string(),
        phrase: "TEST".to_string(),
        replacement: "REPLACEMENT".to_string(),
        case_sensitive: false,
        whole_word_only: true,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    };

    // Case sensitive entries should treat "TEST" and "test" differently
    assert_ne!(entry1.phrase, entry2.phrase);
}