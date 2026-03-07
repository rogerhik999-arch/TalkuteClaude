# Implementation Plan: Product Experience Design Implementation

**Branch**: `002-product-experience-ux` | **Date**: 2026-03-07 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-product-experience-ux/spec.md`

## Summary

Implement the complete product experience design for Talkute, including desktop floating capsule with voice input, system tray management, comprehensive settings interface, custom dictionary, history recording, multi-language support with translation, and mobile keyboard input method. The technical approach uses Flutter for cross-platform UI with Rust core for system integration (hotkeys, tray, text injection, audio capture) via flutter_rust_bridge FFI.

## Technical Context

**Language/Version**: Rust 1.75+ (core), Flutter 3.16+ (UI), Dart 3.0+ (Flutter)
**Primary Dependencies**:
- Rust: tokio (async), serde (serialization), chrono (timestamps), once_cell (lazy static), tracing (logging)
- Flutter: flutter_rust_bridge (FFI), riverpod (state management), flutter_local_notifications (tray alerts)
- External APIs: Azure Speech Services (ASR), Anthropic Claude API (polishing/translation)

**Storage**: SQLite (history, dictionary, preferences) via Rust core
**Testing**: cargo test (Rust), flutter test (Flutter), integration_test (E2E)
**Target Platform**: Cross-platform (Windows, macOS, Linux for desktop; iOS, Android for mobile)
**Project Type**: Desktop + Mobile application with AI integration
**Performance Goals**:
- Context detection: <50ms (p95)
- AI response: <3s end-to-end (speech to text insertion)
- UI rendering: 60fps minimum
- Memory: <100MB idle, <300MB active

**Constraints**:
- CPU: <5% idle, <30% during AI processing
- Network: Required for cloud ASR and AI
- Install size: <50MB (excluding models)

**Scale/Scope**: Single-user application, 1000+ transcriptions/day, 30-day history retention

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Verify compliance with `.specify/memory/constitution.md`:

- [x] **Rust-First Core**: Core logic (hotkey, tray, text injection, audio capture, AI integration) in Rust
- [x] **Flutter Cross-Platform UI**: Single Flutter codebase for all platforms (desktop + mobile)
- [x] **AI-Native Architecture**: LLM calls async with timeout, fallback strategies defined (clipboard)
- [x] **Context-Aware Intelligence**: Application context detection for polishing style adaptation
- [x] **Minimal & Elegant Design**: 5 core modules (UI, Core Logic, AI, Context, Platform Adapters)
- [x] **Test-First Development**: TDD workflow planned, 80%+ coverage target

**Complexity Violations**: None - Design adheres to all constitutional principles.

## Project Structure

### Documentation (this feature)

```text
specs/002-product-experience-ux/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 research findings
├── data-model.md        # Phase 1 data models
├── quickstart.md        # Phase 1 integration guide
├── contracts/           # Phase 1 interface contracts
│   └── ffi-contracts.md # FFI boundary contracts
└── tasks.md             # Phase 2 task breakdown (via /speckit.tasks)
```

### Source Code (repository root)

```text
rust-core/
├── src/
│   ├── context/              # Context detection (OS-specific)
│   │   ├── detector.rs       # Active window detection
│   │   └── registry.rs       # Application classification
│   ├── ai/                   # AI integration
│   │   ├── client.rs         # Claude API client
│   │   ├── prompts.rs        # Prompt templates
│   │   ├── polisher.rs       # Text polishing logic
│   │   └── translator.rs     # Translation service
│   ├── processing/           # Input processing
│   │   ├── mod.rs            # Processing pipeline
│   │   ├── filler_removal.rs # Filler word removal
│   │   └── formatter.rs      # Text formatting
│   ├── speech/               # Speech services
│   │   ├── client.rs         # Azure Speech client
│   │   └── audio_capture.rs  # Audio capture with noise cancellation
│   ├── storage/              # Data persistence
│   │   ├── database.rs       # SQLite database
│   │   ├── dictionary.rs     # Custom dictionary
│   │   ├── history.rs        # Transcription history
│   │   ├── export.rs         # Data export
│   │   └── cleanup.rs        # Data cleanup
│   ├── network/              # Network utilities
│   │   └── offline_handler.rs# Offline detection
│   ├── tools/                # Utility tools
│   │   └── profiler.rs       # Performance profiling
│   ├── platform/             # Platform adapters
│   │   ├── tray.rs           # System tray (platform-specific)
│   │   ├── hotkey.rs         # Global hotkeys (platform-specific)
│   │   ├── text_injection.rs # Text injection (platform-specific)
│   │   └── window.rs         # Floating window (platform-specific)
│   ├── ffi/                  # Flutter FFI bindings
│   │   └── bridge.rs         # flutter_rust_bridge definitions
│   └── lib.rs                # Library entry point
└── tests/
    ├── unit/                 # Unit tests
    ├── integration/          # Integration tests
    ├── network/              # Network tests
    └── benchmarks/           # Performance benchmarks

flutter-ui/
├── lib/
│   ├── main.dart             # App entry point
│   ├── app.dart              # App configuration
│   ├── screens/              # UI screens
│   │   ├── home_screen.dart  # Main screen (desktop mode)
│   │   ├── settings_screen.dart # Settings interface
│   │   ├── history_screen.dart # History view
│   │   └── dictionary_screen.dart # Dictionary management
│   ├── widgets/              # Reusable components
│   │   ├── floating_capsule.dart # Desktop floating capsule
│   │   ├── voice_button.dart # Microphone button
│   │   ├── audio_visualizer.dart # Waveform display
│   │   ├── quota_indicator.dart # Usage quota display
│   │   └── language_selector.dart # Language picker
│   ├── services/             # Rust FFI wrappers
│   │   ├── engine_service.dart # Core engine bridge
│   │   ├── config_service.dart # Settings persistence
│   │   ├── shortcuts.dart    # Keyboard shortcuts
│   │   └── tray_service.dart # System tray management
│   ├── state/                # State management
│   │   ├── voice_state.dart  # Recording state
│   │   ├── app_state.dart    # Application state
│   │   └── providers.dart    # Riverpod providers
│   └── models/               # Data models
│       ├── language.dart     # Language definitions
│       ├── context.dart      # Application context
│       └── preferences.dart  # User preferences
└── test/
    ├── widgets/              # Widget tests
    ├── screens/              # Screen tests
    └── integration/          # Integration tests

platform/
├── windows/                  # Windows-specific config
├── macos/                    # macOS-specific config
├── linux/                    # Linux-specific config
├── ios/                      # iOS keyboard extension
└── android/                  # Android IME
```

**Structure Decision**: Using existing Rust + Flutter cross-platform structure. Desktop-specific features (tray, hotkey, floating window, text injection) go in `rust-core/src/platform/`. Mobile keyboard implementation goes in `platform/ios/` and `platform/android/` as platform-specific extensions.

## Complexity Tracking

> No constitutional violations - design adheres to all principles.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (none) | - | - |

## Implementation Phases

### Phase 1: Desktop Core Experience (P1 Stories)
- Floating capsule with four states (recording, processing, done, error)
- System tray with status indicators and quick menu
- Global hotkey with push-to-talk and toggle modes
- Real-time audio waveform visualization
- Text injection at cursor position

### Phase 2: Settings & Configuration (P2 Stories)
- Settings interface with grouped configuration
- Custom dictionary management
- History recording and review
- Language detection and translation
- AI polishing intensity selection

### Phase 3: Mobile Input Method (P3 Stories)
- iOS Custom Keyboard Extension
- Android InputMethodService
- Mobile keyboard UI with microphone button

### Phase 4: Polish & Integration
- Error handling and edge cases
- Performance optimization
- Accessibility (platform-level)
- E2E testing

## Dependencies

### External APIs
- **Azure Speech Services**: ASR transcription (required)
- **Anthropic Claude API**: Text polishing and translation (required)

### Rust Crates
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `chrono` - Timestamps
- `sqlx` or `rusqlite` - SQLite database
- `once_cell` - Lazy statics
- `tracing` - Logging
- `thiserror` - Error handling
- `reqwest` - HTTP client
- `uuid` - Unique IDs

### Flutter Packages
- `flutter_rust_bridge` - FFI integration
- `flutter_riverpod` - State management
- `shared_preferences` - Simple preferences
- `flutter_local_notifications` - System notifications
- `audio_session` - Audio management
- `permission_handler` - Permissions

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Platform-specific text injection varies | High | Abstract behind platform trait, test on all platforms |
| Global hotkey conflicts with other apps | Medium | Allow hotkey customization, detect conflicts |
| Mobile keyboard has platform limitations | Medium | Document limitations, provide desktop fallback |
| AI API latency affects UX | High | Show processing state, optimize prompts |
| Audio capture permissions denied | High | Graceful error handling with clear instructions |
