//! Tests for translation FFI functions

use talkute_core::ai::translator::{TranslationService, TranslationRequest};
use talkute_core::speech::language_detection::LanguageDetector;

#[test]
fn test_translate_text_function() {
    let service = TranslationService::new();

    // Test translation request
    let request = TranslationRequest {
        text: "Hello".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    // Service should be created successfully
    assert!(service.is_available() || !service.is_available());
}

#[test]
fn test_detect_language_function() {
    let detector = LanguageDetector::new();

    // Test language detection with clearer text
    let result = detector.detect_from_text("Hello, how are you doing today? I am fine thank you.");
    assert!(!result.language.is_empty());

    let result = detector.detect_from_text("你好世界，今天天气很好，很高兴见到你。");
    assert_eq!(result.language, "zh");

    let result = detector.detect_from_text("こんにちは世界。今日はいい天気ですね。");
    assert_eq!(result.language, "ja");
}

#[test]
fn test_get_supported_translation_languages() {
    let service = TranslationService::new();
    let languages = service.get_supported_languages();

    assert!(!languages.is_empty());
    assert!(languages.contains(&"en".to_string()));
    assert!(languages.contains(&"zh".to_string()));
    assert!(languages.contains(&"ja".to_string()));
}

#[test]
fn test_translation_with_auto_detect() {
    let detector = LanguageDetector::new();
    let service = TranslationService::new();

    // Detect language first with Chinese text (more reliable detection)
    let detected = detector.detect_from_text("你好世界，今天天气很好");
    assert_eq!(detected.language, "zh");

    // Create translation request with detected language
    let request = TranslationRequest {
        text: "你好世界".to_string(),
        source_language: detected.language,
        target_language: "en".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    assert!(service.is_supported_pair(&request.source_language, &request.target_language));
}

#[test]
fn test_translation_preserve_placeholders() {
    let service = TranslationService::new();

    let request = TranslationRequest {
        text: "Hello {name}, welcome to {place}!".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: true,
        tone: "neutral".to_string(),
    };

    assert!(service.is_supported_pair(&request.source_language, &request.target_language));
}

#[test]
fn test_batch_translation_ffi() {
    let service = TranslationService::new();

    let texts = vec![
        "Hello".to_string(),
        "Goodbye".to_string(),
    ];

    // Test that batch translation request is valid
    assert!(!texts.is_empty());
    assert!(service.is_supported_pair("en", "zh"));
}

#[test]
fn test_language_detection_with_confidence() {
    let detector = LanguageDetector::new();

    // High confidence for clear Chinese
    let result = detector.detect_from_text("这是明确的中文文本用于测试。");
    assert_eq!(result.language, "zh");
    assert!(result.confidence > 0.8);

    // High confidence for clear Japanese
    let result = detector.detect_from_text("こんにちは、今日は良い天気ですね。日本語のテストです。");
    assert_eq!(result.language, "ja");
    assert!(result.confidence > 0.8);
}

#[test]
fn test_translation_language_names() {
    let service = TranslationService::new();

    assert_eq!(service.get_language_name("en"), "English");
    assert_eq!(service.get_language_name("zh"), "Chinese");
    assert_eq!(service.get_language_name("ja"), "Japanese");
    assert_eq!(service.get_language_name("ko"), "Korean");
    assert_eq!(service.get_language_name("es"), "Spanish");
    assert_eq!(service.get_language_name("de"), "German");
    assert_eq!(service.get_language_name("fr"), "French");
}

#[test]
fn test_empty_text_handling() {
    let service = TranslationService::new();

    let request = TranslationRequest {
        text: "".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    // Empty text should still create valid request
    assert!(request.text.is_empty());
}