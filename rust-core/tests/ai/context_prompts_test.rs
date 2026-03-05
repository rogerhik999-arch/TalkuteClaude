//! Tests for context-specific AI prompts

use talkute_core::ai::prompts::AIPrompts;

#[test]
fn test_prompts_creation() {
    let prompts = AIPrompts::new();
    assert!(!prompts.default_polish.is_empty());
}

#[test]
fn test_email_prompt_contains_formal() {
    let prompts = AIPrompts::new();
    let email_prompt = prompts.for_context("email");

    assert!(email_prompt.to_lowercase().contains("formal") || email_prompt.to_lowercase().contains("email"));
}

#[test]
fn test_chat_prompt_contains_casual() {
    let prompts = AIPrompts::new();
    let chat_prompt = prompts.for_context("chat");

    assert!(chat_prompt.to_lowercase().contains("casual") || chat_prompt.to_lowercase().contains("chat"));
}

#[test]
fn test_document_prompt_contains_professional() {
    let prompts = AIPrompts::new();
    let doc_prompt = prompts.for_context("document");

    assert!(doc_prompt.to_lowercase().contains("professional") || doc_prompt.to_lowercase().contains("document"));
}

#[test]
fn test_code_prompt_contains_technical() {
    let prompts = AIPrompts::new();
    let code_prompt = prompts.for_context("code");

    assert!(code_prompt.to_lowercase().contains("technical") || code_prompt.to_lowercase().contains("code"));
}

#[test]
fn test_browser_prompt_is_accessible() {
    let prompts = AIPrompts::new();
    let browser_prompt = prompts.for_context("browser");

    assert!(browser_prompt.to_lowercase().contains("browser") || browser_prompt.to_lowercase().contains("web"));
}

#[test]
fn test_unknown_context_returns_default() {
    let prompts = AIPrompts::new();
    let default_prompt = prompts.default_polishing();
    let unknown_prompt = prompts.for_context("unknown-context");

    // Unknown contexts should fall back to default
    assert!(!unknown_prompt.is_empty());
}

#[test]
fn test_context_aliases() {
    let prompts = AIPrompts::new();

    // Email aliases - should map to email context
    let outlook_prompt = prompts.for_context("outlook");
    assert!(!outlook_prompt.is_empty());

    let gmail_prompt = prompts.for_context("gmail");
    assert!(!gmail_prompt.is_empty());

    // Chat aliases
    let slack_prompt = prompts.for_context("slack");
    assert!(!slack_prompt.is_empty());

    let discord_prompt = prompts.for_context("discord");
    assert!(!discord_prompt.is_empty());

    // Code alias
    let vscode_prompt = prompts.for_context("vscode");
    assert!(!vscode_prompt.is_empty());
}

#[test]
fn test_prompt_formatting() {
    let prompts = AIPrompts::new();
    let input = "Hello world";
    let formatted = prompts.format(&prompts.default_polish, input);

    assert!(formatted.contains(input));
}

#[test]
fn test_translation_prompt() {
    let prompts = AIPrompts::new();
    let translation_prompt = prompts.translation("Spanish");

    assert!(translation_prompt.contains("Spanish"));
    assert!(translation_prompt.contains("Translate"));
}

#[test]
fn test_all_contexts_have_prompts() {
    let prompts = AIPrompts::new();

    let contexts = vec!["email", "chat", "document", "code", "browser", "general"];

    for context in contexts {
        let prompt = prompts.for_context(context);
        assert!(!prompt.is_empty(), "Context '{}' should have a non-empty prompt", context);
    }
}