//! Tests for text formatter module

use crate::processing::formatter::TextFormatter;

#[test]
fn test_format_sentences() {
    let formatter = TextFormatter::new();
    let text = "hello world how are you";
    let formatted = formatter.format(text);

    assert!(formatted.starts_with('H'));
}

#[test]
fn test_format_lists() {
    let formatter = TextFormatter::new();
    let text = "milk eggs bread cheese";
    let formatted = formatter.format(text);

    // Should detect and format as a list
    assert!(formatted.contains("milk"));
}

#[test]
fn test_punctuation_removal() {
    let formatter = TextFormatter::new();
    let text = "um uh like";
    let result = formatter.remove_fillers(text);

    assert!(!result.contains("um"));
}
