//! Tests for pipeline with dictionary integration

use talkute_core::processing::TextProcessingPipeline;
use talkute_core::processing::dictionary::apply_dictionary_to_text;
use talkute_core::storage::models::{PersonalDictionaryEntry, DictionaryEntryCategory};
use chrono::Utc;

fn create_entry(phrase: &str, replacement: &str) -> PersonalDictionaryEntry {
    PersonalDictionaryEntry {
        entry_id: "test".to_string(),
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

/// Test pipeline with dictionary application
#[test]
fn test_pipeline_with_dictionary() {
    let pipeline = TextProcessingPipeline::new();
    let text = "I need to um API the server";

    // First apply pipeline (filler removal)
    let processed = pipeline.process(text);
    assert!(!processed.contains("um"));

    // Then apply dictionary
    let entries = vec![create_entry("API", "Application Programming Interface")];
    let result = apply_dictionary_to_text(&processed, &entries);

    assert!(result.contains("Application Programming Interface"));
    assert!(!result.contains("API"));
}

/// Test dictionary applied before processing
#[test]
fn test_dictionary_before_pipeline() {
    let pipeline = TextProcessingPipeline::new();
    let text = "I need to um check the API endpoint";

    // Apply dictionary first
    let entries = vec![create_entry("API endpoint", "service endpoint")];
    let dict_applied = apply_dictionary_to_text(text, &entries);

    // Then apply pipeline
    let result = pipeline.process(&dict_applied);

    assert!(result.contains("service endpoint"));
    assert!(!result.contains("um"));
}

/// Test combined workflow: filler removal + self-correction + dictionary
#[test]
fn test_combined_workflow() {
    let pipeline = TextProcessingPipeline::new();
    let text = "I wanted to um deploy to the API no wait to the REST API";

    // Apply pipeline (filler removal + self-correction)
    let processed = pipeline.process(text);

    // Verify filler removal
    assert!(!processed.contains("um"));

    // Apply dictionary
    let entries = vec![
        create_entry("REST API", "RESTful Application Programming Interface"),
    ];
    let result = apply_dictionary_to_text(&processed, &entries);

    // Dictionary should be applied
    assert!(result.contains("RESTful Application Programming Interface"));
}

/// Test multiple dictionary entries with pipeline
#[test]
fn test_multiple_dictionary_entries_with_pipeline() {
    let pipeline = TextProcessingPipeline::new();
    let text = "I need to um integrate the API with CI/CD";

    let entries = vec![
        create_entry("API", "Application Programming Interface"),
        create_entry("CI/CD", "Continuous Integration/Continuous Deployment"),
    ];

    // Apply pipeline then dictionary
    let processed = pipeline.process(text);
    let result = apply_dictionary_to_text(&processed, &entries);

    assert!(result.contains("Application Programming Interface"));
    assert!(result.contains("Continuous Integration/Continuous Deployment"));
    assert!(!result.contains("um"));
}

/// Test dictionary with empty text
#[test]
fn test_dictionary_with_empty_text() {
    let pipeline = TextProcessingPipeline::new();
    let text = "";

    let entries = vec![create_entry("API", "Application Programming Interface")];

    let processed = pipeline.process(text);
    let result = apply_dictionary_to_text(&processed, &entries);

    assert_eq!(result, "");
}

/// Test pipeline preserves dictionary replacements
#[test]
fn test_pipeline_preserves_dictionary_replacements() {
    let pipeline = TextProcessingPipeline::new();
    let text = "I need to API the server";

    // Apply dictionary first
    let entries = vec![create_entry("API", "Application Programming Interface")];
    let dict_applied = apply_dictionary_to_text(text, &entries);

    // Pipeline should not undo the dictionary replacement
    let result = pipeline.process(&dict_applied);

    assert!(result.contains("Application Programming Interface"));
    assert!(!result.contains(" API "));
}