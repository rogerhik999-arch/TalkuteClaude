//! Tests for multi-language filler word removal

use talkute_core::processing::filler_removal::FillerWords;

#[test]
fn test_english_filler_words() {
    let filler = FillerWords::new();

    // English filler words
    assert!(filler.is_filler("um", "en"));
    assert!(filler.is_filler("uh", "en"));
    assert!(filler.is_filler("like", "en"));
    assert!(filler.is_filler("you know", "en"));
    assert!(filler.is_filler("actually", "en"));
    assert!(filler.is_filler("basically", "en"));
    assert!(filler.is_filler("literally", "en"));
    assert!(filler.is_filler("so", "en"));
    assert!(filler.is_filler("well", "en"));
    assert!(filler.is_filler("right", "en"));
}

#[test]
fn test_chinese_filler_words() {
    let filler = FillerWords::new();

    // Chinese filler words
    assert!(filler.is_filler("嗯", "zh"));
    assert!(filler.is_filler("啊", "zh"));
    assert!(filler.is_filler("这个", "zh"));
    assert!(filler.is_filler("那个", "zh"));
    assert!(filler.is_filler("就是", "zh"));
    assert!(filler.is_filler("然后", "zh"));
    assert!(filler.is_filler("其实", "zh"));
}

#[test]
fn test_spanish_filler_words() {
    let filler = FillerWords::new();

    // Spanish filler words
    assert!(filler.is_filler("eh", "es"));
    assert!(filler.is_filler("este", "es"));
    assert!(filler.is_filler("pues", "es"));
    assert!(filler.is_filler("o sea", "es"));
    assert!(filler.is_filler("bueno", "es"));
    assert!(filler.is_filler("vale", "es"));
}

#[test]
fn test_japanese_filler_words() {
    let filler = FillerWords::new();

    // Japanese filler words
    assert!(filler.is_filler("えっと", "ja"));
    assert!(filler.is_filler("あの", "ja"));
    assert!(filler.is_filler("その", "ja"));
    assert!(filler.is_filler("まあ", "ja"));
    assert!(filler.is_filler("なんか", "ja"));
}

#[test]
fn test_german_filler_words() {
    let filler = FillerWords::new();

    // German filler words
    assert!(filler.is_filler("äh", "de"));
    assert!(filler.is_filler("öhm", "de"));
    assert!(filler.is_filler("also", "de"));
    assert!(filler.is_filler("halt", "de"));
    assert!(filler.is_filler("eben", "de"));
}

#[test]
fn test_french_filler_words() {
    let filler = FillerWords::new();

    // French filler words
    assert!(filler.is_filler("euh", "fr"));
    assert!(filler.is_filler("ben", "fr"));
    assert!(filler.is_filler("du coup", "fr"));
    assert!(filler.is_filler("en fait", "fr"));
    assert!(filler.is_filler("quoi", "fr"));
}

#[test]
fn test_korean_filler_words() {
    let filler = FillerWords::new();

    // Korean filler words
    assert!(filler.is_filler("음", "ko"));
    assert!(filler.is_filler("그", "ko"));
    assert!(filler.is_filler("저", "ko"));
    assert!(filler.is_filler("뭐냐", "ko"));
}

#[test]
fn test_remove_english_fillers() {
    let filler = FillerWords::new();

    let text = "I wanted to um schedule a meeting uh tomorrow";
    let result = filler.remove_fillers(text, "en");

    assert!(!result.contains("um"));
    assert!(!result.contains("uh"));
    assert!(result.contains("schedule"));
    assert!(result.contains("meeting"));
}

#[test]
fn test_remove_chinese_fillers() {
    let filler = FillerWords::new();

    let text = "我想嗯安排一个会议啊明天";
    let result = filler.remove_fillers(text, "zh");

    assert!(!result.contains("嗯"));
    assert!(!result.contains("啊"));
    assert!(result.contains("安排"));
    assert!(result.contains("会议"));
}

#[test]
fn test_remove_japanese_fillers() {
    let filler = FillerWords::new();

    let text = "あの明日えっと会議があります";
    let result = filler.remove_fillers(text, "ja");

    assert!(!result.contains("えっと"));
    assert!(!result.contains("あの"));
    assert!(result.contains("会議"));
}

#[test]
fn test_preserve_content_words() {
    let filler = FillerWords::new();

    // Make sure content words are not removed
    let text = "I really like this product";
    let result = filler.remove_fillers(text, "en");

    // "like" as a filler should be removed, but context matters
    // In this case it's part of "really like" which is content
    assert!(result.contains("really") || result.contains("product"));
}

#[test]
fn test_mixed_language_text() {
    let filler = FillerWords::new();

    // Mixed language text - should only remove fillers for specified language
    let text = "Hello 嗯 world";
    let result = filler.remove_fillers(text, "en");

    // Chinese filler should remain when processing as English
    assert!(result.contains("嗯"));

    // Now process as Chinese
    let result = filler.remove_fillers(text, "zh");
    assert!(!result.contains("嗯"));
}

#[test]
fn test_get_filler_list_for_language() {
    let filler = FillerWords::new();

    // Get list of fillers for each language
    let en_fillers = filler.get_fillers_for_language("en");
    assert!(!en_fillers.is_empty());
    assert!(en_fillers.contains(&"um".to_string()));

    let zh_fillers = filler.get_fillers_for_language("zh");
    assert!(!zh_fillers.is_empty());
    assert!(zh_fillers.contains(&"嗯".to_string()));

    let ja_fillers = filler.get_fillers_for_language("ja");
    assert!(!ja_fillers.is_empty());
    assert!(ja_fillers.contains(&"えっと".to_string()));
}

#[test]
fn test_unsupported_language_fallback() {
    let filler = FillerWords::new();

    // Unsupported language should return empty list or fallback to English
    let fillers = filler.get_fillers_for_language("xx");
    // Should not panic, may return empty or English fillers
    assert!(fillers.is_empty() || fillers.contains(&"um".to_string()));
}