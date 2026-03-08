//! Tests for unified context detector

use talkute_core::context::detector::{UnifiedContextDetector, Platform};

#[tokio::test]
async fn test_unified_detector_creation() {
    let detector = UnifiedContextDetector::new().await.unwrap();

    // Should have detected a platform
    let platform = detector.platform();
    assert!(matches!(platform, Platform::Windows | Platform::Macos | Platform::Linux | Platform::Ios | Platform::Android));
}

#[tokio::test]
async fn test_unified_detector_detect() {
    let detector = UnifiedContextDetector::new().await.unwrap();

    let result = detector.detect().await;
    assert!(result.is_ok());

    let context = result.unwrap();
    assert!(!context.application_name.is_empty());
    assert!(!context.application_category.is_empty());
}

#[tokio::test]
async fn test_unified_detector_categorize() {
    let detector = UnifiedContextDetector::new().await.unwrap();

    // Test various application names
    assert_eq!(detector.categorize_application("Outlook"), "email");
    assert_eq!(detector.categorize_application("Gmail"), "email");
    assert_eq!(detector.categorize_application("Slack"), "chat");
    assert_eq!(detector.categorize_application("Discord"), "chat");
    assert_eq!(detector.categorize_application("VSCode"), "code");
    assert_eq!(detector.categorize_application("Word"), "document");
    assert_eq!(detector.categorize_application("Chrome"), "browser");
}

#[tokio::test]
async fn test_unified_detector_categorize_case_insensitive() {
    let detector = UnifiedContextDetector::new().await.unwrap();

    // Should handle different cases
    assert_eq!(detector.categorize_application("outlook"), "email");
    assert_eq!(detector.categorize_application("OUTLOOK"), "email");
    assert_eq!(detector.categorize_application("OutLook"), "email");
}

#[tokio::test]
async fn test_unified_detector_categorize_partial_match() {
    let detector = UnifiedContextDetector::new().await.unwrap();

    // Should handle partial matches
    assert_eq!(detector.categorize_application("Microsoft Outlook"), "email");
    assert_eq!(detector.categorize_application("VSCode - myproject"), "code");
    assert_eq!(detector.categorize_application("Google Chrome"), "browser");
}

#[tokio::test]
async fn test_unified_detector_categorize_unknown() {
    let detector = UnifiedContextDetector::new().await.unwrap();

    // Unknown applications should return "other"
    assert_eq!(detector.categorize_application("UnknownApp"), "other");
    assert_eq!(detector.categorize_application("Random Application"), "other");
}

#[tokio::test]
async fn test_platform_specific_detector() {
    let detector = UnifiedContextDetector::new().await.unwrap();

    // Get platform-specific detector based on current platform
    #[cfg(target_os = "windows")]
    {
        let win_detector = detector.get_windows_detector();
        assert!(win_detector.is_available());
    }

    #[cfg(target_os = "macos")]
    {
        let mac_detector = detector.get_macos_detector();
        assert!(mac_detector.is_available());
    }

    #[cfg(target_os = "linux")]
    {
        let linux_detector = detector.get_linux_detector();
        assert!(linux_detector.is_available());
    }

    #[cfg(target_os = "ios")]
    {
        let ios_detector = detector.get_ios_detector();
        assert!(ios_detector.is_available());
    }

    #[cfg(target_os = "android")]
    {
        let android_detector = detector.get_android_detector();
        assert!(android_detector.is_available());
    }
}

#[tokio::test]
async fn test_detector_async_detect() {
    let detector = UnifiedContextDetector::new().await.unwrap();

    // Test that the async detect method works
    let result = detector.detect().await;
    assert!(result.is_ok());
}