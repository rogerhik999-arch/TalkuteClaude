//! FFI bridge between Rust and Flutter
//!
//! This module defines the function signatures that are exposed to Flutter via flutter_rust_bridge.
//! All functions are async and return Results for proper error handling.

use crate::error::Result;
use crate::speech::SpeechRecognitionService;
use crate::processing::TextProcessingPipeline;
use crate::ai::polisher::TextPolisher;
use crate::context::detector::ContextDetector;
use crate::storage::database::Database;
use crate::storage::profile::DeviceProfileService;
use serde::{Deserialize, Serialize};

use super::session_manager::SessionManager;

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
    Stopped,
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self::Idle
    }
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
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn start_voice_session(device_id: &str, context_id: Option<String>) -> Result<String> {
    let manager = SessionManager::global();
    manager.create_session(device_id, context_id).await
}

/// Stop the current voice session
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn stop_voice_session(session_id: &str) -> Result<VoiceSessionInfo> {
    let manager = SessionManager::global();
    manager.update_status(session_id, SessionStatus::Completed).await?;
    manager.get_session_info(session_id).await
}

/// Cancel the current voice session
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn cancel_voice_session(session_id: &str, _reason: Option<&str>) -> Result<()> {
    let manager = SessionManager::global();
    manager.update_status(session_id, SessionStatus::Cancelled).await?;
    manager.remove_session(session_id).await?;
    Ok(())
}

/// Get the current session status
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_session_status(session_id: &str) -> Result<SessionStatus> {
    let manager = SessionManager::global();
    let session = manager.get_session(session_id).await?;
    Ok(session.status)
}

// ============================================================================
// Audio Processing Functions
// ============================================================================

/// Start audio capture from the microphone
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn start_audio_capture(device_id: &str) -> Result<String> {
    // Create a session to track the capture
    let manager = SessionManager::global();
    let session_id = manager.create_session(device_id, None).await?;
    manager.update_status(&session_id, SessionStatus::Recording).await?;

    Ok(session_id)
}

/// Stop audio capture
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn stop_audio_capture(capture_id: &str) -> Result<()> {
    let manager = SessionManager::global();
    manager.update_status(capture_id, SessionStatus::Transcribing).await?;
    Ok(())
}

/// Get audio level (0.0 - 1.0)
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_audio_level(_capture_id: &str) -> Result<f64> {
    // TODO: Implement actual audio level detection
    Ok(0.0)
}

// ============================================================================
// Transcription & AI Functions
// ============================================================================

/// Transcribe audio to text
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn transcribe_audio(
    session_id: &str,
    _audio_path: &str,
    language: Option<&str>,
) -> Result<String> {
    let manager = SessionManager::global();
    manager.update_status(session_id, SessionStatus::Transcribing).await?;

    // Get session to retrieve device_id
    let session = manager.get_session(session_id).await?;

    // Create speech service and transcribe
    let mut service = SpeechRecognitionService::new()?;
    service.start_session(&session.device_id).await?;
    let text = service.stop_session(language).await?;

    // Process through text pipeline
    let pipeline = TextProcessingPipeline::new();
    let processed = pipeline.process(&text);

    // Store raw transcription
    manager.set_raw_transcription(session_id, &processed).await?;

    Ok(processed)
}

/// Polish text using AI
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn polish_text(
    session_id: &str,
    text: &str,
    _context_id: Option<&str>,
) -> Result<String> {
    let manager = SessionManager::global();
    manager.update_status(session_id, SessionStatus::Polishing).await?;

    // Use AI polisher
    let polisher = TextPolisher::new()?;
    let polished = polisher.polish(text).await?;

    // Store polished text
    manager.set_polished_text(session_id, &polished).await?;
    manager.update_status(session_id, SessionStatus::Completed).await?;

    Ok(polished)
}

/// Get raw transcription for a session
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_raw_transcription(session_id: &str) -> Result<String> {
    let manager = SessionManager::global();
    let session = manager.get_session(session_id).await?;
    Ok(session.raw_transcription)
}

/// Get polished text for a session
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_polished_text(session_id: &str) -> Result<String> {
    let manager = SessionManager::global();
    let session = manager.get_session(session_id).await?;
    Ok(session.polished_text)
}

// ============================================================================
// Context Detection Functions
// ============================================================================

/// Detect the current application context
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn detect_application_context() -> Result<ApplicationContext> {
    let detector = ContextDetector::new().await?;
    let context = detector.detect().await?;

    Ok(ApplicationContext {
        context_id: context.context_id,
        application_name: context.application_name,
        application_title: context.application_title,
        application_category: context.application_category,
        preferred_tone: None,
    })
}

/// Get all known application contexts
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_all_contexts() -> Result<Vec<ApplicationContext>> {
    // TODO: Implement context storage
    Ok(Vec::new())
}

// ============================================================================
// Device Profile Functions
// ============================================================================

/// Get or create device profile
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_or_create_device_profile() -> Result<serde_json::Value> {
    let db = Database::in_memory()?;
    let service = DeviceProfileService::new(db);
    let profile = service.get_or_create()?;

    Ok(serde_json::to_value(profile)
        .map_err(|e| crate::error::Error::Unknown(e.to_string()))?)
}

/// Update device profile settings
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn update_device_profile(_settings: serde_json::Value) -> Result<()> {
    // TODO: Implement profile update
    Ok(())
}

// ============================================================================
// Usage Quota Functions
// ============================================================================

/// Check if quota is available
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn check_quota_available(_words_needed: i32) -> Result<bool> {
    // TODO: Implement quota checking with device ID
    Ok(true)
}

/// Get current quota usage
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_quota_usage() -> Result<serde_json::Value> {
    // TODO: Implement quota retrieval
    Ok(serde_json::json!({
        "words_used_this_week": 0,
        "weekly_limit": 4000,
        "percentage_used": 0.0
    }))
}

/// Add words to quota usage
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn add_words_to_quota(_word_count: i32) -> Result<()> {
    // TODO: Implement quota update
    Ok(())
}

// ============================================================================
// Personal Dictionary Functions
// ============================================================================

/// Add entry to personal dictionary
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn add_dictionary_entry(
    _phrase: &str,
    _replacement: &str,
    _case_sensitive: bool,
) -> Result<()> {
    // TODO: Implement dictionary management
    Ok(())
}

/// Remove entry from personal dictionary
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn remove_dictionary_entry(_phrase: &str) -> Result<()> {
    // TODO: Implement dictionary management
    Ok(())
}

/// Get all dictionary entries
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_dictionary_entries() -> Result<Vec<serde_json::Value>> {
    // TODO: Implement dictionary retrieval
    Ok(Vec::new())
}

// ============================================================================
// Migration & Setup Functions
// ============================================================================

/// Run database migrations
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn run_migrations() -> Result<()> {
    // Database automatically runs migrations on open
    let _db = Database::in_memory()?;
    Ok(())
}

/// Get database schema version
#[allow(clippy::missing_safety_doc)]
#[flutter_rust_bridge::frb]
pub async fn get_schema_version() -> Result<i32> {
    // TODO: Implement version tracking
    Ok(1)
}
