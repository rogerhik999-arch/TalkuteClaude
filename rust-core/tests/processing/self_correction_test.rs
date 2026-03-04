//! Tests for self-correction detection module

use crate::processing::self_correction::SelfCorrectionDetector;

#[test]
fn test_detect_self_correction() {
    let detector = SelfCorrectionDetector::new();
    let text = "I wanted to I need to";
    let corrections = detector.detect(text);

    // Should detect the correction pattern
    assert!(!corrections.is_empty());
}

#[test]
fn test_apply_corrections() {
    let detector = SelfCorrectionDetector::new();
    let text = "I want to I need to";
    let result = detector.apply_corrections(text);

    // Should apply the correction
    assert!(result.contains("I need to"));
}

#[test]
fn test_enabled_flag() {
    let mut detector = SelfCorrectionDetector::new();
    detector.set_enabled(false);

    let corrections = detector.detect("I want to I need to");
    assert!(corrections.is_empty());
}
