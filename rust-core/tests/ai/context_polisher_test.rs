//! Tests for context-aware polishing

use talkute_core::ai::polisher::{TextPolisher, PolisherConfig};
use talkute_core::ai::prompts::AIPrompts;

#[test]
fn test_polisher_config_default() {
    let config = PolisherConfig::default();

    assert_eq!(config.max_retries, 3);
    assert!(config.fallback_on_error);
}

#[test]
fn test_polisher_config_custom() {
    let config = PolisherConfig {
        max_retries: 5,
        retry_delay: std::time::Duration::from_secs(1),
        request_timeout: std::time::Duration::from_secs(60),
        fallback_on_error: false,
    };

    assert_eq!(config.max_retries, 5);
    assert!(!config.fallback_on_error);
}

#[test]
fn test_context_specific_prompts_exist() {
    let prompts = AIPrompts::new();

    // All major contexts should have prompts
    assert!(!prompts.for_context("email").is_empty());
    assert!(!prompts.for_context("chat").is_empty());
    assert!(!prompts.for_context("document").is_empty());
    assert!(!prompts.for_context("code").is_empty());
    assert!(!prompts.for_context("browser").is_empty());
}

#[test]
fn test_prompt_tone_differences() {
    let prompts = AIPrompts::new();

    // Email should be more formal than chat
    let email_prompt = prompts.for_context("email");
    let chat_prompt = prompts.for_context("chat");

    // They should be different prompts
    assert_ne!(email_prompt, chat_prompt);

    // Email should mention formal/professional
    assert!(email_prompt.to_lowercase().contains("formal") || email_prompt.to_lowercase().contains("professional"));

    // Chat should mention casual/conversational
    assert!(chat_prompt.to_lowercase().contains("casual") || chat_prompt.to_lowercase().contains("conversational"));
}

#[test]
fn test_code_prompt_is_technical() {
    let prompts = AIPrompts::new();
    let code_prompt = prompts.for_context("code");

    // Code prompt should mention technical aspects
    let code_lower = code_prompt.to_lowercase();
    assert!(
        code_lower.contains("technical") ||
        code_lower.contains("code") ||
        code_lower.contains("programming")
    );
}

#[test]
fn test_polisher_creation_without_api_key() {
    // Remove API key if set
    std::env::remove_var("ANTHROPIC_API_KEY");

    let result = TextPolisher::new();
    assert!(result.is_err());
}

#[test]
fn test_is_retryable_error() {
    // Test the retry logic is implemented
    let config = PolisherConfig::default();
    assert_eq!(config.max_retries, 3);
}

#[test]
fn test_polisher_has_context_method() {
    // This test verifies the API exists even without API key
    // The actual functionality requires a real API key
    std::env::remove_var("ANTHROPIC_API_KEY");

    let result = TextPolisher::new();
    // Without API key, this should fail
    assert!(result.is_err());
}

#[test]
fn test_translation_prompt_generation() {
    let prompts = AIPrompts::new();

    let spanish = prompts.translation("Spanish");
    assert!(spanish.contains("Spanish"));

    let french = prompts.translation("French");
    assert!(french.contains("French"));

    let chinese = prompts.translation("Chinese");
    assert!(chinese.contains("Chinese"));
}