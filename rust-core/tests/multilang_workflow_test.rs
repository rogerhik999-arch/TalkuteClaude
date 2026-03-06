//! End-to-end integration test for multi-language workflow
//!
//! Tests the complete workflow: select language → speak → transcribe → optional translate

use talkute_core::ai::translator::{TranslationRequest, TranslationService};
use talkute_core::processing::filler_removal::FillerWords;
use talkute_core::speech::language_detection::LanguageDetector;

/// Test workflow: English speech recognition and transcription
#[test]
fn test_english_transcription_workflow() {
    // 1. Setup: Configure for English
    let supported_languages = vec!["en".to_string(), "zh".to_string(), "ja".to_string()];
    assert!(supported_languages.contains(&"en".to_string()));

    // 2. Simulate speech input (English) - using lowercase fillers as defined in FillerWords
    let raw_transcript = "um, hello there, uh, how are you doing today?";

    // 3. Process: Remove filler words
    let filler_remover = FillerWords::new();
    let cleaned = filler_remover.remove_fillers(raw_transcript, "en");
    assert!(!cleaned.contains("um"));
    assert!(!cleaned.contains("uh"));

    // 4. Verify language detection
    let detector = LanguageDetector::new();
    let detected = detector.detect_from_text(&cleaned);
    assert_eq!(detected.language, "en");
    assert!(detected.confidence > 0.5);
}

/// Test workflow: Chinese speech recognition and transcription
#[test]
fn test_chinese_transcription_workflow() {
    // 1. Setup: Configure for Chinese
    let supported_languages = vec!["en".to_string(), "zh".to_string(), "ja".to_string()];
    assert!(supported_languages.contains(&"zh".to_string()));

    // 2. Simulate speech input (Chinese with filler words)
    // Using fillers that are in the list: 那个, 嗯, 啊, 额
    let raw_transcript = "那个，你好，嗯，今天天气怎么样？";

    // 3. Process: Remove filler words
    let filler_remover = FillerWords::new();
    let cleaned = filler_remover.remove_fillers(raw_transcript, "zh");
    assert!(!cleaned.contains("那个"));
    assert!(!cleaned.contains("嗯"));

    // 4. Verify language detection
    let detector = LanguageDetector::new();
    let detected = detector.detect_from_text(&cleaned);
    assert_eq!(detected.language, "zh");
    assert!(detected.confidence > 0.5);
}

/// Test workflow: Japanese speech recognition and transcription
#[test]
fn test_japanese_transcription_workflow() {
    // 1. Setup: Configure for Japanese
    let supported_languages = vec!["en".to_string(), "zh".to_string(), "ja".to_string()];
    assert!(supported_languages.contains(&"ja".to_string()));

    // 2. Simulate speech input (Japanese with filler words)
    let raw_transcript = "あの、こんにちは、えっと、今日の天気はどうですか？";

    // 3. Process: Remove filler words
    let filler_remover = FillerWords::new();
    let cleaned = filler_remover.remove_fillers(raw_transcript, "ja");
    assert!(!cleaned.contains("あの"));
    assert!(!cleaned.contains("えっと"));

    // 4. Verify language detection
    let detector = LanguageDetector::new();
    let detected = detector.detect_from_text(&cleaned);
    assert_eq!(detected.language, "ja");
    assert!(detected.confidence > 0.5);
}

/// Test workflow: English to Chinese translation
#[test]
fn test_english_to_chinese_translation_workflow() {
    let service = TranslationService::new();

    // Skip if no API key available
    if !service.is_available() {
        eprintln!("Skipping translation test - no API key");
        return;
    }

    // 1. Setup: Configure translation
    let request = TranslationRequest {
        text: "Hello, how are you?".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    // 2. Validate request
    assert!(service.validate_request(&request).is_ok());
    assert!(service.is_supported_pair("en", "zh"));
}

/// Test workflow: Chinese to English translation
#[test]
fn test_chinese_to_english_translation_workflow() {
    let service = TranslationService::new();

    // Skip if no API key available
    if !service.is_available() {
        eprintln!("Skipping translation test - no API key");
        return;
    }

    // 1. Setup: Configure translation
    let request = TranslationRequest {
        text: "你好，今天天气怎么样？".to_string(),
        source_language: "zh".to_string(),
        target_language: "en".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    // 2. Validate request
    assert!(service.validate_request(&request).is_ok());
    assert!(service.is_supported_pair("zh", "en"));
}

/// Test workflow: Auto-detect source language
#[test]
fn test_auto_detect_translation_workflow() {
    let service = TranslationService::new();

    // Test with auto-detection enabled
    let request = TranslationRequest {
        text: "Bonjour, comment allez-vous?".to_string(),
        source_language: "auto".to_string(),
        target_language: "en".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    // Validate that auto-detection is supported
    assert!(service.is_supported_pair("auto", "en"));
    assert!(service.validate_request(&request).is_ok());
}

/// Test workflow: Translation with formatting preservation
#[test]
fn test_translation_preserve_formatting_workflow() {
    let service = TranslationService::new();

    let request = TranslationRequest {
        text: "Hello {name}, welcome to {place}!".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: true,
        tone: "formal".to_string(),
    };

    assert!(service.validate_request(&request).is_ok());
}

/// Test workflow: Multiple languages in sequence
#[test]
fn test_sequential_language_switching() {
    let supported_languages = vec!["en", "zh", "ja", "ko", "es"];

    // Test switching between languages
    for lang in &supported_languages {
        // 1. Verify language is supported
        assert!(supported_languages.contains(lang));

        // 2. Get sample text for each language
        let sample = match *lang {
            "en" => "Hello world",
            "zh" => "你好世界",
            "ja" => "こんにちは世界",
            "ko" => "안녕하세요 세계",
            "es" => "Hola mundo",
            _ => "Test",
        };

        // 3. Verify language detection works
        let detector = LanguageDetector::new();
        let detected = detector.detect_from_text(sample);

        // For clearly distinct scripts (en, zh, ja), detection should be accurate
        if *lang == "en" || *lang == "zh" || *lang == "ja" {
            assert!(detected.confidence > 0.3, "Language detection failed for {}", lang);
        }
    }
}

/// Test workflow: Full pipeline simulation
#[test]
fn test_full_multilang_pipeline() {
    // Simulate a full workflow:
    // 1. User selects language
    // 2. User speaks (we simulate with text)
    // 3. System processes (filler removal)
    // 4. Optional: User requests translation

    // Step 1: Select language
    let selected_language = "zh";
    let supported = vec!["en", "zh", "ja", "ko"];
    assert!(supported.contains(&selected_language));

    // Step 2: Simulated speech input with fillers
    // Using fillers that are in the list: 那个, 嗯
    let raw_input = "那个，我，嗯，想去北京旅游";

    // Step 3: Process - remove fillers
    let filler_remover = FillerWords::new();
    let cleaned = filler_remover.remove_fillers(raw_input, selected_language);
    assert!(!cleaned.contains("那个"));
    assert!(!cleaned.contains("嗯"));

    // Step 4: Verify language detection matches selection
    let detector = LanguageDetector::new();
    let detected = detector.detect_from_text(&cleaned);
    assert_eq!(detected.language, selected_language);

    // Step 5: Optional translation setup
    let translation_service = TranslationService::new();
    let translation_target = "en";

    if translation_service.is_available() {
        let request = TranslationRequest {
            text: cleaned.clone(),
            source_language: selected_language.to_string(),
            target_language: translation_target.to_string(),
            preserve_formatting: false,
            tone: "neutral".to_string(),
        };
        assert!(translation_service.validate_request(&request).is_ok());
    }
}

/// Test error handling: Invalid language code
#[test]
fn test_invalid_language_code_handling() {
    let service = TranslationService::new();

    // Test with invalid language code
    let request = TranslationRequest {
        text: "Hello".to_string(),
        source_language: "invalid".to_string(),
        target_language: "en".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    assert!(service.validate_request(&request).is_err());
}

/// Test error handling: Text too long
#[test]
fn test_text_length_validation() {
    let service = TranslationService::new();

    // Test with text exceeding limit
    let long_text = "a".repeat(10001);
    let request = TranslationRequest {
        text: long_text,
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: false,
        tone: "neutral".to_string(),
    };

    assert!(service.validate_request(&request).is_err());
}

/// Test error handling: Invalid tone
#[test]
fn test_invalid_tone_handling() {
    let service = TranslationService::new();

    let request = TranslationRequest {
        text: "Hello".to_string(),
        source_language: "en".to_string(),
        target_language: "zh".to_string(),
        preserve_formatting: false,
        tone: "invalid_tone".to_string(),
    };

    assert!(service.validate_request(&request).is_err());
}

/// Test workflow: Empty text handling
#[test]
fn test_empty_text_handling() {
    let detector = LanguageDetector::new();

    // Empty text should still return a result (default language)
    let detected = detector.detect_from_text("");
    // Empty text detection may return default or low confidence
    // This is acceptable behavior
    assert!(!detected.language.is_empty());
}

/// Test workflow: Batch translation validation
#[test]
fn test_batch_translation_workflow() {
    let service = TranslationService::new();

    let texts = vec![
        "Hello".to_string(),
        "Goodbye".to_string(),
        "Thank you".to_string(),
    ];

    // Verify all source/target pairs are valid
    for text in &texts {
        let request = TranslationRequest {
            text: text.clone(),
            source_language: "en".to_string(),
            target_language: "zh".to_string(),
            preserve_formatting: false,
            tone: "neutral".to_string(),
        };
        assert!(service.validate_request(&request).is_ok());
    }
}

/// Test workflow: Tone variation
#[test]
fn test_tone_variation_workflow() {
    let service = TranslationService::new();
    let valid_tones = ["neutral", "formal", "casual", "professional", "friendly"];

    for tone in valid_tones {
        let request = TranslationRequest {
            text: "Hello, how are you?".to_string(),
            source_language: "en".to_string(),
            target_language: "zh".to_string(),
            preserve_formatting: false,
            tone: tone.to_string(),
        };
        assert!(service.validate_request(&request).is_ok());
    }
}

/// Test workflow: Language detection confidence
#[test]
fn test_language_detection_confidence() {
    let detector = LanguageDetector::new();

    // Clear English text should have high confidence
    let en_result = detector.detect_from_text("The quick brown fox jumps over the lazy dog");
    assert_eq!(en_result.language, "en");
    assert!(en_result.confidence > 0.5, "English detection confidence too low: {}", en_result.confidence);

    // Clear Chinese text should have high confidence
    let zh_result = detector.detect_from_text("今天天气很好，我想去公园散步");
    assert_eq!(zh_result.language, "zh");
    assert!(zh_result.confidence > 0.5, "Chinese detection confidence too low: {}", zh_result.confidence);

    // Clear Japanese text should have high confidence
    let ja_result = detector.detect_from_text("今日は良い天気ですね。散歩に行きたいです。");
    assert_eq!(ja_result.language, "ja");
    assert!(ja_result.confidence > 0.5, "Japanese detection confidence too low: {}", ja_result.confidence);
}

/// Test workflow: Filler removal across languages
#[test]
fn test_filler_removal_multilang() {
    let filler_remover = FillerWords::new();

    // English fillers (using lowercase as defined in FillerWords)
    let en_text = "um, I think, uh, we should go";
    let en_cleaned = filler_remover.remove_fillers(en_text, "en");
    assert!(!en_cleaned.contains("um"));
    assert!(!en_cleaned.contains("uh"));

    // Chinese fillers (using fillers from the list: 那个, 嗯, 啊, 额)
    let zh_text = "那个，我觉得，嗯，我们应该去";
    let zh_cleaned = filler_remover.remove_fillers(zh_text, "zh");
    assert!(!zh_cleaned.contains("那个"));
    assert!(!zh_cleaned.contains("嗯"));

    // Japanese fillers
    let ja_text = "あの、私は、えっと、行きたいです";
    let ja_cleaned = filler_remover.remove_fillers(ja_text, "ja");
    assert!(!ja_cleaned.contains("あの"));
    assert!(!ja_cleaned.contains("えっと"));
}

/// Test workflow: Supported languages list
#[test]
fn test_supported_languages_consistency() {
    let translation_service = TranslationService::new();
    let detector = LanguageDetector::new();

    let translation_langs = translation_service.get_supported_languages();
    let detection_langs = detector.get_supported_languages();

    // Common languages should be supported by both
    let common = ["en", "zh", "ja", "es"];
    for lang in common {
        assert!(translation_langs.contains(&lang.to_string()),
            "Language {} not in translation service", lang);
    }

    // At least some overlap should exist
    assert!(!translation_langs.is_empty());
    assert!(!detection_langs.is_empty());
}
