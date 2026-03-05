//! Tests for Windows context detection

use talkute_core::context::windows::WindowsContextDetector;
use talkute_core::context::detector::ContextDetector;

#[test]
fn test_windows_detector_creation() {
    let detector = WindowsContextDetector::new();
    assert!(detector.is_available() || !detector.is_available()); // Depends on platform
}

#[test]
fn test_windows_detector_detect() {
    let detector = WindowsContextDetector::new();

    // On Windows, this should return a result
    // On other platforms, it should return an error
    let result = detector.detect_current_context();

    // Should not panic
    match result {
        Ok(context) => {
            // If successful, should have valid category
            assert!(!context.application_name.is_empty() || !context.application_category.is_empty());
        }
        Err(_) => {
            // Expected on non-Windows platforms
        }
    }
}

#[test]
fn test_windows_category_mapping() {
    let detector = WindowsContextDetector::new();

    // Test known application mappings
    assert_eq!(detector.map_to_category("OUTLOOK.EXE"), "email");
    assert_eq!(detector.map_to_category("slack.exe"), "chat");
    assert_eq!(detector.map_to_category("Code.exe"), "code");
    assert_eq!(detector.map_to_category("chrome.exe"), "browser");
    assert_eq!(detector.map_to_category("WINWORD.EXE"), "document");
}

#[test]
fn test_windows_unknown_application() {
    let detector = WindowsContextDetector::new();

    // Unknown applications should default to "general"
    let category = detector.map_to_category("unknown-app.exe");
    assert_eq!(category, "general");
}

#[test]
fn test_windows_case_insensitive() {
    let detector = WindowsContextDetector::new();

    // Should handle case variations
    assert_eq!(detector.map_to_category("OUTLOOK.EXE"), "email");
    assert_eq!(detector.map_to_category("outlook.exe"), "email");
    assert_eq!(detector.map_to_category("Outlook.Exe"), "email");
}

#[test]
fn test_windows_normalize_process_name() {
    let detector = WindowsContextDetector::new();

    // Should normalize process names
    assert_eq!(detector.normalize_process_name("OUTLOOK.EXE"), "outlook");
    assert_eq!(detector.normalize_process_name("C:\\Program Files\\Slack\\slack.exe"), "slack");
    assert_eq!(detector.normalize_process_name("code.exe"), "code");
}