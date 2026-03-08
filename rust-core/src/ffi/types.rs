//! FFI type definitions

use serde::{Deserialize, Serialize};

/// Platform-specific device identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceId(String);

/// Application category enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationCategory {
    Email,
    Chat,
    Document,
    Code,
    Browser,
    Other,
}

impl Default for ApplicationCategory {
    fn default() -> Self {
        ApplicationCategory::Other
    }
}

/// Tone preference enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TonePreference {
    Formal,
    Casual,
    Technical,
    Creative,
}

impl Default for TonePreference {
    fn default() -> Self {
        TonePreference::Casual
    }
}

/// Usage quota information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaInfo {
    pub words_used_this_week: i32,
    pub weekly_limit: i32,
    pub percentage_used: f64,
}

/// Speech recognition API choice
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeechApi {
    Azure,
    Google,
    Aws,
}

impl Default for SpeechApi {
    fn default() -> Self {
        SpeechApi::Azure
    }
}

/// Logging level for the application
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}
