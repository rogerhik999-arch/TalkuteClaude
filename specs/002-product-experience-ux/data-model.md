# Data Model: Product Experience Design Implementation

**Feature**: 002-product-experience-ux
**Date**: 2026-03-07
**Source**: [spec.md](./spec.md)

## Overview

This document defines the data entities, relationships, and storage schema for the Product Experience Design feature. Data is persisted in SQLite via the existing Rust core storage layer.

## Entity Relationship Diagram

```
┌─────────────────────┐     ┌─────────────────────┐
│   UserPreferences   │     │   DictionaryEntry   │
├─────────────────────┤     ├─────────────────────┤
│ id: PK              │     │ id: PK              │
│ key: String (unique)│     │ voice_form: String  │
│ value: String       │     │ standard_form: String│
│ updated_at: DateTime│     │ created_at: DateTime│
└─────────────────────┘     │ updated_at: DateTime│
                            └─────────────────────┘

┌─────────────────────────────────────────────────────┐
│                   HistoryEntry                       │
├─────────────────────────────────────────────────────┤
│ id: PK (UUID)                                       │
│ timestamp: DateTime                                  │
│ app_context: String (application name)              │
│ original_text: String (raw transcription)           │
│ polished_text: String (AI-processed)                │
│ input_language: String (ISO 639-1)                  │
│ output_language: String (ISO 639-1)                 │
│ polishing_intensity: Enum (Light/Standard/Deep)     │
│ was_translated: Boolean                             │
│ duration_ms: Integer (recording duration)           │
│ success: Boolean                                     │
│ error_message: String? (nullable)                   │
└─────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────┐
│                TranscriptionSession                  │
├─────────────────────────────────────────────────────┤
│ session_id: UUID                                     │
│ state: Enum (Idle/Recording/Processing/Error)       │
│ started_at: DateTime                                 │
│ audio_buffer: Vec<u8> (temporary, not persisted)    │
│ detected_language: String?                          │
│ target_language: String?                            │
│ retry_count: Integer                                 │
│ last_error: String?                                  │
└─────────────────────────────────────────────────────┘
```

## Entity Definitions

### 1. UserPreferences

Key-value store for all user-configurable settings.

**Table**: `user_preferences`

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | INTEGER | PRIMARY KEY, AUTO_INCREMENT | Internal ID |
| key | TEXT | UNIQUE, NOT NULL | Setting key (e.g., "input_mode") |
| value | TEXT | NOT NULL | JSON-encoded value |
| updated_at | TEXT | NOT NULL | ISO 8601 timestamp |

**Predefined Keys**:

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `input_mode` | String | `"push_toTalk"` | `"push_to_talk"` or `"toggle"` |
| `idle_timeout_ms` | Integer | `1500` | Range: 500-5000ms |
| `polishing_intensity` | String | `"standard"` | `"light"`, `"standard"`, `"deep"` |
| `input_language` | String | `"auto"` | ISO code or `"auto"` for detection |
| `output_language` | String | `"same"` | ISO code or `"same"` for no translation |
| `translation_enabled` | Boolean | `false` | Enable translation feature |
| `noise_cancellation` | Boolean | `true` | Enable audio noise cancellation |
| `auto_process` | Boolean | `true` | Auto-trigger AI after recording |
| `filler_removal` | Boolean | `true` | Remove filler words |
| `context_aware` | Boolean | `true` | Adapt polishing to app context |
| `hotkey` | String | `"Ctrl+Shift+Space"` | Global hotkey combination |
| `history_retention_days` | Integer | `30` | 0=forever, 7/30/90 days |
| `crash_reporting` | Boolean | `true` | Anonymous crash reports |
| `usage_analytics` | Boolean | `false` | Anonymous usage stats |

### 2. DictionaryEntry

User-defined voice-to-text mappings for improved accuracy.

**Table**: `dictionary_entries`

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | INTEGER | PRIMARY KEY, AUTO_INCREMENT | Internal ID |
| voice_form | TEXT | NOT NULL | Spoken/nickname form |
| standard_form | TEXT | NOT NULL | Written form to output |
| created_at | TEXT | NOT NULL | ISO 8601 timestamp |
| updated_at | TEXT | NOT NULL | ISO 8601 timestamp |

**Indexes**:
- `idx_voice_form` on `voice_form` for fast lookup during transcription

**Example Entries**:
```
voice_form: "泰普勒斯" → standard_form: "Talkute"
voice_form: "AI" → standard_form: "artificial intelligence"
voice_form: "拉斯特" → standard_form: "Rust"
```

### 3. HistoryEntry

Persistent record of completed transcriptions.

**Table**: `history_entries`

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | TEXT | PRIMARY KEY | UUID v4 |
| timestamp | TEXT | NOT NULL, INDEX | ISO 8601 timestamp |
| app_context | TEXT | NOT NULL | Target application name |
| original_text | TEXT | NOT NULL | Raw ASR transcription |
| polished_text | TEXT | NOT NULL | AI-processed output |
| input_language | TEXT | NOT NULL | ISO 639-1 code |
| output_language | TEXT | NOT NULL | ISO 639-1 code |
| polishing_intensity | TEXT | NOT NULL | `light`/`standard`/`deep` |
| was_translated | INTEGER | NOT NULL | 0=false, 1=true |
| duration_ms | INTEGER | NOT NULL | Recording duration |
| success | INTEGER | NOT NULL | 0=failed, 1=succeeded |
| error_message | TEXT | NULL | Error if failed |

**Indexes**:
- `idx_timestamp` on `timestamp` DESC for date-grouped queries
- `idx_app_context` on `app_context` for filtering by app

**Retention Policy**:
- Default: 30 days
- Configurable: 7, 30, 90 days, forever, or disabled
- Cleanup runs on app startup via `cleanup.rs` module

### 4. TranscriptionSession (Runtime Only)

In-memory state for active recording session. Not persisted.

**Rust Struct**:
```rust
pub enum SessionState {
    Idle,
    Recording,
    Processing,
    Error,
}

pub struct TranscriptionSession {
    pub session_id: Uuid,
    pub state: SessionState,
    pub started_at: DateTime<Utc>,
    pub audio_buffer: Vec<u8>,
    pub detected_language: Option<String>,
    pub target_language: Option<String>,
    pub retry_count: u32,
    pub last_error: Option<String>,
}
```

**State Transitions**:
```
Idle ──[hotkey]──> Recording ──[release/timeout]──> Processing
  ↑                                                    │
  │                                                    │
  └──────────────────[success]─────────────────────────┘
                           │
                      [failure]
                           │
                           v
                        Error ──[retry]──> Recording
                           │
                      [dismiss]
                           │
                           v
                         Idle
```

## Enumerations

### PolishingIntensity

```rust
pub enum PolishingIntensity {
    Light,      // Remove filler words only
    Standard,   // Grammar + logic improvements
    Deep,       // Full rewrite for formality
}
```

### InputMode

```rust
pub enum InputMode {
    PushToTalk,  // Hold hotkey to record
    Toggle,      // Press to start/stop
}
```

### SessionState

```rust
pub enum SessionState {
    Idle,        // Ready to record
    Recording,   // Active audio capture
    Processing,  // AI/transcription in progress
    Error,       // Failure with retry option
}
```

### SupportedLanguage

```rust
pub struct Language {
    pub code: String,       // ISO 639-1 code
    pub name: String,       // Display name
    pub native_name: String, // Native name
}

pub const SUPPORTED_LANGUAGES: &[Language] = &[
    Language { code: "zh-CN", name: "Chinese (Simplified)", native_name: "简体中文" },
    Language { code: "zh-TW", name: "Chinese (Traditional)", native_name: "繁體中文" },
    Language { code: "en-US", name: "English", native_name: "English" },
    Language { code: "ja-JP", name: "Japanese", native_name: "日本語" },
    Language { code: "ko-KR", name: "Korean", native_name: "한국어" },
];
```

## Database Migrations

### Migration: 002_product_experience.sql

```sql
-- User preferences table (key-value store)
CREATE TABLE IF NOT EXISTS user_preferences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT UNIQUE NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Custom dictionary entries
CREATE TABLE IF NOT EXISTS dictionary_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    voice_form TEXT NOT NULL,
    standard_form TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_voice_form ON dictionary_entries(voice_form);

-- Transcription history
CREATE TABLE IF NOT EXISTS history_entries (
    id TEXT PRIMARY KEY,
    timestamp TEXT NOT NULL,
    app_context TEXT NOT NULL,
    original_text TEXT NOT NULL,
    polished_text TEXT NOT NULL,
    input_language TEXT NOT NULL,
    output_language TEXT NOT NULL,
    polishing_intensity TEXT NOT NULL,
    was_translated INTEGER NOT NULL DEFAULT 0,
    duration_ms INTEGER NOT NULL,
    success INTEGER NOT NULL DEFAULT 1,
    error_message TEXT
);

CREATE INDEX IF NOT EXISTS idx_timestamp ON history_entries(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_app_context ON history_entries(app_context);

-- Insert default preferences
INSERT OR IGNORE INTO user_preferences (key, value) VALUES
    ('input_mode', '"push_to_talk"'),
    ('idle_timeout_ms', '1500'),
    ('polishing_intensity', '"standard"'),
    ('input_language', '"auto"'),
    ('output_language', '"same"'),
    ('translation_enabled', 'false'),
    ('noise_cancellation', 'true'),
    ('auto_process', 'true'),
    ('filler_removal', 'true'),
    ('context_aware', 'true'),
    ('hotkey', '"Ctrl+Shift+Space"'),
    ('history_retention_days', '30'),
    ('crash_reporting', 'true'),
    ('usage_analytics', 'false');
```

## Validation Rules

### UserPreferences
- `idle_timeout_ms`: Must be 500-5000
- `polishing_intensity`: Must be one of `light`, `standard`, `deep`
- `input_language`: Must be valid ISO code or `auto`
- `output_language`: Must be valid ISO code or `same`
- `history_retention_days`: Must be 0, 7, 30, or 90

### DictionaryEntry
- `voice_form`: Required, 1-100 characters
- `standard_form`: Required, 1-200 characters
- Unique constraint on `(voice_form)`

### HistoryEntry
- `original_text`: Required, max 100KB
- `polished_text`: Required, max 100KB
- `input_language`/`output_language`: Required, valid ISO code

## Data Access Patterns

### Rust Repository Traits

```rust
#[async_trait]
pub trait PreferencesRepository: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>>;
    async fn set(&self, key: &str, value: &str) -> Result<()>;
    async fn get_all(&self) -> Result<HashMap<String, String>>;
}

#[async_trait]
pub trait DictionaryRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<DictionaryEntry>>;
    async fn add(&self, entry: DictionaryEntry) -> Result<()>;
    async fn update(&self, id: i64, entry: DictionaryEntry) -> Result<()>;
    async fn delete(&self, id: i64) -> Result<()>;
    async fn lookup(&self, voice_form: &str) -> Result<Option<String>>;
}

#[async_trait]
pub trait HistoryRepository: Send + Sync {
    async fn add(&self, entry: HistoryEntry) -> Result<()>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<HistoryEntry>>;
    async fn list_by_date(&self, date: Date) -> Result<Vec<HistoryEntry>>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn cleanup_old(&self, retention_days: i64) -> Result<u64>;
}
```

## Performance Considerations

- **History queries**: Indexed by timestamp, efficient date grouping
- **Dictionary lookup**: Indexed by voice_form, O(log n) lookup
- **Preferences**: In-memory cache with write-through to SQLite
- **Session state**: Purely in-memory, no persistence overhead
