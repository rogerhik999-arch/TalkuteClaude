-- Create personal_dictionary table
CREATE TABLE IF NOT EXISTS personal_dictionary (
    entry_id TEXT PRIMARY KEY NOT NULL,
    device_id TEXT NOT NULL,
    phrase TEXT NOT NULL,
    replacement TEXT NOT NULL,
    case_sensitive INTEGER NOT NULL DEFAULT 0,
    whole_word_only INTEGER NOT NULL DEFAULT 0,
    category TEXT NOT NULL,
    created_at TEXT NOT NULL,
    last_used_at TEXT,
    usage_count INTEGER NOT NULL DEFAULT 0,
    UNIQUE(device_id, phrase, replacement, case_sensitive)
);

-- Create device_profiles table (for foreign key support)
CREATE TABLE IF NOT EXISTS device_profiles (
    device_id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL,
    last_active_at TEXT NOT NULL,
    preferred_language TEXT NOT NULL DEFAULT 'en-US',
    voice_speed_preference REAL NOT NULL DEFAULT 1.0,
    auto_punctuation_enabled INTEGER NOT NULL DEFAULT 1,
    filler_removal_enabled INTEGER NOT NULL DEFAULT 1,
    self_correction_enabled INTEGER NOT NULL DEFAULT 1,
    crash_reporting_enabled INTEGER NOT NULL DEFAULT 0
);

-- Create schema_versions table for migration tracking
CREATE TABLE IF NOT EXISTS schema_versions (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);

-- Insert initial schema version
INSERT INTO schema_versions (version, applied_at) VALUES (1, datetime('now'));
