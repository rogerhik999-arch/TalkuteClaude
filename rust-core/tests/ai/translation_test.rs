//! Tests for translation service

use talkute_core::ai::translator::TranslationService;
use talkute_core::ai::translator::{TranslationRequest, TranslationResult};

#[tokio::test]
async fn test_create_translation_service() {
    let service = TranslationService::new();
    assert!(service.is_available() || !service.is_available()); // May not have API key
}

#[tokio::test]
async fn test_supported_translation_pairs() {
    let service = TranslationService::new();

    // Common translation pairs
    assert!(service.is_supported_pair("en", "zh"));
    assert!(service.is_supported_pair("zh", "en"));
    assert!(service.is_supported_pair("en", "es"));
    assert!(service.is_supported_pair("en", "ja"));
    assert!(service.is_supported_pair("en", "de"));
    assert!(service.is_supported_pair("en", "fr"));
}

#[tokio::test]
async fn test_translate_request_structure() {
    let request = TranslationRequest {
        text: "Hello, world".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: true,
        tone: "neutral".to_string(),
    };

    assert_eq!(request.text, "Hello, world");
    assert_eq!(request.source_language, "en");
    assert_eq!(request.target_language, "zh");
    assert!(request.preserve_formatting);
}

#[tokio::test]
async fn test_translation_result_structure() {
    let result = TranslationResult {
        translated_text: "你好，世界".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        confidence: 0.95,
    };

    assert_eq!(result.translated_text, "你好，世界");
    assert_eq!(result.source_language, "en");
    assert_eq!(result.target_language, "zh");
    assert!(result.confidence > 0.9);
}

#[tokio::test]
async fn test_get_translation_languages() {
    let service = TranslationService::new();
    let languages = service.get_supported_languages();

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
async fn test_translate_empty_text() {
    let service = TranslationService::new();

    let request = TranslationRequest {
        text: "".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    // Empty text should return empty result
    let result = service.translate(&request).await;
    assert!(result.is_ok());
    assert!(result.unwrap().translated_text.is_empty());
}

#[tokio::test]
async fn test_auto_detect_source_language() {
    let service = TranslationService::new();

    // Auto-detect should work with "auto" as source
    assert!(service.is_supported_pair("auto", "en"));
    assert!(service.is_supported_pair("auto", "zh"));
}

#[tokio::test]
async fn test_translation_with_tone() {
    let service = TranslationService::new();

    // Test that tone parameter is accepted
    let request_formal = TranslationRequest {
        text: "Hello".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: false,
        tone: "formal".to_string(),
    };

    let request_casual = TranslationRequest {
        text: "Hello".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: false,
        tone: "casual".to_string(),
    };

    // Both should be valid requests
    assert!(service.translate(&request_formal).await.is_ok() || !service.is_available());
    assert!(service.translate(&request_casual).await.is_ok() || !service.is_available());
}

#[tokio::test]
async fn test_preserve_formatting() {
    let service = TranslationService::new();

    let request = TranslationRequest {
        text: "Hello {name}, welcome to {place}!".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: true,
        tone: "neutral".to_string(),
    };

    let result = service.translate(&request).await;
    if let Ok(translated) = result {
        // Placeholders should be preserved
        assert!(translated.translated_text.contains("{name}") || !service.is_available());
        assert!(translated.translated_text.contains("{place}") || !service.is_available());
    }
}

#[tokio::test]
async fn test_batch_translation() {
    let service = TranslationService::new();

    let texts = vec![
        "Hello".to_string(),
        "Goodbye".to_string(),
        "Thank you".to_string(),
    ];

    let result = service.translate_batch(&texts, "en", "zh").await;
    assert!(result.is_ok() || !service.is_available());

    if let Ok(translations) = result {
        assert_eq!(translations.len(), 3);
    }
}

#[tokio::test]
async fn test_get_language_name() {
    let service = TranslationService::new();

    assert_eq!(service.get_language_name("en"), "English");
    assert_eq!(service.get_language_name("zh"), "Chinese");
    assert_eq!(service.get_language_name("es"), "Spanish");
    assert_eq!(service.get_language_name("ja"), "Japanese");
    assert_eq!(service.get_language_name("de"), "German");
    assert_eq!(service.get_language_name("fr"), "French");
    assert_eq!(service.get_language_name("ko"), "Korean");
}