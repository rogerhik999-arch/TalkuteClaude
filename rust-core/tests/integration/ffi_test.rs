//! FFI integration tests

use talkute_core::ffi::bridge::*;

#[tokio::test]
async fn test_start_stop_voice_session() {
    let session_id = start_voice_session("test-device", None).await;
    assert!(session_id.is_ok());

    let id = session_id.unwrap();
    assert!(!id.is_empty());

    let result = stop_voice_session(&id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_cancel_voice_session() {
    let session_id = start_voice_session("test-device", None).await.unwrap();
    let result = cancel_voice_session(&session_id, Some("User cancelled")).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_session_status() {
    let session_id = start_voice_session("test-device", None).await.unwrap();
    let status = get_session_status(&session_id).await;
    assert!(status.is_ok());

    stop_voice_session(&session_id).await.unwrap();
}

#[tokio::test]
async fn test_detect_application_context() {
    let context = detect_application_context().await;
    // May fail in headless environment, but should not panic
    let _ = context;
}

#[tokio::test]
async fn test_get_quota_usage() {
    let quota = get_quota_usage().await;
    assert!(quota.is_ok());

    let quota_value = quota.unwrap();
    assert!(quota_value.is_object());
}

#[tokio::test]
async fn test_check_quota_available() {
    let available = check_quota_available(100).await;
    assert!(available.is_ok());
}

#[tokio::test]
async fn test_get_or_create_device_profile() {
    let profile = get_or_create_device_profile().await;
    assert!(profile.is_ok());
}

#[tokio::test]
async fn test_run_migrations() {
    let result = run_migrations().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_schema_version() {
    let version = get_schema_version().await;
    assert!(version.is_ok());
    assert!(version.unwrap() > 0);
}

#[tokio::test]
async fn test_get_preference() {
    let value = get_preference("test_key".to_string()).await;
    assert!(value.is_ok());
}

#[tokio::test]
async fn test_set_preference() {
    let result = set_preference("test_key".to_string(), "test_value".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_history() {
    let history = list_history(10, 0).await;
    assert!(history.is_ok());
}

#[tokio::test]
async fn test_clear_all_history() {
    let result = clear_all_history().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_export_data() {
    let result = export_data("json".to_string()).await;
    assert!(result.is_ok());
}

#[test]
fn test_set_tray_icon() {
    let result = set_tray_icon("idle".to_string());
    assert!(result.is_ok());

    let result = set_tray_icon("recording".to_string());
    assert!(result.is_ok());

    let result = set_tray_icon("processing".to_string());
    assert!(result.is_ok());

    let result = set_tray_icon("error".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_show_tray_notification() {
    let result = show_tray_notification("Test Title".to_string(), "Test Message".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_register_global_hotkey() {
    let result = register_global_hotkey("Ctrl+Space".to_string());
    // May fail if not on a desktop environment
    let _ = result;
}

#[test]
fn test_get_current_hotkey() {
    let result = get_current_hotkey();
    assert!(result.is_ok());
}

#[test]
fn test_inject_text_at_cursor() {
    let result = inject_text_at_cursor("Test text".to_string());
    // May fail in headless environment
    let _ = result;
}

#[test]
fn test_copy_to_clipboard() {
    let result = copy_to_clipboard("Test text".to_string());
    assert!(result.is_ok());
}