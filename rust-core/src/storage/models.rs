//! Model definitions for data entities

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Datelike, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub preferred_language: String,
    pub voice_speed_preference: f64,
    pub auto_punctuation_enabled: bool,
    pub filler_removal_enabled: bool,
    pub self_correction_enabled: bool,
    pub crash_reporting_enabled: bool,
}

impl DeviceProfile {
    pub fn new(device_id: String) -> Self {
        let now = Utc::now();
        Self {
            device_id,
            created_at: now,
            last_active_at: now,
            preferred_language: "en-US".to_string(),
            voice_speed_preference: 1.0,
            auto_punctuation_enabled: true,
            filler_removal_enabled: true,
            self_correction_enabled: true,
            crash_reporting_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationContext {
    pub context_id: String,
    pub application_name: String,
    pub application_title: Option<String>,
    pub application_category: String,
    pub detected_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub usage_count: i32,
    pub preferred_tone: Option<String>,
    pub custom_instructions: Option<String>,
}

impl ApplicationContext {
    pub fn new(
        context_id: String,
        application_name: String,
        application_category: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            context_id,
            application_name,
            application_title: None,
            application_category,
            detected_at: now,
            last_used_at: now,
            usage_count: 1,
            preferred_tone: None,
            custom_instructions: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSession {
    pub session_id: String,
    pub device_id: String,
    pub context_id: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub raw_transcription: Option<String>,
    pub polished_text: Option<String>,
    pub word_count: i32,
    pub status: String,
    pub error_message: Option<String>,
    pub ai_model_used: Option<String>,
    pub speech_api_used: Option<String>,
}

impl VoiceSession {
    pub fn new(session_id: String, device_id: String) -> Self {
        Self {
            session_id,
            device_id,
            context_id: None,
            started_at: Utc::now(),
            ended_at: None,
            duration_seconds: None,
            raw_transcription: None,
            polished_text: None,
            word_count: 0,
            status: "RECORDING".to_string(),
            error_message: None,
            ai_model_used: None,
            speech_api_used: None,
        }
    }
}

/// Dictionary entry category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DictionaryEntryCategory {
    Technical,
    Business,
    Medical,
    General,
}

impl DictionaryEntryCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            DictionaryEntryCategory::Technical => "technical",
            DictionaryEntryCategory::Business => "business",
            DictionaryEntryCategory::Medical => "medical",
            DictionaryEntryCategory::General => "general",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDictionaryEntry {
    pub entry_id: String,
    pub device_id: String,
    pub phrase: String,
    pub replacement: String,
    pub case_sensitive: bool,
    pub whole_word_only: bool,
    pub category: DictionaryEntryCategory,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub usage_count: i32,
}

impl PersonalDictionaryEntry {
    pub fn new(
        entry_id: String,
        device_id: String,
        phrase: String,
        replacement: String,
        category: DictionaryEntryCategory,
    ) -> Self {
        Self {
            entry_id,
            device_id,
            phrase,
            replacement,
            category,
            case_sensitive: false,
            whole_word_only: true,
            created_at: Utc::now(),
            last_used_at: None,
            usage_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionHistory {
    pub history_id: String,
    pub session_id: String,
    pub timestamp: DateTime<Utc>,
    pub stage: String,
    pub text_content: String,
    pub metadata: Option<String>,
}

impl TranscriptionHistory {
    pub fn new(
        history_id: String,
        session_id: String,
        stage: String,
        text_content: String,
    ) -> Self {
        Self {
            history_id,
            session_id,
            timestamp: Utc::now(),
            stage,
            text_content,
            metadata: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageQuota {
    pub quota_id: String,
    pub device_id: String,
    pub current_week_start: String,
    pub words_used_this_week: i32,
    pub weekly_limit: i32,
    pub last_reset_at: DateTime<Utc>,
    pub total_words_all_time: i32,
}

impl UsageQuota {
    pub fn new(quota_id: String, device_id: String) -> Self {
        let now = Utc::now();
        // Calculate week start (Monday)
        let today = now.date_naive();
        let weekday = today.weekday().num_days_from_monday() as i64;
        let week_start = today - Duration::days(weekday);

        Self {
            quota_id,
            device_id,
            current_week_start: week_start.to_string(),
            words_used_this_week: 0,
            weekly_limit: 4000,
            last_reset_at: now,
            total_words_all_time: 0,
        }
    }
}
