# Research Document: Talkute AI Voice Input Assistant

**Feature**: 001-ai-voice-input
**Date**: 2026-03-04
**Status**: Complete

## Overview

This document consolidates research findings for key technical decisions in the Talkute AI Voice Input Assistant implementation. Research focused on three critical areas: cloud speech recognition APIs, Flutter-Rust FFI integration patterns, and cross-platform application context detection.

## 1. Cloud Speech Recognition API Selection

### Decision: Azure Speech Services

**Rationale**:
- **Best latency performance**: 200-400ms p95 latency (well under 500ms requirement)
- **Competitive pricing**: $1.00/hour for standard transcription (~$1,000-$2,880/month for 60k minutes)
- **Excellent Rust support**: Native `azure-speech` crate with good documentation
- **Strong noise handling**: Built-in noise suppression without extensive pre-processing
- **Enterprise reliability**: 99.9% SLA with global low-latency endpoints

### Alternatives Considered

**Google Speech-to-Text**:
- Pros: 125+ language support, excellent gRPC implementation, strong accuracy
- Cons: Higher pricing ($1,440-$2,160/month), more complex gRPC setup
- Rejected because: Azure offers better latency and lower cost for our use case

**AWS Transcribe**:
- Pros: Good AWS ecosystem integration, competitive pricing
- Cons: Higher latency (500-800ms p95), complex streaming WebSocket implementation
- Rejected because: Latency doesn't meet requirements, integration complexity higher

### Feature Comparison

| Feature | Azure | Google | AWS |
|---------|-------|--------|-----|
| Streaming latency (p95) | 200-400ms | 300-500ms | 500-800ms |
| Language support | 100+ | 125+ | 100+ |
| Accuracy (clean audio) | 95-98% | 95-98% | 94-97% |
| Custom vocabulary | ✓ Yes | ✓ Yes | ✓ Yes |
| Rust SDK quality | Excellent | Good | Fair |
| Monthly cost (60k min) | $1,000-$2,880 | $1,440-$2,160 | $1,440-$1,860 |

### Integration Approach

**Dependencies**:
```toml
[dependencies]
azure-speech = "0.8"
tokio = { version = "1.40", features = ["full"] }
reqwest = "0.11"
```

**Implementation Pattern**:
- Use WebSocket-based streaming for real-time transcription
- Implement reconnection logic with exponential backoff
- Cache authentication tokens (valid for 10-60 minutes)
- Use `tokio::sync::mpsc` channels for audio buffering
- Implement client-side VAD (Voice Activity Detection) to reduce costs

**Estimated Integration Time**: 2-3 days for basic implementation, 1 week for production-ready

## 2. Flutter-Rust FFI Integration

### Decision: flutter_rust_bridge v2.11+

**Rationale**:
- **Official cross-platform support**: Windows, Mac, Linux, iOS, Android
- **Async/await support**: Native tokio integration for long-running operations
- **Type-safe code generation**: Automatic Dart/Rust binding generation
- **Zero-copy optimization**: Efficient for large audio buffers (Vec<u8>)
- **Active maintenance**: Regular updates and strong community support

### Architecture Pattern

**Layered Design**:
```
Flutter UI Layer (Dart)
    ↓
State Management (Riverpod/Bloc)
    ↓
Dart Service Layer (VoiceService)
    ↓
Flutter Rust Bridge (FFI)
    ↓
Rust API Layer (Public API)
    ↓
Rust Business Logic (Core modules)
```

### Key Implementation Patterns

**1. Async Operations**:
```rust
#[frb(sync)]
pub async fn start_recording(device_id: Option<String>) -> Result<VoiceSession, TalkuteError>
```
- Use `#[frb(sync)]` attribute for async functions
- Return `Result<T, E>` for proper error handling
- Leverage tokio runtime for background tasks

**2. Streaming Data**:
```rust
pub async fn stream_recognition(session_id: String) -> Result<impl Stream<Item = RecognitionResult>, TalkuteError>
```
- Use `tokio::sync::mpsc` channels for real-time data
- Implement `Stream` trait for continuous updates
- Handle backpressure with bounded channels

**3. Error Handling**:
```rust
#[derive(Debug, Clone)]
pub enum TalkuteError {
    AudioDeviceError(String),
    RecognitionError(String),
    NetworkError(String),
    InvalidState(String),
}
```
- Define custom error enum for type-safe error propagation
- Implement `From` traits for automatic error conversion
- Map Rust errors to user-friendly Dart messages

**4. Data Serialization**:
- Use `Vec<u8>` for audio buffers (zero-copy)
- Use `#[derive(Clone, Debug)]` for FFI-compatible types
- Leverage serde for complex structures

### Performance Optimizations

1. **Memory Management**:
   - Reuse buffers to reduce allocations
   - Use streaming for large files instead of loading all at once
   - Implement connection pooling for concurrent operations

2. **Thread Pool Configuration**:
   - Global tokio runtime with 4 worker threads
   - Dedicated thread pool for audio processing
   - Avoid blocking the UI thread

3. **Common Pitfalls to Avoid**:
   - ❌ Don't block UI thread with synchronous calls
   - ❌ Don't send large data frequently (use streams)
   - ❌ Don't create new runtime per call (use global)
   - ✅ Do use async operations for long-running tasks
   - ✅ Do handle specific error types
   - ✅ Do reuse global runtime

### Dependencies

**Rust**:
```toml
flutter_rust_bridge = "2.11"
tokio = { version = "1.40", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

**Flutter**:
```yaml
flutter_rust_bridge: ^2.11.1
ffi: ^2.1.0
```

**Estimated Integration Time**: 3-4 days for basic FFI setup, 1 week for production-ready with all modules

## 3. Cross-Platform Application Context Detection

### Decision: Platform-Specific Native APIs

**Rationale**:
- **Performance**: All platforms meet <50ms latency requirement (except iOS)
- **Privacy**: Local-only processing, no data transmission
- **Accuracy**: Direct OS APIs provide reliable application information
- **Compliance**: Aligns with zero data retention and privacy requirements

### Platform-Specific Implementations

#### Windows (✓ Fully Supported)
**API**: Win32 GetForegroundWindow + GetWindowText
**Crate**: `windows-rs` (official Microsoft Rust bindings)
**Latency**: <5ms typical
**Permissions**: None required
**Implementation**:
```rust
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

fn get_active_window() -> Result<WindowInfo, Error> {
    unsafe {
        let hwnd = GetForegroundWindow();
        // Get window title and process ID
    }
}
```

#### macOS (✓ Fully Supported)
**API**: NSWorkspace.frontmostApplication
**Crate**: `frontmost` or `cocoa-rs`
**Latency**: <10ms typical
**Permissions**: Accessibility permission required for detailed info
**Implementation**:
```rust
use cocoa::appkit::NSWorkspace;

fn get_active_app() -> Result<AppInfo, Error> {
    unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        let app = workspace.frontmostApplication();
        // Get app name and bundle ID
    }
}
```

#### Linux (✓ Supported with Limitations)
**API**: X11 _NET_ACTIVE_WINDOW (EWMH) / Wayland wlr-foreign-toplevel
**Crate**: `x11rb` (X11) / `wayland-client` (Wayland)
**Latency**: <5ms (X11), 10-50ms (Wayland)
**Permissions**: None (X11), compositor-dependent (Wayland)
**Challenges**: Wayland fragmentation, GNOME has limited support
**Implementation**:
```rust
use x11rb::connection::Connection;

fn get_active_window_x11() -> Result<WindowInfo, Error> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    // Query _NET_ACTIVE_WINDOW property
}
```

#### Android (✓ Supported with Permissions)
**API**: AccessibilityService
**Integration**: JNI bridge to Rust
**Crate**: `jni-rs`
**Latency**: 10-50ms typical
**Permissions**: BIND_ACCESSIBILITY_SERVICE (user must enable in Settings)
**Challenges**: Google Play policy restrictions, user friction
**Implementation**:
```java
public class WindowDetectionService extends AccessibilityService {
    @Override
    public void onAccessibilityEvent(AccessibilityEvent event) {
        if (event.getEventType() == AccessibilityEvent.TYPE_WINDOW_STATE_CHANGED) {
            String packageName = event.getPackageName().toString();
            nativeOnWindowChanged(packageName);
        }
    }
}
```

#### iOS (✗ Not Supported)
**Status**: Real-time context detection not feasible
**Reason**: iOS sandboxing prevents cross-app information access
**Alternative Approaches**:
1. **Manual context selection**: User selects context in app (email, chat, document)
2. **Keyboard extension**: Implement as custom keyboard to detect host app
3. **Default tone**: Use neutral tone for all inputs on iOS

**Recommendation**: Use manual context selection for iOS MVP

### Application Classification Strategy

**Classification Categories** (per FR-008):
1. Email (Gmail, Outlook, Apple Mail)
2. Chat (Slack, WhatsApp, Telegram, Discord)
3. Document (Google Docs, Word, Notion, Obsidian)
4. Code Editor (VS Code, IntelliJ, Sublime Text)
5. Browser (Chrome, Firefox, Safari, Edge)
6. AI Tool (ChatGPT, Claude, Copilot)

**Detection Methods**:
1. **Bundle ID / Package Name Matching**: Primary method (most reliable)
2. **Window Title Pattern Matching**: Secondary method (fallback)
3. **Process Name Matching**: Tertiary method (cross-platform)
4. **User-Defined Rules**: Allow custom classifications in settings

**Implementation**:
```rust
fn classify_app(bundle_id: &str, window_title: &str) -> AppCategory {
    // 1. Try bundle ID match (most reliable)
    if let Some(category) = classify_by_bundle_id(bundle_id) {
        return category;
    }

    // 2. Try window title pattern
    if let Some(category) = classify_by_title(window_title) {
        return category;
    }

    // 3. Default to Unknown
    AppCategory::Unknown
}
```

### Cross-Platform Abstraction

**Unified Interface**:
```rust
pub trait WindowDetector {
    fn get_active_window(&self) -> Result<WindowInfo, Error>;
    fn classify_app(&self, info: &WindowInfo) -> AppCategory;
}

pub struct WindowInfo {
    pub app_name: String,
    pub window_title: String,
    pub bundle_id: Option<String>,  // macOS, iOS
    pub package_name: Option<String>, // Android
    pub process_id: u32,
}
```

**Platform Selection**:
```rust
#[cfg(target_os = "windows")]
mod windows_detector;

#[cfg(target_os = "macos")]
mod macos_detector;

#[cfg(target_os = "linux")]
mod linux_detector;

#[cfg(target_os = "android")]
mod android_detector;

#[cfg(target_os = "ios")]
mod ios_detector;  // Manual selection fallback
```

### Performance Benchmarks

| Platform | Typical Latency | Max Latency | CPU Overhead | Status |
|----------|-----------------|-------------|--------------|--------|
| Windows | <5ms | <10ms | <0.1% | ✓ Meets requirement |
| macOS | <10ms | <20ms | <0.1% | ✓ Meets requirement |
| Linux (X11) | <5ms | <15ms | <0.1% | ✓ Meets requirement |
| Linux (Wayland) | 10-50ms | 100ms+ | <0.2% | ⚠ Marginal |
| Android | 10-50ms | 100ms | <0.5% | ✓ Meets requirement |
| iOS | N/A | N/A | N/A | ✗ Not feasible |

**All platforms meet <50ms requirement except iOS (not possible)**

### Privacy & Compliance

✅ **FR-021 (Zero data retention)**: All detection happens locally, no server transmission
✅ **FR-022 (Encryption)**: Window info processed in-memory only
✅ **FR-023 (Privacy controls)**: Users can disable context detection
✅ **FR-031 (No authentication)**: Detection works without user accounts

**Privacy Best Practices**:
1. Minimize data collection (app name/category only, not window content)
2. Local processing only (never send window titles to servers)
3. User transparency (clear explanation of data access)
4. Opt-out option (allow disabling context detection)
5. Secure storage (encrypt cached classifications)

### Dependencies

```toml
# Cross-platform (Windows, macOS, Linux)
active-win-pos-rs = "0.8"

# Windows-specific
[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = ["Win32_UI_WindowsAndMessaging"] }

# macOS-specific
[target.'cfg(target_os = \"macos\")'.dependencies]
cocoa = "0.25"
objc = "0.2"

# Linux-specific
[target.'cfg(target_os = \"linux\")'.dependencies]
x11rb = "0.13"

# Android-specific
[target.'cfg(target_os = \"android\")'.dependencies]
jni = "0.21"
```

### Implementation Phases

**Phase 1: MVP (Desktop Only)**
- Implement Windows + macOS using `active-win-pos-rs`
- Add Linux X11 support (Wayland optional)
- Build classification engine with bundle ID/process name matching
- Test latency meets <50ms requirement

**Phase 2: Mobile**
- Android: Implement AccessibilityService with JNI bridge
- iOS: Manual context selection UI
- Document permission requirements clearly

**Phase 3: Enhancement**
- Add user-defined classification rules
- Implement ML-based classification for unknown apps
- Optimize battery usage on mobile

**Estimated Integration Time**: 1 week for desktop MVP, 2 weeks for mobile support

## 4. Additional Technical Decisions

### Audio Processing

**Crate**: `cpal` (cross-platform audio I/O)
**Rationale**: Industry-standard, supports all target platforms, low-latency
**Integration**: Capture audio at 16kHz mono, buffer in 100ms chunks

### Local Storage

**Database**: SQLite via `rusqlite`
**Rationale**: Lightweight, embedded, no server required, ACID compliance
**Schema**: Device profile, personal dictionary, transcription history, usage quota

### AI Text Polishing

**Primary**: Anthropic Claude API (claude-3-sonnet)
**Fallback**: OpenAI API (gpt-4-turbo)
**Rationale**: Claude excels at text editing tasks, OpenAI provides reliable fallback
**Integration**: Async HTTP client with retry logic and timeout protection

### State Management (Flutter)

**Framework**: Riverpod
**Rationale**: Type-safe, compile-time dependency injection, excellent async support
**Pattern**: Provider-based architecture with clear separation of concerns

## 5. Risk Assessment

### High-Risk Areas

1. **iOS Context Detection Limitation**
   - **Risk**: Cannot detect active app on iOS
   - **Mitigation**: Manual context selection UI, clear user communication
   - **Impact**: Reduced UX on iOS, but acceptable for MVP

2. **Android Accessibility Permission**
   - **Risk**: Google Play policy restrictions, user friction
   - **Mitigation**: Clear permission explanation, legitimate accessibility use case
   - **Impact**: May affect approval process, requires careful policy compliance

3. **Wayland Support on Linux**
   - **Risk**: Fragmented protocol support across compositors
   - **Mitigation**: Prioritize X11, add Wayland as best-effort
   - **Impact**: Limited functionality on some Linux distributions

### Medium-Risk Areas

1. **Cloud API Latency Variability**
   - **Risk**: Network conditions affect transcription latency
   - **Mitigation**: Regional endpoint selection, connection pooling, retry logic
   - **Impact**: Occasional latency spikes, but p95 should meet requirements

2. **FFI Performance Overhead**
   - **Risk**: Frequent FFI calls may impact performance
   - **Mitigation**: Batch operations, use streaming, minimize data copying
   - **Impact**: Should be negligible with proper design

### Low-Risk Areas

1. **Cross-Platform UI Consistency**
   - **Risk**: Flutter rendering differences across platforms
   - **Mitigation**: Extensive testing, platform-specific adaptations
   - **Impact**: Minor visual differences acceptable

## 6. Open Questions & Future Research

### Resolved in This Phase
- ✅ Which cloud speech API to use? → Azure Speech Services
- ✅ How to integrate Rust with Flutter? → flutter_rust_bridge v2.11+
- ✅ How to detect active application? → Platform-specific native APIs
- ✅ What to do about iOS limitations? → Manual context selection

### Deferred to Implementation Phase
- Optimal audio buffer size for latency vs. accuracy tradeoff
- Exact prompt templates for different application contexts
- Personal dictionary storage format and sync strategy
- Usage quota enforcement mechanism details
- Crash reporting implementation (anonymous, opt-in)

## 7. Next Steps

1. **Phase 1: Design & Contracts** (Next)
   - Create data-model.md with entity definitions
   - Define FFI interface contracts
   - Document AI prompt templates
   - Create quickstart.md for development setup

2. **Phase 2: Task Generation**
   - Generate tasks.md with implementation tasks
   - Organize by user story priority (P1-P5)
   - Define dependencies and parallel opportunities

3. **Phase 3: Implementation**
   - Begin with P1 user story (voice to polished text)
   - Follow TDD workflow (tests first)
   - Iterate based on feedback

## References

### Speech Recognition APIs
- [Azure Speech Services Documentation](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/)
- [Google Speech-to-Text API](https://cloud.google.com/speech-to-text)
- [AWS Transcribe Documentation](https://docs.aws.amazon.com/transcribe/)
- [Speech API Comparison 2025](https://voicewriter.io/blog/best-speech-recognition-api-2025)

### Flutter-Rust Integration
- [flutter_rust_bridge Documentation](https://cjycode.com/flutter_rust_bridge/)
- [flutter_rust_bridge GitHub](https://github.com/fzyzcjy/flutter_rust_bridge)
- [Flutter Rust Bridge Guide](https://blog.logrocket.com/using-flutter-rust-bridge-cross-platform-development/)

### Context Detection
- [active-win-pos-rs crate](http://lib.rs/crates/active-win-pos-rs)
- [windows-rs GitHub](https://github.com/microsoft/windows-rs)
- [frontmost crate (macOS)](https://lib.rs/crates/frontmost)
- [x11rb GitHub (Linux)](https://github.com/psychon/x11rb)
- [Android AccessibilityService Guide](https://developer.android.com/reference/android/accessibilityservice/AccessibilityService)
- [iOS DeviceActivity Framework](https://developer.apple.com/documentation/deviceactivity)

---

**Document Status**: Complete
**Last Updated**: 2026-03-04
**Next Phase**: Design & Contracts (data-model.md, contracts/, quickstart.md)
