//! FFI bridge between Rust and Flutter
//!
//! This module defines the function signatures that are exposed to Flutter via flutter_rust_bridge.
//! All functions are async and return Results for proper error handling.

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Represents the current state of a voice session
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Idle,
    Recording,
    Transcribing,
    Polishing,
    Completed,
    Failed,
    Cancelled,
}

/// Voice session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSessionInfo {
    pub session_id: String,
    pub status: SessionStatus,
    pub started_at: String,
    pub duration_seconds: i64,
    pub word_count: i32,
}

/// Application context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationContext {
    pub context_id: String,
    pub application_name: String,
    pub application_title: Option<String>,
    pub application_category: String,
    pub preferred_tone: Option<String>,
}

/// Error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
    pub details: Option<String>,
}

// ============================================================================
// Session Management Functions
// ============================================================================

/// Start a new voice input session
///
/// # Arguments
/// * `device_id` - The device identifier
/// * `context_id` - Optional application context ID
///
/// # Returns
/// * `session_id` - UUID of the new session
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn start_voice_session(device_id: &str, context_id: Option<String>) -> Result<String> {
    todo!("Implement session management")
}

/// Stop the current voice session
///
/// # Arguments
/// * `session_id` - The session to stop
///
/// # Returns
/// * `session_info` - Information about the completed session
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn stop_voice_session(session_id: &str) -> Result<VoiceSessionInfo> {
    todo!("Implement session management")
}

/// Cancel the current voice session
///
/// # Arguments
/// * `session_id` - The session to cancel
/// * `reason` - Optional cancellation reason
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn cancel_voice_session(session_id: &str, reason: Option<&str>) -> Result<()> {
    todo!("Implement session management")
}

/// Get the current session status
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_session_status(session_id: &str) -> Result<SessionStatus> {
    todo!("Implement session management")
}

// ============================================================================
// Audio Processing Functions
// ============================================================================

/// Start audio capture from the microphone
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn start_audio_capture(device_id: &str) -> Result<String> {
    todo!("Implement audio capture")
}

/// Stop audio capture
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn stop_audio_capture(capture_id: &str) -> Result<()> {
    todo!("Implement audio capture")
}

/// Get audio level (0.0 - 1.0)
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_audio_level(capture_id: &str) -> Result<f64> {
    todo!("Implement audio capture")
}

// ============================================================================
// Transcription & AI Functions
// ============================================================================

/// Transcribe audio to text
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn transcribe_audio(
    session_id: &str,
    audio_path: &str,
    language: Option<&str>,
) -> Result<String> {
    todo!("Implement speech recognition")
}

/// Polish text using AI
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn polish_text(
    session_id: &str,
    text: &str,
    context_id: Option<&str>,
) -> Result<String> {
    todo!("Implement AI polishing")
}

/// Get raw transcription for a session
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_raw_transcription(session_id: &str) -> Result<String> {
    todo!("Implement session data access")
}

/// Get polished text for a session
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_polished_text(session_id: &str) -> Result<String> {
    todo!("Implement session data access")
}

// ============================================================================
// Context Detection Functions
// ============================================================================

/// Detect the current application context
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn detect_application_context() -> Result<ApplicationContext> {
    todo!("Implement context detection")
}

/// Get all known application contexts
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_all_contexts() -> Result<Vec<ApplicationContext>> {
    todo!("Implement context storage")
}

// ============================================================================
// Device Profile Functions
// ============================================================================

/// Get or create device profile
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_or_create_device_profile() -> Result<serde_json::Value> {
    todo!("Implement device profile management")
}

/// Update device profile settings
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn update_device_profile(settings: serde_json::Value) -> Result<()> {
    todo!("Implement device profile management")
}

// ============================================================================
// Usage Quota Functions
// ============================================================================

/// Check if quota is available
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn check_quota_available(words_needed: i32) -> Result<bool> {
    todo!("Implement quota checking")
}

/// Get current quota usage
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_quota_usage() -> Result<serde_json::Value> {
    todo!("Implement quota tracking")
}

/// Add words to quota usage
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn add_words_to_quota(word_count: i32) -> Result<()> {
    todo!("Implement quota tracking")
}

// ============================================================================
// Personal Dictionary Functions
// ============================================================================

/// Add entry to personal dictionary
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn add_dictionary_entry(
    phrase: &str,
    replacement: &str,
    case_sensitive: bool,
) -> Result<()> {
    todo!("Implement dictionary management")
}

/// Remove entry from personal dictionary
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn remove_dictionary_entry(phrase: &str) -> Result<()> {
    todo!("Implement dictionary management")
}

/// Get all dictionary entries
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_dictionary_entries() -> Result<Vec<serde_json::Value>> {
    todo!("Implement dictionary management")
}

// ============================================================================
// Migration & Setup Functions
// ============================================================================

/// Run database migrations
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn run_migrations() -> Result<()> {
    todo!("Implement migration runner")
}

/// Get database schema version
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_schema_version() -> Result<i32> {
    todo!("Implement migration runner")
}
