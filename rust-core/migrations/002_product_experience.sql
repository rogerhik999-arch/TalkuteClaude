-- Migration 002: Product Experience Design
-- Creates tables for user preferences, custom dictionary, and transcription history

-- User Preferences table
-- Stores key-value pairs for all user-configurable settings
CREATE TABLE IF NOT EXISTS user_preferences (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,  -- JSON-encoded value
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Insert default preferences
INSERT OR IGNORE INTO user_preferences (key, value) VALUES
    ('input_mode', '"push_to_talk"'),
    ('idle_timeout_seconds', '1.5'),
    ('polishing_intensity', '"standard"'),
    ('input_language', '"auto"'),
    ('output_language', '"same_as_input"'),
    ('translation_enabled', 'false'),
    ('auto_process', 'true'),
    ('filler_removal', 'true'),
    ('context_aware', 'true'),
    ('noise_cancellation', 'true'),
    ('hotkey', '"Ctrl+Shift+Space"'),
    ('history_retention_days', '30'),
    ('crash_reporting', 'true'),
    ('analytics', 'false');

-- Dictionary entries table
-- Maps spoken forms to standard written forms for improved transcription accuracy
CREATE TABLE IF NOT EXISTS dictionary_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    voice_form TEXT NOT NULL UNIQUE,      -- How the word is spoken/detected
    standard_form TEXT NOT NULL,           -- The replacement text
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Index for fast voice_form lookup during transcription
CREATE INDEX IF NOT EXISTS idx_dictionary_voice_form ON dictionary_entries(voice_form);

-- History entries table
-- Records all transcription sessions for review and recovery
CREATE TABLE IF NOT EXISTS history_entries (
    id TEXT PRIMARY KEY NOT NULL,          -- UUID
    timestamp TEXT NOT NULL,               -- ISO 8601 timestamp
    app_context TEXT,                      -- Target application name/category
    app_bundle_id TEXT,                    -- Platform-specific app identifier
    original_text TEXT NOT NULL,           -- Raw transcription before polishing
    polished_text TEXT,                    -- AI-polished output (nullable if failed)
    input_language TEXT NOT NULL,          -- Detected/specified input language
    output_language TEXT,                  -- Target language if translated
    polishing_intensity TEXT NOT NULL,     -- 'light', 'standard', or 'deep'
    was_translated INTEGER NOT NULL DEFAULT 0,  -- Boolean: 0 or 1
    duration_ms INTEGER NOT NULL,          -- Recording duration in milliseconds
    success INTEGER NOT NULL DEFAULT 1,    -- Boolean: 0 or 1
    error_message TEXT                     -- Error details if failed
);

-- Index for history queries by date
CREATE INDEX IF NOT EXISTS idx_history_timestamp ON history_entries(timestamp);

-- Index for history queries by app
CREATE INDEX IF NOT EXISTS idx_history_app_context ON history_entries(app_context);

-- Quota tracking table
-- Tracks daily usage for quota management
CREATE TABLE IF NOT EXISTS quota_tracking (
    date TEXT PRIMARY KEY NOT NULL,        -- Date in YYYY-MM-DD format
    transcription_count INTEGER NOT NULL DEFAULT 0,
    total_duration_ms INTEGER NOT NULL DEFAULT 0,
    grace_used INTEGER NOT NULL DEFAULT 0  -- Boolean: 0 or 1
);

-- Trigger to auto-cleanup old history entries based on retention policy
CREATE TRIGGER IF NOT EXISTS cleanup_old_history
AFTER INSERT ON history_entries
BEGIN
    DELETE FROM history_entries
    WHERE timestamp < datetime('now', '-' || COALESCE(
        (SELECT CAST(value AS INTEGER) FROM user_preferences WHERE key = 'history_retention_days'),
        30
    ) || ' days');
END;