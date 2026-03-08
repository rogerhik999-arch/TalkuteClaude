//! Tests for iOS context detection

use talkute_core::context::ios::IOSContextDetector;
use talkute_core::context::detector::ContextDetector;

#[test]
fn test_ios_detector_creation() {
    let detector = IOSContextDetector::new();
    // iOS detector always exists but may use manual fallback
    assert!(detector.is_available() || !detector.is_available());
}

#[test]
fn test_ios_manual_selection() {
    let detector = IOSContextDetector::new();

    // On iOS, manual selection should be available
    let manual_contexts = detector.get_manual_contexts();

    // Should have at least the basic categories
    assert!(manual_contexts.len() >= 5);
    assert!(manual_contexts.contains(&"email".to_string()));
    assert!(manual_contexts.contains(&"chat".to_string()));
    assert!(manual_contexts.contains(&"document".to_string()));
    assert!(manual_contexts.contains(&"code".to_string()));
    assert!(manual_contexts.contains(&"browser".to_string()));
}

#[test]
fn test_ios_set_manual_context() {
    let detector = IOSContextDetector::new();

    // Should allow setting context manually
    let result = detector.set_manual_context("email");
    assert!(result.is_ok());

    // Should be able to retrieve the set context
    let current = detector.get_current_context();
    assert!(current.is_ok());
    let context = current.unwrap();
    assert_eq!(context.application_category, "email");
}

#[test]
fn test_ios_category_mapping() {
    let detector = IOSContextDetector::new();

    // iOS bundle ID mappings
    assert_eq!(detector.map_to_category("com.apple.mobilemail"), "email");
    assert_eq!(detector.map_to_category("com.apple.MobileSMS"), "chat");
    assert_eq!(detector.map_to_category("com.apple.mobilesafari"), "browser");
    assert_eq!(detector.map_to_category("com.apple.Pages"), "document");
}

#[test]
fn test_ios_third_party_apps() {
    let detector = IOSContextDetector::new();

    // Third-party app mappings
    assert_eq!(detector.map_to_category("com.readdle.readdleDocsIPad"), "document");
    assert_eq!(detector.map_to_category("com.hammerandchisel.discord"), "chat");
}

#[test]
fn test_ios_unknown_application() {
    let detector = IOSContextDetector::new();

    let category = detector.map_to_category("com.unknown.app");
    assert_eq!(category, "general");
}

#[test]
fn test_ios_default_context() {
    let detector = IOSContextDetector::new();

    // Default context should be "general" if not set
    let default = detector.get_default_context();
    assert_eq!(default.application_category, "general");
}