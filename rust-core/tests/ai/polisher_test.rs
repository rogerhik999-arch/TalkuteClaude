//! Tests for text polishing service

use std::sync::Mutex;
use talkute_core::ai::polisher::TextPolisher;

// Mutex to prevent parallel test interference with environment variables
static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_polisher_creation_without_api_key() {
    let _lock = ENV_MUTEX.lock().unwrap();
    // Ensure no API key is set
    std::env::remove_var("ANTHROPIC_API_KEY");

    let result = TextPolisher::new();
    // Should fail without API key
    assert!(result.is_err());
}

#[test]
fn test_polisher_creation_with_api_key() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");

    let result = TextPolisher::new();
    assert!(result.is_ok());

    std::env::remove_var("ANTHROPIC_API_KEY");
}

// Integration test - requires actual API key
#[tokio::test]
#[ignore]
async fn test_polisher_polish() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");

    let polisher = TextPolisher::new().expect("Failed to create polisher");

    let result = polisher.polish("I wanted to um schedule a meeting").await;
    assert!(result.is_ok());

    let polished = result.unwrap();
    // Should not contain filler words
    assert!(!polished.to_lowercase().contains(" um "));

    std::env::remove_var("ANTHROPIC_API_KEY");
}

// Integration test - requires actual API key
#[tokio::test]
#[ignore]
async fn test_polisher_polish_with_context() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");

    let polisher = TextPolisher::new().expect("Failed to create polisher");

    let result = polisher
        .polish_with_context("I want to meet tomorrow", "email")
        .await;
    assert!(result.is_ok());

    std::env::remove_var("ANTHROPIC_API_KEY");
}

// Integration test - requires actual API key
#[tokio::test]
#[ignore]
async fn test_polisher_translate() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");

    let polisher = TextPolisher::new().expect("Failed to create polisher");

    let result = polisher.translate("Hello, world!", "Spanish").await;
    assert!(result.is_ok());

    std::env::remove_var("ANTHROPIC_API_KEY");
}