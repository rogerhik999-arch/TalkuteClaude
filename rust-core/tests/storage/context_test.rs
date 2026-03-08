//! Tests for ApplicationContext model

use talkute_core::storage::models::ApplicationContext;
use chrono::Utc;

#[test]
fn test_application_context_creation() {
    let context = ApplicationContext::new(
        "ctx-123".to_string(),
        "Outlook".to_string(),
        "email".to_string(),
    );

    assert_eq!(context.context_id, "ctx-123");
    assert_eq!(context.application_name, "Outlook");
    assert_eq!(context.application_category, "email");
    assert!(context.application_title.is_none());
}

#[test]
fn test_application_context_with_title() {
    let mut context = ApplicationContext::new(
        "ctx-456".to_string(),
        "Slack".to_string(),
        "chat".to_string(),
    );

    context.application_title = Some("Engineering Team".to_string());

    assert_eq!(context.application_title, Some("Engineering Team".to_string()));
}

#[test]
fn test_application_context_usage_count() {
    let context = ApplicationContext::new(
        "ctx-789".to_string(),
        "VSCode".to_string(),
        "code".to_string(),
    );

    // Initial usage count should be 1
    assert_eq!(context.usage_count, 1);
}

#[test]
fn test_application_context_timestamps() {
    let before = Utc::now();
    let context = ApplicationContext::new(
        "ctx-abc".to_string(),
        "Chrome".to_string(),
        "browser".to_string(),
    );
    let after = Utc::now();

    // detected_at should be between before and after
    assert!(context.detected_at >= before);
    assert!(context.detected_at <= after);

    // last_used_at should equal detected_at initially
    assert_eq!(context.detected_at, context.last_used_at);
}

#[test]
fn test_application_context_preferred_tone() {
    let mut context = ApplicationContext::new(
        "ctx-def".to_string(),
        "Gmail".to_string(),
        "email".to_string(),
    );

    context.preferred_tone = Some("formal".to_string());

    assert_eq!(context.preferred_tone, Some("formal".to_string()));
}

#[test]
fn test_application_context_custom_instructions() {
    let mut context = ApplicationContext::new(
        "ctx-ghi".to_string(),
        "Word".to_string(),
        "document".to_string(),
    );

    context.custom_instructions = Some("Use professional business tone".to_string());

    assert_eq!(
        context.custom_instructions,
        Some("Use professional business tone".to_string())
    );
}

#[test]
fn test_application_context_categories() {
    // Test different application categories
    let categories = vec![
        ("email", "Outlook"),
        ("chat", "Slack"),
        ("document", "Word"),
        ("code", "VSCode"),
        ("browser", "Chrome"),
    ];

    for (category, app_name) in categories {
        let context = ApplicationContext::new(
            format!("ctx-{}", category),
            app_name.to_string(),
            category.to_string(),
        );

        assert_eq!(context.application_category, category);
        assert_eq!(context.application_name, app_name);
    }
}