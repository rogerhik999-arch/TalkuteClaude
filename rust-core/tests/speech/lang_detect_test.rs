//! Tests for automatic language detection

use talkute_core::speech::client::AzureSpeechClient;
use talkute_core::speech::language_detection::LanguageDetector;

#[tokio::test]
async fn test_detect_language_from_text() {
    let detector = LanguageDetector::new();

    // English text
    let en_result = detector.detect_from_text("Hello, how are you doing today?");
    assert_eq!(en_result.language, "en");
    assert!(en_result.confidence > 0.5);

    // Chinese text
    let zh_result = detector.detect_from_text("你好，今天天气怎么样？");
    assert_eq!(zh_result.language, "zh");
    assert!(zh_result.confidence > 0.8);

    // Spanish text
    let es_result = detector.detect_from_text("Hola, ¿cómo estás hoy?");
    assert_eq!(es_result.language, "es");
    assert!(es_result.confidence > 0.5);

    // Japanese text
    let ja_result = detector.detect_from_text("こんにちは、お元気ですか？");
    assert_eq!(ja_result.language, "ja");
    assert!(ja_result.confidence > 0.8);
}

#[tokio::test]
async fn test_detect_language_short_text() {
    let detector = LanguageDetector::new();

    // Short texts - detection may be less reliable
    let result = detector.detect_from_text("Hello");
    assert!(!result.language.is_empty());

    let result = detector.detect_from_text("你好");
    assert_eq!(result.language, "zh");
}

#[tokio::test]
async fn test_detect_language_mixed_script() {
    let detector = LanguageDetector::new();

    // Mixed script (e.g., "Hello 世界") should detect dominant or first language
    let result = detector.detect_from_text("Hello 世界");
    assert!(!result.language.is_empty());
}

#[tokio::test]
async fn test_detect_language_empty_text() {
    let detector = LanguageDetector::new();

    // Empty text should return default with low confidence
    let result = detector.detect_from_text("");
    assert_eq!(result.language, "en"); // Default fallback
    assert!(result.confidence < 0.5);
}

#[tokio::test]
async fn test_get_supported_languages() {
    let detector = LanguageDetector::new();
    let languages = detector.get_supported_languages();

    // Should include major languages
    assert!(languages.contains(&"en".to_string()));
    assert!(languages.contains(&"zh".to_string()));
    assert!(languages.contains(&"es".to_string()));
    assert!(languages.contains(&"ja".to_string()));
    assert!(languages.contains(&"de".to_string()));
    assert!(languages.contains(&"fr".to_string()));
    assert!(languages.contains(&"ko".to_string()));
}

#[tokio::test]
async fn test_language_confidence_levels() {
    let detector = LanguageDetector::new();

    // Very clear Chinese should have high confidence
    let clear_zh = detector.detect_from_text("这是一个用于测试的中文句子。语音识别语言检测功能需要准确的识别结果。");
    assert_eq!(clear_zh.language, "zh");
    assert!(clear_zh.confidence > 0.9);

    // Very clear Japanese should have high confidence
    let clear_ja = detector.detect_from_text("こんにちは、今日は良い天気ですね。日本語のテストです。");
    assert_eq!(clear_ja.language, "ja");
    assert!(clear_ja.confidence > 0.9);
}

#[tokio::test]
async fn test_detect_with_hints() {
    let detector = LanguageDetector::new();

    // Detect with language hints (narrow down candidates)
    let hints = vec!["en".to_string(), "zh".to_string()];
    let result = detector.detect_with_hints("Hello world", &hints);
    assert!(result.language == "en" || result.language == "zh");
}

#[tokio::test]
async fn test_continuous_detection() {
    let detector = LanguageDetector::new();

    // Multiple Chinese segments should give consistent results
    let segments = vec![
        "你好",
        "今天天气很好",
        "很高兴见到你",
    ];

    let results: Vec<_> = segments.iter()
        .map(|s| detector.detect_from_text(s))
        .collect();

    // All should detect Chinese
    for result in results {
        assert_eq!(result.language, "zh");
    }
}

#[tokio::test]
async fn test_language_detector_singleton() {
    // Language detector should be cheap to create or use singleton
    let detector1 = LanguageDetector::new();
    let detector2 = LanguageDetector::new();

    // Both should give same results
    let result1 = detector1.detect_from_text("你好世界");
    let result2 = detector2.detect_from_text("你好世界");

    assert_eq!(result1.language, result2.language);
}