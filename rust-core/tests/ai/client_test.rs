//! Tests for Claude API client

use std::sync::Mutex;
use talkute_core::ai::client::{ClaudeClient, ClaudeConfig};

// Mutex to prevent parallel test interference with environment variables
static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[test]
fn test_claude_config_creation() {
    let config = ClaudeConfig {
        api_key: "test-api-key".to_string(),
        model: "claude-3-5-sonnet-20241022".to_string(),
        max_tokens: 4096,
        temperature: 0.3,
    };

    assert_eq!(config.api_key, "test-api-key");
    assert_eq!(config.model, "claude-3-5-sonnet-20241022");
    assert_eq!(config.max_tokens, 4096);
    assert!((config.temperature - 0.3).abs() < f32::EPSILON);
}

#[test]
fn test_claude_config_from_env_missing_key() {
    let _lock = ENV_MUTEX.lock().unwrap();
    // Ensure ANTHROPIC_API_KEY is not set
    std::env::remove_var("ANTHROPIC_API_KEY");

    let result = ClaudeConfig::from_env();
    assert!(result.is_err());
}

#[test]
fn test_claude_config_from_env_with_key() {
    let _lock = ENV_MUTEX.lock().unwrap();
    // Set the API key
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");

    let result = ClaudeConfig::from_env();
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.api_key, "test-key");

    // Clean up
    std::env::remove_var("ANTHROPIC_API_KEY");
}

#[test]
fn test_claude_config_default_model() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");
    std::env::remove_var("CLAUDE_MODEL");

    let config = ClaudeConfig::from_env().unwrap();
    assert_eq!(config.model, "claude-3-5-sonnet-20241022");

    std::env::remove_var("ANTHROPIC_API_KEY");
}

#[test]
fn test_claude_config_custom_model() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");
    std::env::set_var("CLAUDE_MODEL", "claude-3-opus-20240229");

    let config = ClaudeConfig::from_env().unwrap();
    assert_eq!(config.model, "claude-3-opus-20240229");

    std::env::remove_var("ANTHROPIC_API_KEY");
    std::env::remove_var("CLAUDE_MODEL");
}

#[test]
fn test_claude_client_creation() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");

    let result = ClaudeClient::new();
    assert!(result.is_ok());

    std::env::remove_var("ANTHROPIC_API_KEY");
}

// Integration test - requires actual API key
#[tokio::test]
#[ignore]
async fn test_claude_client_complete() {
    let client = ClaudeClient::new().expect("Failed to create client");

    let result = client
        .complete("You are a helpful assistant.", "Say 'Hello, World!'")
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.is_empty());
}