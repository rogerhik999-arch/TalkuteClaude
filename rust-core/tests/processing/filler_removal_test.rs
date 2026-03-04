//! Tests for filler word removal module

use crate::processing::filler_removal::FillerWords;

#[test]
fn test_remove_english_fillers() {
    let filler = FillerWords::new();
    let text = "I wanted to um schedule a meeting for tomorrow";
    let cleaned = filler.remove_fillers(text, "en");

    assert!(!cleaned.contains("um"));
}

#[test]
fn test_remove_chinese_fillers() {
    let filler = FillerWords::new();
    let text = "我想嗯安排一个会议";
    let cleaned = filler.remove_fillers(text, "zh");

    assert!(!cleaned.contains("嗯"));
}

#[test]
fn test_is_filler_detection() {
    let filler = FillerWords::new();

    assert!(filler.is_filler("um", "en"));
    assert!(filler.is_filler("uh", "en"));
    assert!(filler.is_filler("嗯", "zh"));
}
