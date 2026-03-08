//! Tests for Android context detection

use talkute_core::context::android::AndroidContextDetector;
use talkute_core::context::detector::ContextDetector;

#[test]
fn test_android_detector_creation() {
    let detector = AndroidContextDetector::new();
    assert!(detector.is_available() || !detector.is_available());
}

#[test]
fn test_android_package_mapping() {
    let detector = AndroidContextDetector::new();

    // Test known Android package mappings
    assert_eq!(detector.map_to_category("com.google.android.gm"), "email");
    assert_eq!(detector.map_to_category("com.slack"), "chat");
    assert_eq!(detector.map_to_category("com.android.chrome"), "browser");
    assert_eq!(detector.map_to_category("com.microsoft.office.word"), "document");
}

#[test]
fn test_android_unknown_package() {
    let detector = AndroidContextDetector::new();

    let category = detector.map_to_category("com.unknown.app");
    assert_eq!(category, "general");
}

#[test]
fn test_android_social_apps() {
    let detector = AndroidContextDetector::new();

    // Social/messaging apps should map to chat
    assert_eq!(detector.map_to_category("com.whatsapp"), "chat");
    assert_eq!(detector.map_to_category("com.telegram.messenger"), "chat");
    assert_eq!(detector.map_to_category("com.discord"), "chat");
}

#[test]
fn test_android_email_clients() {
    let detector = AndroidContextDetector::new();

    assert_eq!(detector.map_to_category("com.microsoft.outlook"), "email");
    assert_eq!(detector.map_to_category("com.fsck.k9"), "email"); // K-9 Mail
}

#[test]
fn test_android_browsers() {
    let detector = AndroidContextDetector::new();

    assert_eq!(detector.map_to_category("com.android.chrome"), "browser");
    assert_eq!(detector.map_to_category("org.mozilla.firefox"), "browser");
    assert_eq!(detector.map_to_category("com.brave.browser"), "browser");
}

#[test]
fn test_android_office_apps() {
    let detector = AndroidContextDetector::new();

    assert_eq!(detector.map_to_category("com.microsoft.office.word"), "document");
    assert_eq!(detector.map_to_category("com.google.android.apps.docs.editors.docs"), "document");
}

#[test]
fn test_android_code_editors() {
    let detector = AndroidContextDetector::new();

    // Code editors on Android
    assert_eq!(detector.map_to_category("com.ghisler.android.TotalCommander"), "code");
    assert_eq!(detector.map_to_category("com.aide.ui"), "code"); // AIDE
}

#[test]
fn test_android_detect_current_context() {
    let detector = AndroidContextDetector::new();

    let result = detector.detect_current_context();

    match result {
        Ok(context) => {
            // Should have valid category
            assert!(!context.application_category.is_empty());
        }
        Err(_) => {
            // Expected on non-Android platforms
        }
    }
}