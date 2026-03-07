# TalkuteClaude Development Guidelines

Auto-generated from all feature plans. **Last updated**: 2026-03-07

## Active Technologies
- Rust 1.75+ (core), Flutter 3.16+ (UI), Dart 3.0+ (Flutter) (002-product-experience-ux)
- SQLite (history, dictionary, preferences) via Rust core (002-product-experience-ux)
- Platform abstraction: tray-item, global-hotkey, enigo (002-product-experience-ux)

- Rust 1.75+ (core) + Flutter 3.16+ (UI) (001-ai-voice-input)
- flutter_rust_bridge v2.11+ (FFI) (001-ai-voice-input)
- Azure Speech Services API (001-ai-voice-input)
- Anthropic Claude API (001-ai-voice-input)
- Local SQLite database (001-ai-voice-input)

## Project Structure

```text
rust-core/
├── src/
│   ├── ai/           # AI integration (polisher, prompts, client)
│   ├── context/      # Application context detection
│   ├── ffi/          # Flutter bridge definitions
│   ├── network/      # Offline handling, connectivity
│   ├── platform/     # Cross-platform abstractions
│   │   ├── windows/  # Windows-specific implementations
│   │   ├── macos/    # macOS-specific implementations
│   │   └── linux/    # Linux-specific implementations
│   ├── processing/   # Text processing pipeline
│   ├── speech/       # Speech recognition
│   ├── state/        # Session state management
│   ├── storage/      # Database, preferences, history
│   └── tools/        # Profiler, utilities
flutter-ui/
├── lib/
│   ├── models/       # Data models
│   ├── screens/      # UI screens (settings, history, dictionary)
│   ├── services/     # Business logic services
│   ├── state/        # Riverpod state management
│   └── widgets/      # Reusable UI components
platform/
├── ios/              # iOS keyboard extension (future)
└── android/          # Android IME (future)
tests/
specs/
```

## Commands

```bash
# Rust core
cd rust-core
cargo test
cargo clippy

# Flutter UI
cd flutter-ui
flutter test
flutter analyze

# FFI binding generation
flutter_rust_bridge_codegen \
  --rust-input rust-core/src/ffi/bridge.rs \
  --dart-output flutter-ui/lib/bridge_generated.dart
```

## Code Style

- Rust: Follow standard Rust conventions (rustfmt, clippy)
- Dart/Flutter: Follow Dart style guide
- Immutability preferred in Rust core
- Async-first architecture with tokio (Rust) and Future (Dart)

## Platform Modules

### System Tray (PlatformTray)
- Windows: Uses tray-item with winapi
- macOS: Uses tray-item with cocoa
- Linux: Uses tray-item with gtk

### Global Hotkey (PlatformHotkey)
- Windows: Uses global-hotkey with winapi
- macOS: Uses global-hotkey with cocoa
- Linux: Uses global-hotkey with x11

### Text Injection (TextInjector)
- Windows: Uses enigo with winapi keyboard simulation
- macOS: Uses enigo with core-graphics
- Linux: Uses enigo with x11

### Window Management (WindowManager)
- Windows: Uses winapi for floating window
- macOS: Uses cocoa for floating window
- Linux: Uses gtk for floating window

## Recent Changes
- 002-product-experience-ux: Added platform modules, session state, polishing intensity, quota management

- 001-ai-voice-input: Added Rust 1.75+ (core) + Flutter 3.16+ (UI) + Azure Speech + Claude API

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
