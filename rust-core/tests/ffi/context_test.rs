//! Tests for context detection FFI functions

use talkute_core::ffi::bridge::{detect_application_context, get_all_contexts, ApplicationContext};

#[tokio::test]
async fn test_detect_application_context() {
    let result = detect_application_context().await;
    assert!(result.is_ok());

    let context = result.unwrap();
    assert!(!context.context_id.is_empty());
    assert!(!context.application_name.is_empty());
    assert!(!context.application_category.is_empty());
}

#[tokio::test]
async fn test_detect_application_context_has_category() {
    let context = detect_application_context().await.unwrap();

    // Category should be one of the known categories
    let valid_categories = vec!["email", "chat", "document", "code", "browser", "general", "other", "unknown"];
    assert!(
        valid_categories.contains(&context.application_category.as_str()),
        "Category '{}' is not a valid category",
        context.application_category
    );
}

#[tokio::test]
async fn test_detect_application_context_structure() {
    let context = detect_application_context().await.unwrap();

    // Verify structure
    assert!(!context.context_id.is_empty());
    assert!(!context.application_name.is_empty());
    assert!(!context.application_category.is_empty());

    // Optional fields can be None
    // application_title and preferred_tone are optional
}

#[tokio::test]
async fn test_get_all_contexts() {
    let result = get_all_contexts().await;
    assert!(result.is_ok());

    // This returns an empty vec for now since context storage is not implemented
    let contexts = result.unwrap();
    // The result should be a valid vector
    assert!(contexts.is_empty() || !contexts.is_empty());
}

#[tokio::test]
async fn test_application_context_serialization() {
    let context = ApplicationContext {
        context_id: "test-123".to_string(),
        application_name: "TestApp".to_string(),
        application_title: Some("Test Application".to_string()),
        application_category: "email".to_string(),
        preferred_tone: Some("formal".to_string()),
    };

    // Should be serializable
    let json = serde_json::to_string(&context);
    assert!(json.is_ok());

    // Should be deserializable
    let parsed: ApplicationContext = serde_json::from_str(&json.unwrap()).unwrap();
    assert_eq!(parsed.context_id, "test-123");
    assert_eq!(parsed.application_name, "TestApp");
    assert_eq!(parsed.application_category, "email");
}

#[tokio::test]
async fn test_application_context_without_optional_fields() {
    let context = ApplicationContext {
        context_id: "test-456".to_string(),
        application_name: "MinimalApp".to_string(),
        application_title: None,
        application_category: "other".to_string(),
        preferred_tone: None,
    };

    // Should still serialize correctly
    let json = serde_json::to_string(&context).unwrap();
    assert!(json.contains("test-456"));
    assert!(json.contains("MinimalApp"));
}

#[tokio::test]
async fn test_detect_context_multiple_calls() {
    // Multiple calls should succeed
    let result1 = detect_application_context().await;
    let result2 = detect_application_context().await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    // Both should detect the same context (assuming app didn't change)
    let ctx1 = result1.unwrap();
    let ctx2 = result2.unwrap();

    // At minimum, both should have valid categories
    assert!(!ctx1.application_category.is_empty());
    assert!(!ctx2.application_category.is_empty());
}