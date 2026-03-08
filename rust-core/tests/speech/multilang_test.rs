//! Tests for multi-language Azure Speech API client

use talkute_core::speech::client::AzureSpeechClient;

#[tokio::test]
async fn test_create_client_with_config() {
    let client = AzureSpeechClient::with_config(
        "test-key".to_string(),
        "test-region".to_string(),
        None,
    );

    // Client should be created successfully
    assert!(client.is_configured());
}

#[tokio::test]
async fn test_supported_languages() {
    let client = AzureSpeechClient::with_config(
        "test-key".to_string(),
        "test-region".to_string(),
        None,
    );

    // Get list of supported languages
    let languages = client.get_supported_languages();

    // Should include major languages
    assert!(languages.contains(&"en-US".to_string()), "Should support English");
    assert!(languages.contains(&"zh-CN".to_string()), "Should support Chinese");
    assert!(languages.contains(&"es-ES".to_string()), "Should support Spanish");
    assert!(languages.contains(&"ja-JP".to_string()), "Should support Japanese");
    assert!(languages.contains(&"de-DE".to_string()), "Should support German");
    assert!(languages.contains(&"fr-FR".to_string()), "Should support French");
}

#[tokio::test]
async fn test_set_recognition_language() {
    let mut client = AzureSpeechClient::with_config(
        "test-key".to_string(),
        "test-region".to_string(),
        None,
    );

    // Set language to Chinese
    client.set_recognition_language("zh-CN");
    assert_eq!(client.get_recognition_language(), "zh-CN");

    // Set language to English
    client.set_recognition_language("en-US");
    assert_eq!(client.get_recognition_language(), "en-US");
}

#[tokio::test]
async fn test_validate_language_code() {
    let client = AzureSpeechClient::with_config(
        "test-key".to_string(),
        "test-region".to_string(),
        None,
    );

    // Valid language codes
    assert!(client.is_valid_language("en-US"));
    assert!(client.is_valid_language("zh-CN"));
    assert!(client.is_valid_language("es-ES"));
    assert!(client.is_valid_language("ja-JP"));

    // Invalid language codes
    assert!(!client.is_valid_language("invalid"));
    assert!(!client.is_valid_language("xx-XX"));
    assert!(!client.is_valid_language(""));
}

#[tokio::test]
async fn test_language_fallback() {
    let mut client = AzureSpeechClient::with_config(
        "test-key".to_string(),
        "test-region".to_string(),
        None,
    );

    // Set invalid language should fall back to default
    client.set_recognition_language("invalid");
    assert_eq!(client.get_recognition_language(), "en-US"); // Default fallback
}

#[tokio::test]
async fn test_multilingual_recognition_config() {
    let client = AzureSpeechClient::with_config(
        "test-key".to_string(),
        "test-region".to_string(),
        None,
    );

    // Test that multilingual recognition can be enabled
    let config = client.create_recognition_config(Some("zh-CN"), true);
    assert!(config.is_ok());

    // Config should include language setting
    let config = config.unwrap();
    assert_eq!(config["language"], "zh-CN");
}

#[tokio::test]
async fn test_continuous_recognition_with_language() {
    let client = AzureSpeechClient::with_config(
        "test-key".to_string(),
        "test-region".to_string(),
        None,
    );

    // Test continuous recognition setup with specific language
    let result = client.setup_continuous_recognition("zh-CN");
    assert!(result.is_ok()); // Should succeed with valid language
}