//! Tests for dictionary application logic

use talkute_core::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use talkute_core::storage::dictionary::apply_dictionary_to_text;
use chrono::Utc;

fn create_test_entry(phrase: &str, replacement: &str, case_sensitive: bool, whole_word_only: bool) -> PersonalDictionaryEntry {
    PersonalDictionaryEntry {
        entry_id: "test".to_string(),
        device_id: "test-device".to_string(),
        phrase: phrase.to_string(),
        replacement: replacement.to_string(),
        case_sensitive,
        whole_word_only,
        category: DictionaryEntryCategory::Technical,
        created_at: Utc::now(),
        last_used_at: None,
        usage_count: 0,
    }
}

/// Test dictionary application to transcription
#[test]
fn test_apply_dictionary_to_transcription_simple() {
    let text = "I need to API the server";
    let entries = vec![create_test_entry("API", "Application Programming Interface", false, true)];
    let result = apply_dictionary_to_text(text, &entries);
    assert_eq!(result, "I need to Application Programming Interface the server");
}

/// Test dictionary application with multiple entries
#[test]
fn test_apply_dictionary_multiple_entries() {
    let text = "Let's sync with the API and check the CI/CD";

    let entries = vec![
        create_test_entry("API", "Application Programming Interface", false, true),
        create_test_entry("CI/CD", "Continuous Integration/Continuous Deployment", true, false),
    ];

    let result = apply_dictionary_to_text(&text, &entries);

    assert!(result.contains("Application Programming Interface"));
    assert!(result.contains("Continuous Integration/Continuous Deployment"));
}

/// Test dictionary application with case sensitivity
#[test]
fn test_apply_dictionary_case_sensitive() {
    let text = "I'm deploying to the CI environment";

    let entry = create_test_entry("CI", "Continuous Integration", true, true);
    let result = apply_dictionary_to_text(&text, &[entry]);

    assert_eq!(result, "I'm deploying to the Continuous Integration environment");
}

/// Test dictionary application with whole word only
#[test]
fn test_apply_dictionary_whole_word_only() {
    let text = "The k8s pod is running";

    let entry = create_test_entry("k8s", "Kubernetes", false, true);
    let result = apply_dictionary_to_text(&text, &[entry]);

    assert!(result.contains("Kubernetes"));
}

/// Test dictionary application with no matching entries
#[test]
fn test_apply_dictionary_no_matches() {
    let text = "This is a generic message without special terms";

    let entry = create_test_entry("API", "Application Programming Interface", false, true);
    let result = apply_dictionary_to_text(&text, &[entry]);

    assert_eq!(result, text);
}

/// Test dictionary application with overlapping entries
#[test]
fn test_apply_dictionary_overlapping() {
    let text = "The API endpoint accepts JSON input";

    let entries = vec![
        create_test_entry("API", "Application Programming Interface", false, false),
        create_test_entry("API endpoint", "endpoint", true, false),
    ];

    let result = apply_dictionary_to_text(&text, &entries);

    // Longer phrase should take priority
    assert!(result.contains("endpoint"));
}

/// Test dictionary application with category context
#[test]
fn test_apply_dictionary_with_category() {
    let text = "I'll use the React framework for this project";

    let entry = create_test_entry("React", "React.js", false, false);
    let result = apply_dictionary_to_text(&text, &[entry]);

    assert_eq!(result, "I'll use the React.js framework for this project");
}