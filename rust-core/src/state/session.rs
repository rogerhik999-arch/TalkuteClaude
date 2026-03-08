//! Session state and TranscriptionSession definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Session state for the voice input flow
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionState {
    /// Ready to start recording
    Idle,
    /// Currently capturing audio
    Recording,
    /// Processing transcription and AI polishing
    Processing,
    /// Error occurred during processing
    Error,
}

impl Default for SessionState {
    fn default() -> Self {
        Self::Idle
    }
}

impl SessionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Recording => "recording",
            Self::Processing => "processing",
            Self::Error => "error",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "idle" => Some(Self::Idle),
            "recording" => Some(Self::Recording),
            "processing" => Some(Self::Processing),
            "error" => Some(Self::Error),
            _ => None,
        }
    }
}

/// Represents a single voice-to-text transcription session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptionSession {
    /// Unique session identifier
    pub id: Uuid,
    /// Current state of the session
    pub state: SessionState,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session was last updated
    pub updated_at: DateTime<Utc>,
    /// Duration of the recording in milliseconds
    pub duration_ms: Option<u64>,
    /// The active application when recording started
    pub app_context: Option<String>,
    /// Raw transcription from ASR
    pub raw_transcription: Option<String>,
    /// AI-polished text
    pub polished_text: Option<String>,
    /// Detected or specified input language
    pub input_language: Option<String>,
    /// Target output language (if translation enabled)
    pub output_language: Option<String>,
    /// Polishing intensity level
    pub polishing_intensity: Option<String>,
    /// Error message if state is Error
    pub error_message: Option<String>,
    /// Retry count for failed processing
    pub retry_count: u32,
}

impl TranscriptionSession {
    /// Create a new session
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            state: SessionState::Idle,
            created_at: now,
            updated_at: now,
            duration_ms: None,
            app_context: None,
            raw_transcription: None,
            polished_text: None,
            input_language: None,
            output_language: None,
            polishing_intensity: None,
            error_message: None,
            retry_count: 0,
        }
    }

    /// Start recording
    pub fn start_recording(&mut self) {
        self.state = SessionState::Recording;
        self.updated_at = Utc::now();
    }

    /// Stop recording and start processing
    pub fn stop_recording(&mut self, duration_ms: u64) {
        self.duration_ms = Some(duration_ms);
        self.state = SessionState::Processing;
        self.updated_at = Utc::now();
    }

    /// Mark processing as complete
    pub fn complete(&mut self, polished_text: String) {
        self.polished_text = Some(polished_text);
        self.state = SessionState::Idle;
        self.updated_at = Utc::now();
    }

    /// Mark session as failed
    pub fn fail(&mut self, error_message: String) {
        self.error_message = Some(error_message);
        self.state = SessionState::Error;
        self.updated_at = Utc::now();
    }

    /// Retry from error state
    pub fn retry(&mut self) -> bool {
        if self.state == SessionState::Error && self.retry_count < 3 {
            self.retry_count += 1;
            self.state = SessionState::Processing;
            self.error_message = None;
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Cancel the session
    pub fn cancel(&mut self) {
        self.state = SessionState::Idle;
        self.raw_transcription = None;
        self.polished_text = None;
        self.error_message = None;
        self.updated_at = Utc::now();
    }
}

impl Default for TranscriptionSession {
    fn default() -> Self {
        Self::new()
    }
}