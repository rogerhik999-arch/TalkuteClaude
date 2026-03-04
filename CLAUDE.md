# TalkuteClaude Development Guidelines

Auto-generated from all feature plans. **Last updated**: 2026-03-04

## Active Technologies

- Rust 1.75+ (core) + Flutter 3.16+ (UI) (001-ai-voice-input)
- flutter_rust_bridge v2.11+ (FFI) (001-ai-voice-input)
- Azure Speech Services API (001-ai-voice-input)
- Anthropic Claude API (001-ai-voice-input)
- Local SQLite database (001-ai-voice-input)

## Project Structure

```text
rust-core/
flutter-ui/
platform/
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

## Recent Changes

- 001-ai-voice-input: Added Rust 1.75+ (core) + Flutter 3.16+ (UI) + Azure Speech + Claude API

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
