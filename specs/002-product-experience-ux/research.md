# Research Findings: Product Experience Design Implementation

**Feature**: 002-product-experience-ux
**Date**: 2026-03-07
**Status**: Complete

## Executive Summary

Research completed for cross-platform desktop floating capsule, system tray integration, global hotkeys, text injection, and mobile keyboard input methods. All technical decisions align with existing Rust + Flutter architecture.

## Technical Decisions

### 1. System Tray Implementation

**Decision**: Use platform-specific crates with unified Rust trait interface

**Rationale**:
- `tray-item` for cross-platform tray support (Windows, macOS, Linux)
- Abstract behind `PlatformTray` trait for consistent API
- Each platform has different capabilities - abstract common operations

**Alternatives Considered**:
- `trayicon` - Less maintained, fewer features
- Native Flutter plugins - Limited platform support, Rust provides better control

**Implementation Notes**:
- Windows: Uses Win32 API via `winapi` crate
- macOS: Uses AppKit via `cocoa` crate
- Linux: Uses GTK/AppIndicator via `libappindicator-sys`

### 2. Global Hotkey Registration

**Decision**: Use `global-hotkey` crate with platform-specific backends

**Rationale**:
- Supports both press-and-hold (push-to-talk) and toggle modes
- Cross-platform abstraction with native implementations
- Low latency for responsive recording triggers

**Alternatives Considered**:
- `hotkey` - Simpler but less features
- Flutter keyboard listeners - Only works when app has focus

**Default Hotkey**:
- Windows/Linux: `Ctrl+Shift+Space`
- macOS: `Cmd+Shift+Space`

### 3. Text Injection at Cursor Position

**Decision**: Platform-specific text injection via simulated keyboard input

**Rationale**:
- Most reliable cross-application text insertion method
- Works with all text fields (native, web, electron apps)
- Clipboard fallback for restricted contexts

**Platform Implementation**:
- Windows: `SendInput` via `winapi` crate
- macOS: CGEvent keyboard simulation via `core-graphics`
- Linux: XTest extension via `x11` crate

**Alternatives Considered**:
- Accessibility APIs - Requires permissions, inconsistent across apps
- Clipboard-only - Requires user paste action
- Direct window text manipulation - Too fragile, app-specific

### 4. Floating Window Implementation

**Decision**: Flutter window with platform-specific overlay configuration

**Rationale**:
- Single Flutter codebase for all platforms
- Platform adapters handle window styling (acrylic/vibrancy)
- Consistent UI with rest of application

**Platform Styling**:
- Windows: Acrylic blur effect via DWM API
- macOS: Vibrancy effect via NSVisualEffectView
- Linux: Transparency with compositor support

**Window Properties**:
- Always-on-top flag
- Frameless/decorations removed
- Skip taskbar
- Centered on screen initially

### 5. Audio Waveform Visualization

**Decision**: Custom Flutter widget with Rust audio level streaming

**Rationale**:
- Real-time audio level data from Rust capture module
- Efficient data transfer via FFI stream
- Smooth 60fps animation in Flutter

**Implementation**:
- Rust: Calculate RMS levels from audio buffer (50ms intervals)
- FFI: Stream levels via `StreamSink<f32>`
- Flutter: Animated container with color/width based on level

### 6. Mobile Keyboard Implementation

**Decision**: Platform-specific keyboard extensions with shared Flutter UI

**Rationale**:
- iOS: Custom Keyboard Extension (required by iOS architecture)
- Android: InputMethodService (required by Android architecture)
- Shared UI components via Flutter where possible

**iOS Keyboard Extension**:
- Separate target in Xcode project
- Communicates with main app via shared container
- Full access mode for AI features

**Android IME**:
- Extends `InputMethodService`
- Custom keyboard layout XML
- Connection to Rust core via FFI

### 7. State Machine for Recording Flow

**Decision**: Explicit state machine with four primary states

**States**:
1. **Idle**: Ready to record, tray shows idle icon
2. **Recording**: Active capture, capsule shows waveform
3. **Processing**: AI/transcription in progress, spinning indicator
4. **Error**: Failure state with retry/dismiss options

**Transitions**:
- Idle → Recording: Hotkey press (push-to-talk) or press (toggle)
- Recording → Processing: Hotkey release (push-to-talk) or idle timeout
- Processing → Idle: Success, text injected
- Processing → Error: Network/API failure
- Error → Recording: Retry button pressed
- Error → Idle: Dismiss button pressed

### 8. Data Persistence Strategy

**Decision**: SQLite via existing `sqlx` crate with migrations

**Rationale**:
- Already integrated in project (from 001-ai-voice-input)
- Reliable, performant for local storage
- Supports complex queries for history filtering

**Schema Extensions**:
- `dictionary_entries` table: voice_form, standard_form, created_at
- `history_entries` table: timestamp, app_context, original_text, polished_text, input_lang, output_lang
- `user_preferences` table: key-value pairs for settings

### 9. Language Detection Strategy

**Decision**: Azure Speech Language ID + manual override

**Rationale**:
- Azure Speech SDK provides language identification
- Manual selection for edge cases
- User preference for default language

**Supported Languages** (initial):
- Chinese (Simplified): `zh-CN`
- Chinese (Traditional): `zh-TW`
- English: `en-US`
- Japanese: `ja-JP`
- Korean: `ko-KR`

### 10. Offline Handling

**Decision**: Graceful degradation with cached transcription option

**Rationale**:
- Network detection via Rust `is_online` check
- Queue transcription for retry when online
- Clear error feedback in capsule UI

**Implementation**:
- Check network before API calls
- Cache failed requests in SQLite
- Retry queue processed on connectivity restore

## Best Practices Applied

### Rust Core Patterns
- Async/await with tokio for all I/O operations
- Error handling via `thiserror` for custom errors
- Trait-based platform abstraction for portability
- Logging via `tracing` crate for observability

### Flutter UI Patterns
- Riverpod for state management
- Immutable state with `freezed` for data classes
- Platform-adaptive widgets where needed
- Accessibility via platform-level features only

### FFI Boundary
- All FFI calls return `Result<T, String>` for error handling
- Streams for continuous data (audio levels, transcription)
- Primitive types only across boundary
- Async operations handled via Rust futures

## Dependencies Confirmed

### Rust Crates (New)
- `tray-item` - System tray
- `global-hotkey` - Hotkey registration
- `enigo` - Cross-platform input simulation (alternative to raw platform APIs)
- `x11` - Linux X11 support
- `winapi` - Windows API
- `cocoa` / `core-graphics` - macOS support

### Flutter Packages (New)
- `window_manager` - Window control (always-on-top, position)
- `flutter_local_notifications` - System notifications
- No new major packages needed - using existing flutter_rust_bridge

## Risk Mitigations

| Risk | Mitigation Strategy |
|------|---------------------|
| Platform-specific text injection varies | Abstract behind trait, test on all platforms |
| Global hotkey conflicts | Allow customization, detect and warn |
| Mobile keyboard limitations | Document clearly, provide desktop fallback |
| AI API latency | Show processing state, optimize prompts |
| Audio permission denied | Graceful error with clear instructions |

## Open Items

None - all technical questions resolved during planning phase.
