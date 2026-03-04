-- SQLite database schema for Talkute AI Voice Input Assistant
-- Version 1: Initial schema

-- Device profile table - stores user preferences per device
CREATE TABLE IF NOT EXISTS device_profiles (
    device_id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL,
    last_active_at TEXT NOT NULL,
    preferred_language TEXT NOT NULL DEFAULT 'en-US',
    voice_speed_preference REAL NOT NULL DEFAULT 1.0,
    auto_punctuation_enabled INTEGER NOT NULL DEFAULT 1,
    filler_removal_enabled INTEGER NOT NULL DEFAULT 1,
    self_correction_enabled INTEGER NOT NULL DEFAULT 1,
    crash_reporting_enabled INTEGER NOT NULL DEFAULT 0,
    CHECK (voice_speed_preference BETWEEN 0.5 AND 2.0)
);

-- Application context table - tracks detected applications
CREATE TABLE IF NOT EXISTS application_contexts (
    context_id TEXT PRIMARY KEY,
    application_name TEXT NOT NULL,
    application_title TEXT,
    application_category TEXT NOT NULL,
    detected_at TEXT NOT NULL,
    last_used_at TEXT NOT NULL,
    usage_count INTEGER NOT NULL DEFAULT 1,
    preferred_tone TEXT,
    custom_instructions TEXT,
    CHECK (usage_count >= 1)
);

-- Voice session table - tracks each voice input session
CREATE TABLE IF NOT EXISTS voice_sessions (
    session_id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    context_id TEXT,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    duration_seconds INTEGER,
    raw_transcription TEXT,
    polished_text TEXT,
    word_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    error_message TEXT,
    ai_model_used TEXT,
    speech_api_used TEXT,
    FOREIGN KEY (device_id) REFERENCES device_profiles(device_id),
    FOREIGN KEY (context_id) REFERENCES application_contexts(context_id),
    CHECK (duration_seconds IS NULL OR duration_seconds <= 300),
    CHECK (word_count >= 0)
);

-- Personal dictionary table - user-defined custom words and phrases
CREATE TABLE IF NOT EXISTS personal_dictionary (
    entry_id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    phrase TEXT NOT NULL,
    replacement TEXT NOT NULL,
    case_sensitive INTEGER NOT NULL DEFAULT 0,
    whole_word_only INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    last_used_at TEXT,
    usage_count INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (device_id) REFERENCES device_profiles(device_id),
    UNIQUE (device_id, phrase, case_sensitive)
);

-- Transcription history table - intermediate processing states
CREATE TABLE IF NOT EXISTS transcription_history (
    history_id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    stage TEXT NOT NULL,
    text_content TEXT NOT NULL,
    metadata TEXT,
    FOREIGN KEY (session_id) REFERENCES voice_sessions(session_id)
);

-- Usage quota table - tracks weekly word count
CREATE TABLE IF NOT EXISTS usage_quotas (
    quota_id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL UNIQUE,
    current_week_start TEXT NOT NULL,
    words_used_this_week INTEGER NOT NULL DEFAULT 0,
    weekly_limit INTEGER NOT NULL DEFAULT 4000,
    last_reset_at TEXT NOT NULL,
    total_words_all_time INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (device_id) REFERENCES device_profiles(device_id),
    CHECK (words_used_this_week >= 0),
    CHECK (weekly_limit > 0),
    CHECK (total_words_all_time >= words_used_this_week)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_voice_sessions_device ON voice_sessions(device_id);
CREATE INDEX IF NOT EXISTS idx_voice_sessions_started ON voice_sessions(started_at DESC);
CREATE INDEX IF NOT EXISTS idx_transcription_history_session ON transcription_history(session_id);
CREATE INDEX IF NOT EXISTS idx_personal_dictionary_device ON personal_dictionary(device_id);
CREATE INDEX IF NOT EXISTS idx_application_contexts_last_used ON application_contexts(last_used_at DESC);
