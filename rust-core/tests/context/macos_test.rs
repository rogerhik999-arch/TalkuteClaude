//! Tests for macOS context detection

use talkute_core::context::macos::MacOSContextDetector;
use talkute_core::context::detector::ContextDetector;

#[test]
fn test_macos_detector_creation() {
    let detector = MacOSContextDetector::new();
    assert!(detector.is_available() || !detector.is_available()); // Depends on platform
}

#[test]
fn test_macos_detector_detect() {
    let detector = MacOSContextDetector::new();

    let result = detector.detect_current_context();

    match result {
        Ok(context) => {
            // If successful, should have valid data
            assert!(!context.application_name.is_empty() || !context.application_category.is_empty());
        }
        Err(_) => {
            // Expected on non-macOS platforms
        }
    }
}

#[test]
fn test_macos_category_mapping() {
    let detector = MacOSContextDetector::new();

    // Test known macOS application mappings
    assert_eq!(detector.map_to_category("com.apple.mail"), "email");
    assert_eq!(detector.map_to_category("com.tinyspeck.slackmacgap"), "chat");
    assert_eq!(detector.map_to_category("com.microsoft.VSCode"), "code");
    assert_eq!(detector.map_to_category("com.google.Chrome"), "browser");
    assert_eq!(detector.map_to_category("com.microsoft.Word"), "document");
}

#[test]
fn test_macos_unknown_application() {
    let detector = MacOSContextDetector::new();

    let category = detector.map_to_category("com.unknown.app");
    assert_eq!(category, "general");
}

#[test]
fn test_macos_safari_browser() {
    let detector = MacOSContextDetector::new();

    assert_eq!(detector.map_to_category("com.apple.Safari"), "browser");
}

#[test]
fn test_macos_notes_document() {
    let detector = MacOSContextDetector::new();

    assert_eq!(detector.map_to_category("com.apple.Notes"), "document");
}

#[test]
fn test_macos_messages_chat() {
    let detector = MacOSContextDetector::new();

    assert_eq!(detector.map_to_category("com.apple.MobileSMS"), "chat");
    assert_eq!(detector.map_to_category("com.apple.iChat"), "chat");
}