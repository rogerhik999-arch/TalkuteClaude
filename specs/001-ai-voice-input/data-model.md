# Data Model: Talkute AI Voice Input Assistant

**Feature**: 001-ai-voice-input | **Date**: 2026-03-04 | **Spec**: [spec.md](./spec.md)

## Overview

This document defines the core data entities, their relationships, validation rules, and state transitions for the Talkute AI Voice Input Assistant. All entities are stored locally on the device using SQLite with no cloud synchronization.

## Entity Relationship Diagram

```text
DeviceProfile (1) ──< (N) VoiceSession
DeviceProfile (1) ──< (N) PersonalDictionaryEntry
DeviceProfile (1) ──── (1) UsageQuota
VoiceSession (1) ──< (N) TranscriptionHistory
VoiceSession (N) ──> (1) ApplicationContext
```

## Core Entities

### 1. DeviceProfile

**Purpose**: Represents the single user profile for this device installation.

**Fields**:
- `device_id` (UUID, PRIMARY KEY) - Unique device identifier, generated on first launch
- `created_at` (DateTime, NOT NULL) - Profile creation timestamp
- `last_active_at` (DateTime, NOT NULL) - Last activity timestamp
- `preferred_language` (String, NOT NULL, DEFAULT "en-US") - ISO 639-1 language code
- `voice_speed_preference` (Float, NOT NULL, DEFAULT 1.0) - Speech rate multiplier (0.5-2.0)
- `auto_punctuation_enabled` (Boolean, NOT NULL, DEFAULT true) - Enable automatic punctuation
- `filler_removal_enabled` (Boolean, NOT NULL, DEFAULT true) - Enable filler word removal
- `self_correction_enabled` (Boolean, NOT NULL, DEFAULT true) - Enable self-correction detection
- `crash_reporting_enabled` (Boolean, NOT NULL, DEFAULT false) - Anonymous crash reporting opt-in

**Validation Rules**:
- `device_id` must be valid UUID v4
- `voice_speed_preference` must be between 0.5 and 2.0 inclusive
- `preferred_language` must match supported language list (en-US, zh-CN, ja-JP, es-ES, fr-FR, de-DE)

**Relationships**:
- One-to-many with VoiceSession
- One-to-many with PersonalDictionaryEntry
- One-to-one with UsageQuota

**Lifecycle**: Created on first app launch, persists until app uninstall.

---

### 2. VoiceSession

**Purpose**: Represents a single voice input session from start to completion.

**Fields**:
- `session_id` (UUID, PRIMARY KEY) - Unique session identifier
- `device_id` (UUID, FOREIGN KEY → DeviceProfile, NOT NULL) - Owner device
- `context_id` (UUID, FOREIGN KEY → ApplicationContext, NULLABLE) - Detected application context
- `started_at` (DateTime, NOT NULL) - Session start timestamp
- `ended_at` (DateTime, NULLABLE) - Session end timestamp
- `duration_seconds` (Integer, NULLABLE) - Total session duration
- `raw_transcription` (Text, NULLABLE) - Original speech-to-text output
- `polished_text` (Text, NULLABLE) - AI-enhanced final text
- `word_count` (Integer, NOT NULL, DEFAULT 0) - Word count for quota tracking
- `status` (Enum, NOT NULL) - Session state: RECORDING | TRANSCRIBING | POLISHING | COMPLETED | FAILED | CANCELLED
- `error_message` (Text, NULLABLE) - Error details if status = FAILED
- `ai_model_used` (String, NULLABLE) - AI model identifier (e.g., "claude-3-5-sonnet-20241022")
- `speech_api_used` (String, NULLABLE) - Speech API identifier (e.g., "azure-speech-v1")

**Validation Rules**:
- `duration_seconds` must be ≤ 300 (5 minutes max)
- `word_count` must be ≥ 0
- `ended_at` must be ≥ `started_at` if not NULL
- `status` transitions must follow state machine (see State Transitions below)

**Relationships**:
- Many-to-one with DeviceProfile
- Many-to-one with ApplicationContext (optional)
- One-to-many with TranscriptionHistory

**State Transitions**:
```text
RECORDING → TRANSCRIBING → POLISHING → COMPLETED
         ↘                ↘          ↘
          CANCELLED        FAILED     FAILED
```

**Lifecycle**: Created when user starts voice input, updated during processing, finalized on completion/cancellation.

---

### 3. ApplicationContext

**Purpose**: Represents the detected application and context where voice input is being used.

**Fields**:
- `context_id` (UUID, PRIMARY KEY) - Unique context identifier
- `application_name` (String, NOT NULL) - Application executable name
- `application_title` (String, NULLABLE) - Window title or activity name
- `application_category` (Enum, NOT NULL) - Category: EMAIL | CHAT | DOCUMENT | CODE | BROWSER | OTHER
- `detected_at` (DateTime, NOT NULL) - First detection timestamp
- `last_used_at` (DateTime, NOT NULL) - Most recent usage timestamp
- `usage_count` (Integer, NOT NULL, DEFAULT 1) - Number of times used
- `preferred_tone` (Enum, NULLABLE) - Tone preference: FORMAL | CASUAL | TECHNICAL | CREATIVE
- `custom_instructions` (Text, NULLABLE) - User-defined context-specific instructions

**Validation Rules**:
- `application_name` must not be empty
- `usage_count` must be ≥ 1
- `last_used_at` must be ≥ `detected_at`

**Relationships**:
- One-to-many with VoiceSession

**Lifecycle**: Created on first detection of new application, updated on each subsequent use.

---

### 4. PersonalDictionaryEntry

**Purpose**: User-defined custom words, phrases, and replacements for personalized text processing.

**Fields**:
- `entry_id` (UUID, PRIMARY KEY) - Unique entry identifier
- `device_id` (UUID, FOREIGN KEY → DeviceProfile, NOT NULL) - Owner device
- `phrase` (String, NOT NULL) - Original phrase to match
- `replacement` (String, NOT NULL) - Replacement text
- `case_sensitive` (Boolean, NOT NULL, DEFAULT false) - Case-sensitive matching
- `whole_word_only` (Boolean, NOT NULL, DEFAULT true) - Match whole words only
- `created_at` (DateTime, NOT NULL) - Entry creation timestamp
- `last_used_at` (DateTime, NULLABLE) - Most recent usage timestamp
- `usage_count` (Integer, NOT NULL, DEFAULT 0) - Number of times applied

**Validation Rules**:
- `phrase` must be 1-100 characters
- `replacement` must be 1-200 characters
- `phrase` and `replacement` must not be identical
- Unique constraint on (`device_id`, `phrase`, `case_sensitive`)

**Relationships**:
- Many-to-one with DeviceProfile

**Lifecycle**: Created by user, persists until manually deleted.

---

### 5. TranscriptionHistory

**Purpose**: Stores intermediate transcription states for debugging and user review.

**Fields**:
- `history_id` (UUID, PRIMARY KEY) - Unique history entry identifier
- `session_id` (UUID, FOREIGN KEY → VoiceSession, NOT NULL) - Parent session
- `timestamp` (DateTime, NOT NULL) - Entry creation timestamp
- `stage` (Enum, NOT NULL) - Processing stage: RAW_SPEECH | TRANSCRIBED | FILLER_REMOVED | SELF_CORRECTED | AI_POLISHED
- `text_content` (Text, NOT NULL) - Text at this stage
- `metadata` (JSON, NULLABLE) - Stage-specific metadata (confidence scores, detected corrections, etc.)

**Validation Rules**:
- `text_content` must not be empty
- `stage` must follow processing order within a session

**Relationships**:
- Many-to-one with VoiceSession

**Lifecycle**: Created during session processing, retained for 30 days, then auto-deleted.

---

### 6. UsageQuota

**Purpose**: Tracks weekly word count usage for free tier enforcement.

**Fields**:
- `quota_id` (UUID, PRIMARY KEY) - Unique quota identifier
- `device_id` (UUID, FOREIGN KEY → DeviceProfile, NOT NULL, UNIQUE) - Owner device
- `current_week_start` (Date, NOT NULL) - Start date of current tracking week (Monday)
- `words_used_this_week` (Integer, NOT NULL, DEFAULT 0) - Word count used in current week
- `weekly_limit` (Integer, NOT NULL, DEFAULT 4000) - Weekly word limit
- `last_reset_at` (DateTime, NOT NULL) - Last quota reset timestamp
- `total_words_all_time` (Integer, NOT NULL, DEFAULT 0) - Cumulative word count

**Validation Rules**:
- `words_used_this_week` must be ≥ 0
- `weekly_limit` must be > 0
- `total_words_all_time` must be ≥ `words_used_this_week`
- `current_week_start` must be a Monday

**Relationships**:
- One-to-one with DeviceProfile

**Lifecycle**: Created with DeviceProfile, reset every Monday at 00:00 local time.

---

### 7. LicenseKey (Future - Not MVP)

**Purpose**: Stores license key information for paid tier upgrades.

**Fields**:
- `license_id` (UUID, PRIMARY KEY) - Unique license identifier
- `device_id` (UUID, FOREIGN KEY → DeviceProfile, NOT NULL, UNIQUE) - Owner device
- `license_key` (String, NOT NULL, UNIQUE) - License key string
- `tier` (Enum, NOT NULL) - License tier: PRO | ENTERPRISE
- `activated_at` (DateTime, NOT NULL) - Activation timestamp
- `expires_at` (DateTime, NULLABLE) - Expiration timestamp (NULL = lifetime)
- `is_active` (Boolean, NOT NULL, DEFAULT true) - License active status

**Validation Rules**:
- `license_key` must match format: `TALKUTE-[A-Z0-9]{4}-[A-Z0-9]{4}-[A-Z0-9]{4}`
- `expires_at` must be > `activated_at` if not NULL
- Cannot have multiple active licenses per device

**Relationships**:
- One-to-one with DeviceProfile

**Lifecycle**: Created on license activation, deactivated on expiration or revocation.

**Note**: This entity is planned for future paid tier implementation and is NOT part of the MVP scope.

---

## Database Schema (SQLite)

```sql
-- DeviceProfile table
CREATE TABLE device_profiles (
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

-- ApplicationContext table
CREATE TABLE application_contexts (
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

-- VoiceSession table
CREATE TABLE voice_sessions (
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

-- PersonalDictionaryEntry table
CREATE TABLE personal_dictionary (
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

-- TranscriptionHistory table
CREATE TABLE transcription_history (
    history_id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    stage TEXT NOT NULL,
    text_content TEXT NOT NULL,
    metadata TEXT,
    FOREIGN KEY (session_id) REFERENCES voice_sessions(session_id)
);

-- UsageQuota table
CREATE TABLE usage_quotas (
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
CREATE INDEX idx_voice_sessions_device ON voice_sessions(device_id);
CREATE INDEX idx_voice_sessions_started ON voice_sessions(started_at DESC);
CREATE INDEX idx_transcription_history_session ON transcription_history(session_id);
CREATE INDEX idx_personal_dictionary_device ON personal_dictionary(device_id);
CREATE INDEX idx_application_contexts_last_used ON application_contexts(last_used_at DESC);
```

## Data Retention Policy

- **VoiceSession**: Retained for 90 days, then auto-deleted
- **TranscriptionHistory**: Retained for 30 days, then auto-deleted
- **ApplicationContext**: Retained indefinitely, pruned if unused for 180 days
- **PersonalDictionaryEntry**: Retained indefinitely until user deletion
- **DeviceProfile**: Retained until app uninstall
- **UsageQuota**: Retained until app uninstall

## Privacy & Security

- All data stored locally on device using SQLite with encryption at rest (platform-provided)
- No cloud synchronization or backup
- No personally identifiable information (PII) collected
- Crash reports (if opted in) are anonymized and contain no transcription content
- User can export all data as JSON via settings
- User can delete all data via settings (factory reset)

## Migration Strategy

Database schema versioning using sequential migration files:
- `migrations/001_initial_schema.sql` - Initial tables
- `migrations/002_add_indexes.sql` - Performance indexes
- Future migrations numbered sequentially

Rust migration runner checks current schema version on app start and applies pending migrations atomically.
