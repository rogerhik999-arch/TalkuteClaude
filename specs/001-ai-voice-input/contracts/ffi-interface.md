# FFI Interface Contract: Flutter ↔ Rust

**Feature**: 001-ai-voice-input | **Date**: 2026-03-04 | **Spec**: [spec.md](../spec.md)

## Overview

This document defines the Foreign Function Interface (FFI) contract between the Flutter UI layer and the Rust core logic layer using `flutter_rust_bridge` v2.11+. All async operations use Dart `Future` types, and all data structures are serializable via JSON.

## Bridge Architecture

```text
Flutter (Dart)          FFI Bridge              Rust Core
─────────────────       ──────────────          ─────────────
UI Components    ←──→   flutter_rust_bridge  ←──→  Core Logic
State Management        (Auto-generated)           Business Logic
Platform Services       Type-safe bindings         Platform APIs
```

## Core Principles

1. **Async-First**: All I/O operations (speech API, AI API, database) return `Future<T>` in Dart
2. **Error Handling**: All fallible operations return `Result<T, E>` in Rust, mapped to Dart exceptions
3. **Type Safety**: Strong typing enforced at compile time via code generation
4. **Zero-Copy**: Large data (audio buffers) passed via pointers, not serialization
5. **Thread Safety**: Rust handles all concurrency, Flutter receives callbacks on main isolate

## Data Transfer Objects (DTOs)

### VoiceSessionDto

```rust
// Rust definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSessionDto {
    pub session_id: String,
    pub device_id: String,
    pub context_id: Option<String>,
    pub started_at: String,  // ISO 8601 format
    pub ended_at: Option<String>,
    pub duration_seconds: Option<u32>,
    pub raw_transcription: Option<String>,
    pub polished_text: Option<String>,
    pub word_count: u32,
    pub status: SessionStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Recording,
    Transcribing,
    Polishing,
    Completed,
    Failed,
    Cancelled,
}
```

```dart
// Dart equivalent (auto-generated)
class VoiceSessionDto {
  final String sessionId;
  final String deviceId;
  final String? contextId;
  final String startedAt;
  final String? endedAt;
  final int? durationSeconds;
  final String? rawTranscription;
  final String? polishedText;
  final int wordCount;
  final SessionStatus status;
  final String? errorMessage;
}

enum SessionStatus {
  recording,
  transcribing,
  polishing,
  completed,
  failed,
  cancelled,
}
```

### ApplicationContextDto

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationContextDto {
    pub context_id: String,
    pub application_name: String,
    pub application_title: Option<String>,
    pub application_category: AppCategory,
    pub preferred_tone: Option<TonePreference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppCategory {
    Email,
    Chat,
    Document,
    Code,
    Browser,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TonePreference {
    Formal,
    Casual,
    Technical,
    Creative,
}
```

### DeviceProfileDto

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfileDto {
    pub device_id: String,
    pub preferred_language: String,
    pub voice_speed_preference: f32,
    pub auto_punctuation_enabled: bool,
    pub filler_removal_enabled: bool,
    pub self_correction_enabled: bool,
    pub crash_reporting_enabled: bool,
}
```

### UsageQuotaDto

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageQuotaDto {
    pub words_used_this_week: u32,
    pub weekly_limit: u32,
    pub current_week_start: String,  // ISO 8601 date
    pub percentage_used: f32,
    pub is_quota_exceeded: bool,
}
```

### PersonalDictionaryEntryDto

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDictionaryEntryDto {
    pub entry_id: String,
    pub phrase: String,
    pub replacement: String,
    pub case_sensitive: bool,
    pub whole_word_only: bool,
    pub usage_count: u32,
}
```

## FFI API Surface

### Session Management

```rust
/// Initialize a new voice input session
#[flutter_rust_bridge::frb(sync)]
pub async fn start_voice_session(
    device_id: String,
    context_id: Option<String>,
) -> Result<VoiceSessionDto, SessionError>;

/// Stop the current voice session
#[flutter_rust_bridge::frb(sync)]
pub async fn stop_voice_session(
    session_id: String,
) -> Result<VoiceSessionDto, SessionError>;

/// Cancel the current voice session
#[flutter_rust_bridge::frb(sync)]
pub async fn cancel_voice_session(
    session_id: String,
) -> Result<(), SessionError>;

/// Get session by ID
#[flutter_rust_bridge::frb(sync)]
pub async fn get_voice_session(
    session_id: String,
) -> Result<VoiceSessionDto, SessionError>;

/// Get recent sessions (last N)
#[flutter_rust_bridge::frb(sync)]
pub async fn get_recent_sessions(
    device_id: String,
    limit: u32,
) -> Result<Vec<VoiceSessionDto>, SessionError>;
```

### Audio Processing

```rust
/// Start audio capture from microphone
#[flutter_rust_bridge::frb(sync)]
pub async fn start_audio_capture() -> Result<(), AudioError>;

/// Stop audio capture
#[flutter_rust_bridge::frb(sync)]
pub async fn stop_audio_capture() -> Result<(), AudioError>;

/// Get current audio level (0.0 - 1.0) for UI visualization
#[flutter_rust_bridge::frb(sync)]
pub fn get_audio_level() -> f32;
```

### Transcription & AI Processing

```rust
/// Transcribe audio to text using speech API
#[flutter_rust_bridge::frb(sync)]
pub async fn transcribe_audio(
    session_id: String,
) -> Result<String, TranscriptionError>;

/// Polish text using AI
#[flutter_rust_bridge::frb(sync)]
pub async fn polish_text(
    session_id: String,
    raw_text: String,
    context: Option<ApplicationContextDto>,
) -> Result<String, AiError>;

/// Apply personal dictionary replacements
#[flutter_rust_bridge::frb(sync)]
pub async fn apply_dictionary(
    device_id: String,
    text: String,
) -> Result<String, DictionaryError>;
```

### Context Detection

```rust
/// Detect current application context
#[flutter_rust_bridge::frb(sync)]
pub async fn detect_application_context() -> Result<ApplicationContextDto, ContextError>;

/// Get all known application contexts
#[flutter_rust_bridge::frb(sync)]
pub async fn get_all_contexts(
    device_id: String,
) -> Result<Vec<ApplicationContextDto>, ContextError>;

/// Update context preferences
#[flutter_rust_bridge::frb(sync)]
pub async fn update_context_preferences(
    context_id: String,
    preferred_tone: Option<TonePreference>,
    custom_instructions: Option<String>,
) -> Result<(), ContextError>;
```

### Device Profile Management

```rust
/// Get device profile
#[flutter_rust_bridge::frb(sync)]
pub async fn get_device_profile(
    device_id: String,
) -> Result<DeviceProfileDto, ProfileError>;

/// Update device profile
#[flutter_rust_bridge::frb(sync)]
pub async fn update_device_profile(
    profile: DeviceProfileDto,
) -> Result<(), ProfileError>;

/// Initialize device profile (first launch)
#[flutter_rust_bridge::frb(sync)]
pub async fn initialize_device_profile() -> Result<DeviceProfileDto, ProfileError>;
```

### Usage Quota Management

```rust
/// Get current usage quota
#[flutter_rust_bridge::frb(sync)]
pub async fn get_usage_quota(
    device_id: String,
) -> Result<UsageQuotaDto, QuotaError>;

/// Check if quota allows N words
#[flutter_rust_bridge::frb(sync)]
pub async fn check_quota_available(
    device_id: String,
    word_count: u32,
) -> Result<bool, QuotaError>;

/// Increment usage quota
#[flutter_rust_bridge::frb(sync)]
pub async fn increment_usage_quota(
    device_id: String,
    word_count: u32,
) -> Result<UsageQuotaDto, QuotaError>;
```

### Personal Dictionary Management

```rust
/// Add dictionary entry
#[flutter_rust_bridge::frb(sync)]
pub async fn add_dictionary_entry(
    device_id: String,
    phrase: String,
    replacement: String,
    case_sensitive: bool,
    whole_word_only: bool,
) -> Result<PersonalDictionaryEntryDto, DictionaryError>;

/// Remove dictionary entry
#[flutter_rust_bridge::frb(sync)]
pub async fn remove_dictionary_entry(
    entry_id: String,
) -> Result<(), DictionaryError>;

/// Get all dictionary entries
#[flutter_rust_bridge::frb(sync)]
pub async fn get_all_dictionary_entries(
    device_id: String,
) -> Result<Vec<PersonalDictionaryEntryDto>, DictionaryError>;

/// Update dictionary entry
#[flutter_rust_bridge::frb(sync)]
pub async fn update_dictionary_entry(
    entry: PersonalDictionaryEntryDto,
) -> Result<(), DictionaryError>;
```

## Error Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionError {
    NotFound,
    AlreadyActive,
    InvalidState,
    DatabaseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioError {
    DeviceNotFound,
    PermissionDenied,
    CaptureError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscriptionError {
    ApiError(String),
    NetworkError(String),
    InvalidAudio,
    QuotaExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiError {
    ApiError(String),
    NetworkError(String),
    InvalidInput,
    QuotaExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextError {
    DetectionFailed(String),
    PermissionDenied,
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProfileError {
    NotFound,
    InvalidData(String),
    DatabaseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuotaError {
    Exceeded,
    NotFound,
    DatabaseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DictionaryError {
    DuplicateEntry,
    NotFound,
    InvalidData(String),
    DatabaseError(String),
}
```

## Stream-Based APIs (Real-Time Updates)

```rust
/// Stream of transcription updates (partial results)
#[flutter_rust_bridge::frb(sync)]
pub async fn stream_transcription_updates(
    session_id: String,
) -> impl Stream<Item = String>;

/// Stream of audio level updates for visualization
#[flutter_rust_bridge::frb(sync)]
pub fn stream_audio_levels() -> impl Stream<Item = f32>;

/// Stream of session status updates
#[flutter_rust_bridge::frb(sync)]
pub async fn stream_session_status(
    session_id: String,
) -> impl Stream<Item = SessionStatus>;
```

## Code Generation

### Build Process

1. Define Rust API in `rust-core/src/ffi/bridge.rs`
2. Run `flutter_rust_bridge_codegen` to generate Dart bindings
3. Generated files:
   - `flutter-ui/lib/bridge_generated.dart` (Dart bindings)
   - `rust-core/src/ffi/bridge_generated.rs` (Rust glue code)

### Build Command

```bash
flutter_rust_bridge_codegen \
  --rust-input rust-core/src/ffi/bridge.rs \
  --dart-output flutter-ui/lib/bridge_generated.dart \
  --dart-decl-output flutter-ui/lib/bridge_definitions.dart
```

## Testing Strategy

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_start_voice_session() {
        let result = start_voice_session("test-device".to_string(), None).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests (Flutter)

```dart
void main() {
  test('FFI: start voice session', () async {
    final session = await startVoiceSession('test-device', null);
    expect(session.status, SessionStatus.recording);
  });
}
```

## Performance Considerations

- **Async Operations**: All I/O operations are async to prevent UI blocking
- **Batch Updates**: Use streams for high-frequency updates (audio levels, partial transcriptions)
- **Memory Management**: Large audio buffers passed via pointers, not copied
- **Error Propagation**: Errors mapped to Dart exceptions with detailed messages
- **Thread Safety**: Rust handles all concurrency, Flutter receives callbacks on main isolate

## Versioning

- **FFI Contract Version**: 1.0.0
- **Breaking Changes**: Increment major version
- **Additions**: Increment minor version
- **Bug Fixes**: Increment patch version

## Migration Path

When FFI contract changes:
1. Update Rust API definitions
2. Regenerate bindings with `flutter_rust_bridge_codegen`
3. Update Flutter code to match new API
4. Run integration tests to verify compatibility
5. Document breaking changes in CHANGELOG.md
