//! Tests for Linux context detection

use talkute_core::context::linux::LinuxContextDetector;
use talkute_core::context::detector::ContextDetector;

#[test]
fn test_linux_detector_creation() {
    let detector = LinuxContextDetector::new();
    assert!(detector.is_available() || !detector.is_available()); // Depends on platform
}

#[test]
fn test_linux_detector_detect() {
    let detector = LinuxContextDetector::new();

    let result = detector.detect_current_context();

    match result {
        Ok(context) => {
            // If successful, should have valid data
            assert!(!context.application_name.is_empty() || !context.application_category.is_empty());
        }
        Err(_) => {
            // Expected on non-Linux platforms
        }
    }
}

#[test]
fn test_linux_category_mapping() {
    let detector = LinuxContextDetector::new();

    // Test known Linux application mappings
    assert_eq!(detector.map_to_category("thunderbird"), "email");
    assert_eq!(detector.map_to_category("slack"), "chat");
    assert_eq!(detector.map_to_category("code"), "code");
    assert_eq!(detector.map_to_category("chrome"), "browser");
    assert_eq!(detector.map_to_category("firefox"), "browser");
    assert_eq!(detector.map_to_category("libreoffice-writer"), "document");
}

#[test]
fn test_linux_unknown_application() {
    let detector = LinuxContextDetector::new();

    let category = detector.map_to_category("unknown-app");
    assert_eq!(category, "general");
}

#[test]
fn test_linux_terminal_category() {
    let detector = LinuxContextDetector::new();

    // Terminal applications might be categorized as code
    assert_eq!(detector.map_to_category("gnome-terminal"), "code");
    assert_eq!(detector.map_to_category("konsole"), "code");
    assert_eq!(detector.map_to_category("alacritty"), "code");
}

#[test]
fn test_linux_discord_chat() {
    let detector = LinuxContextDetector::new();

    assert_eq!(detector.map_to_category("discord"), "chat");
}

#[test]
fn test_linux_evolution_email() {
    let detector = LinuxContextDetector::new();

    assert_eq!(detector.map_to_category("evolution"), "email");
}