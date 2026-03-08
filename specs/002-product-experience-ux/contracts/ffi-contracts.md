# FFI Contracts: Product Experience Design Implementation

**Feature**: 002-product-experience-ux
**Date**: 2026-03-07
**Bridge**: flutter_rust_bridge v2.11+

## Overview

This document defines the FFI boundary contracts between Flutter UI and Rust core for the Product Experience Design feature. All functions are exposed via `flutter_rust_bridge` with proper error handling.

## Contract Principles

1. **Primitive Types Only**: All parameters and returns use primitive types or simple structs
2. **Result Types**: All functions return `Result<T, String>` for error propagation
3. **Async Operations**: Long-running operations use async/await
4. **Streams**: Continuous data (audio levels, state changes) use `StreamSink`

## FFI Functions

### 1. Session Management

#### `start_recording`

Start a new recording session.

```rust
#[frb]
pub async fn start_recording() -> Result<String, String>;
```

**Returns**: Session ID (UUID string)

**Side Effects**:
- Initializes audio capture
- Changes session state to `Recording`
- Updates tray icon to recording state

**Errors**:
- `"Microphone not available"` - No microphone or permission denied
- `"Already recording"` - Session already in progress

---

#### `stop_recording`

Stop the current recording session.

```rust
#[frb]
pub async fn stop_recording(session_id: String) -> Result<(), String>;
```

**Parameters**:
- `session_id`: ID from `start_recording`

**Side Effects**:
- Stops audio capture
- Changes state to `Processing`
- Triggers transcription pipeline

**Errors**:
- `"Invalid session"` - Session ID doesn't match active session
- `"Not recording"` - No active recording to stop

---

#### `cancel_recording`

Cancel recording without processing.

```rust
#[frb]
pub async fn cancel_recording(session_id: String) -> Result<(), String>;
```

**Side Effects**:
- Discards audio buffer
- Returns to `Idle` state

---

#### `retry_processing`

Retry failed processing.

```rust
#[frb]
pub async fn retry_processing(session_id: String) -> Result<(), String>;
```

**Side Effects**:
- Retries from `Error` state
- Increments retry count

---

### 2. State Observation

#### `session_state_stream`

Stream of session state changes.

```rust
#[frb]
pub fn session_state_stream(sink: StreamSink<SessionStateEvent>);
```

**Event Structure**:
```dart
class SessionStateEvent {
  String sessionId;
  String state; // "idle", "recording", "processing", "error"
  String? errorMessage;
  double? progress; // 0.0 to 1.0 for processing progress
}
```

**Usage**: Flutter subscribes to update UI state.

---

#### `audio_level_stream`

Stream of real-time audio levels for visualization.

```rust
#[frb]
pub fn audio_level_stream(sink: StreamSink<f32>);
```

**Values**: 0.0 (silence) to 1.0 (max), updated every 50ms

**Usage**: Flutter visualizes waveform/amplitude.

---

### 3. Text Injection

#### `inject_text`

Inject text at cursor position.

```rust
#[frb]
pub async fn inject_text(text: String) -> Result<(), String>;
```

**Side Effects**:
- Simulates keyboard input at cursor
- Falls back to clipboard if injection fails

**Errors**:
- `"Injection failed"` - Platform injection error
- Returns success even if clipboard fallback used

---

#### `copy_to_clipboard`

Copy text to system clipboard.

```rust
#[frb]
pub async fn copy_to_clipboard(text: String) -> Result<(), String>;
```

**Usage**: Fallback when text injection fails.

---

### 4. Preferences Management

#### `get_preference`

Get a preference value.

```rust
#[frb]
pub async fn get_preference(key: String) -> Result<String?, String>;
```

**Returns**: JSON-encoded value or null if not set

---

#### `set_preference`

Set a preference value.

```rust
#[frb]
pub async fn set_preference(key: String, value: String) -> Result<(), String>;
```

**Parameters**:
- `key`: Preference key (see data-model.md)
- `value`: JSON-encoded value

**Side Effects**: Persists to SQLite

---

#### `get_all_preferences`

Get all preferences.

```rust
#[frb]
pub async fn get_all_preferences() -> Result<HashMap<String, String>, String>;
```

**Returns**: Map of all key-value pairs

---

### 5. Dictionary Management

#### `list_dictionary_entries`

List all dictionary entries.

```rust
#[frb]
pub async fn list_dictionary_entries() -> Result<Vec<DictionaryEntry>, String>;
```

**Structure**:
```dart
class DictionaryEntry {
  int id;
  String voiceForm;
  String standardForm;
  String createdAt;
  String updatedAt;
}
```

---

#### `add_dictionary_entry`

Add a new dictionary entry.

```rust
#[frb]
pub async fn add_dictionary_entry(
  voice_form: String,
  standard_form: String
) -> Result<i64, String>;
```

**Returns**: New entry ID

**Errors**:
- `"Duplicate entry"` - Voice form already exists

---

#### `update_dictionary_entry`

Update an existing entry.

```rust
#[frb]
pub async fn update_dictionary_entry(
  id: i64,
  voice_form: String,
  standard_form: String
) -> Result<(), String>;
```

---

#### `delete_dictionary_entry`

Delete an entry.

```rust
#[frb]
pub async fn delete_dictionary_entry(id: i64) -> Result<(), String>;
```

---

### 6. History Management

#### `list_history`

List history entries with pagination.

```rust
#[frb]
pub async fn list_history(
  limit: i32,
  offset: i32
) -> Result<Vec<HistoryEntry>, String>;
```

**Structure**:
```dart
class HistoryEntry {
  String id;
  String timestamp;
  String appContext;
  String originalText;
  String polishedText;
  String inputLanguage;
  String outputLanguage;
  String polishingIntensity;
  bool wasTranslated;
  int durationMs;
  bool success;
  String? errorMessage;
}
```

---

#### `list_history_by_date`

List entries for a specific date.

```rust
#[frb]
pub async fn list_history_by_date(date: String) -> Result<Vec<HistoryEntry>, String>;
```

**Parameters**:
- `date`: ISO date string (YYYY-MM-DD)

---

#### `delete_history_entry`

Delete a history entry.

```rust
#[frb]
pub async fn delete_history_entry(id: String) -> Result<(), String>;
```

---

#### `clear_all_history`

Clear all history entries.

```rust
#[frb]
pub async fn clear_all_history() -> Result<(), String>;
```

---

### 7. System Tray

#### `set_tray_icon`

Update tray icon state.

```rust
#[frb]
pub fn set_tray_icon(state: String) -> Result<(), String>;
```

**States**: `"idle"`, `"recording"`, `"processing"`, `"error"`

---

#### `set_tray_menu`

Update tray context menu.

```rust
#[frb]
pub fn set_tray_menu(items: Vec<TrayMenuItem>) -> Result<(), String>;
```

**Structure**:
```dart
class TrayMenuItem {
  String id;
  String label;
  bool enabled;
  bool isSeparator;
}
```

---

### 8. Hotkey Management

#### `register_hotkey`

Register global hotkey.

```rust
#[frb]
pub fn register_hotkey(hotkey: String) -> Result<(), String>;
```

**Parameters**:
- `hotkey`: Key combination (e.g., "Ctrl+Shift+Space")

**Errors**:
- `"Hotkey conflict"` - Key already registered by another app
- `"Invalid hotkey"` - Malformed hotkey string

---

#### `hotkey_event_stream`

Stream of hotkey events.

```rust
#[frb]
pub fn hotkey_event_stream(sink: StreamSink<HotkeyEvent>);
```

**Structure**:
```dart
class HotkeyEvent {
  String hotkey;
  String action; // "pressed", "released"
}
```

---

### 9. Window Management

#### `show_floating_capsule`

Show floating capsule window.

```rust
#[frb]
pub fn show_floating_capsule() -> Result<(), String>;
```

**Side Effects**:
- Makes window visible
- Centers on screen
- Sets always-on-top

---

#### `hide_floating_capsule`

Hide floating capsule window.

```rust
#[frb]
pub fn hide_floating_capsule() -> Result<(), String>;
```

---

#### `set_capsule_state`

Update capsule visual state.

```rust
#[frb]
pub fn set_capsule_state(state: String) -> Result<(), String>;
```

**States**: `"idle"`, `"recording"`, `"processing"`, `"success"`, `"error"`

---

### 10. Context Detection

#### `get_active_application`

Get the currently focused application.

```rust
#[frb]
pub async fn get_active_application() -> Result<ApplicationInfo, String>;
```

**Structure**:
```dart
class ApplicationInfo {
  String name;
  String? bundleId; // macOS/iOS
  String? processId;
  String category; // "chat", "email", "document", "code", "browser", "other"
}
```

---

### 11. Data Export/Import

#### `export_data`

Export user data.

```rust
#[frb]
pub async fn export_data(format: String) -> Result<String, String>;
```

**Parameters**:
- `format`: `"json"` or `"csv"`

**Returns**: File path to exported data

---

#### `import_dictionary`

Import dictionary entries.

```rust
#[frb]
pub async fn import_dictionary(json_data: String) -> Result<i32, String>;
```

**Returns**: Number of entries imported

---

### 12. Cleanup

#### `run_cleanup`

Run data cleanup based on retention policy.

```rust
#[frb]
pub async fn run_cleanup() -> Result<u64, String>;
```

**Returns**: Number of entries removed

---

## Error Handling Convention

All FFI functions return `Result<T, String>`:

```rust
// Success
Ok(value)

// Error with message
Err("Error type: details")
```

**Error Categories**:
- `"Microphone not available"` - Audio hardware/permission issues
- `"Network error"` - API connectivity issues
- `"API error: {details}"` - External API failures
- `"Invalid {field}"` - Validation failures
- `"Not found"` - Resource not found
- `"Already exists"` - Duplicate resource
- `"Permission denied"` - OS permission issues

## Type Mapping Reference

| Rust | Dart |
|------|------|
| `String` | `String` |
| `i32`, `i64` | `int` |
| `f32`, `f64` | `double` |
| `bool` | `bool` |
| `Vec<T>` | `List<T>` |
| `HashMap<K, V>` | `Map<K, V>` |
| `Option<T>` | `T?` |
| `Result<T, String>` | `Future<T>` (throws on error) |
| `StreamSink<T>` | `Stream<T>` |

## Testing Requirements

Each FFI function must have:

1. **Unit test** in Rust (mocked dependencies)
2. **Integration test** with actual FFI bridge
3. **Error case test** for each error variant

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_start_recording_success() {
        let result = start_recording().await;
        assert!(result.is_ok());
        assert!(uuid::Uuid::parse_str(&result.unwrap()).is_ok());
    }

    #[tokio::test]
    async fn test_start_recording_no_microphone() {
        // Mock microphone unavailable
        let result = start_recording().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Microphone"));
    }
}
```
