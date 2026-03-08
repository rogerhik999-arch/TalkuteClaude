//! Dictionary application logic
//!
//! Applies personal dictionary entries to transcribed text.

use crate::storage::models::PersonalDictionaryEntry;
use regex::Regex;

/// Apply dictionary entries to text
///
/// This function replaces phrases in the text according to dictionary entries.
/// Longer, more specific phrases are applied first to avoid partial matches.
pub fn apply_dictionary_to_text(text: &str, entries: &[PersonalDictionaryEntry]) -> String {
    if entries.is_empty() {
        return text.to_string();
    }

    // Sort entries by phrase length (descending) to handle longer phrases first
    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort_by(|a, b| b.phrase.len().cmp(&a.phrase.len()));

    let mut result = text.to_string();

    for entry in sorted_entries {
        if entry.phrase.is_empty() {
            continue;
        }

        if entry.whole_word_only {
            // Use word boundary matching for whole-word-only entries
            let pattern = if entry.case_sensitive {
                format!(r"\b{}\b", regex::escape(&entry.phrase))
            } else {
                format!(r"(?i)\b{}\b", regex::escape(&entry.phrase))
            };

            if let Ok(re) = Regex::new(&pattern) {
                result = re.replace_all(&result, entry.replacement.as_str()).to_string();
            }
        } else {
            // Simple substring replacement
            if entry.case_sensitive {
                result = result.replace(&entry.phrase, &entry.replacement);
            } else {
                // Case-insensitive replacement
                let pattern = format!(r"(?i){}", regex::escape(&entry.phrase));
                if let Ok(re) = Regex::new(&pattern) {
                    result = re.replace_all(&result, entry.replacement.as_str()).to_string();
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::models::DictionaryEntryCategory;
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

    #[test]
    fn test_simple_replacement() {
        let text = "I need to API the server";
        let entries = vec![create_test_entry("API", "Application Programming Interface", false, true)];
        let result = apply_dictionary_to_text(text, &entries);
        assert_eq!(result, "I need to Application Programming Interface the server");
    }

    #[test]
    fn test_case_insensitive_replacement() {
        let text = "The api and API are the same";
        let entries = vec![create_test_entry("API", "Application Programming Interface", false, true)];
        let result = apply_dictionary_to_text(text, &entries);
        assert!(result.contains("Application Programming Interface"));
    }

    #[test]
    fn test_case_sensitive_replacement() {
        let text = "The api and API are different";
        let entries = vec![create_test_entry("API", "Application Programming Interface", true, true)];
        let result = apply_dictionary_to_text(text, &entries);
        assert!(result.contains("api")); // lowercase should remain
        assert!(result.contains("Application Programming Interface")); // uppercase replaced
    }

    #[test]
    fn test_multiple_entries() {
        let text = "Use the API and CI/CD pipeline";
        let entries = vec![
            create_test_entry("API", "Application Programming Interface", false, true),
            create_test_entry("CI/CD", "Continuous Integration/Continuous Deployment", true, false),
        ];
        let result = apply_dictionary_to_text(text, &entries);
        assert!(result.contains("Application Programming Interface"));
        assert!(result.contains("Continuous Integration/Continuous Deployment"));
    }

    #[test]
    fn test_longer_phrase_priority() {
        let text = "Use the API endpoint";
        let entries = vec![
            create_test_entry("API", "Application Programming Interface", false, false),
            create_test_entry("API endpoint", "service endpoint", false, false),
        ];
        let result = apply_dictionary_to_text(text, &entries);
        // Longer phrase should be applied first
        assert!(result.contains("service endpoint"));
    }

    #[test]
    fn test_no_entries() {
        let text = "No changes here";
        let entries: Vec<PersonalDictionaryEntry> = vec![];
        let result = apply_dictionary_to_text(text, &entries);
        assert_eq!(result, text);
    }

    #[test]
    fn test_empty_phrase() {
        let text = "Some text";
        let entries = vec![create_test_entry("", "replacement", false, true)];
        let result = apply_dictionary_to_text(text, &entries);
        assert_eq!(result, text); // Empty phrase should not cause changes
    }
}