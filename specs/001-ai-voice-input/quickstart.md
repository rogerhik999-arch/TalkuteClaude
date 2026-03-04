# Quickstart Guide: Talkute AI Voice Input Assistant

**Feature**: 001-ai-voice-input | **Date**: 2026-03-04 | **Spec**: [spec.md](./spec.md)

## Overview

This guide will help you set up the development environment for Talkute, a cross-platform AI voice input assistant built with Rust (core) and Flutter (UI).

## Prerequisites

### Required Tools

- **Rust**: 1.75+ ([rustup.rs](https://rustup.rs))
- **Flutter**: 3.16+ ([flutter.dev](https://flutter.dev/docs/get-started/install))
- **Dart**: 3.0+ (included with Flutter)
- **Git**: Latest version
- **Platform-specific tools**:
  - **Windows**: Visual Studio 2022 with C++ tools
  - **macOS**: Xcode 15+ with Command Line Tools
  - **Linux**: GCC/Clang, pkg-config, GTK3 development libraries
  - **iOS**: Xcode 15+ (macOS only)
  - **Android**: Android Studio with SDK 33+

### API Keys (Required for MVP)

1. **Azure Speech Services** (or Google/AWS alternative)
   - Sign up at [azure.microsoft.com](https://azure.microsoft.com/en-us/services/cognitive-services/speech-services/)
   - Create a Speech resource
   - Copy API key and region

2. **Anthropic Claude API**
   - Sign up at [console.anthropic.com](https://console.anthropic.com)
   - Generate API key
   - Note: OpenAI API key optional for fallback

## Project Structure

```text
TalkuteClaude/
├── rust-core/              # Rust core logic
│   ├── Cargo.toml
│   ├── src/
│   └── tests/
├── flutter-ui/             # Flutter UI
│   ├── pubspec.yaml
│   ├── lib/
│   └── test/
├── platform/               # Platform-specific code
│   ├── windows/
│   ├── macos/
│   ├── linux/
│   ├── ios/
│   └── android/
└── specs/                  # Feature specifications
    └── 001-ai-voice-input/
```

## Setup Steps

### 1. Clone Repository

```bash
git clone <repository-url>
cd TalkuteClaude
git checkout 001-ai-voice-input
```

### 2. Install Rust Dependencies

```bash
cd rust-core

# Install required Rust targets
rustup target add aarch64-apple-ios      # iOS (macOS only)
rustup target add aarch64-linux-android  # Android
rustup target add x86_64-pc-windows-msvc # Windows
rustup target add x86_64-apple-darwin    # macOS Intel
rustup target add aarch64-apple-darwin   # macOS Apple Silicon
rustup target add x86_64-unknown-linux-gnu # Linux

# Install cargo tools
cargo install flutter_rust_bridge_codegen
cargo install cargo-watch  # Optional: for auto-rebuild

# Build Rust core
cargo build --release
```

### 3. Install Flutter Dependencies

```bash
cd ../flutter-ui

# Get Flutter dependencies
flutter pub get

# Verify Flutter installation
flutter doctor -v

# Enable desktop platforms (if needed)
flutter config --enable-windows-desktop
flutter config --enable-macos-desktop
flutter config --enable-linux-desktop
```

### 4. Generate FFI Bindings

```bash
# From project root
flutter_rust_bridge_codegen \
  --rust-input rust-core/src/ffi/bridge.rs \
  --dart-output flutter-ui/lib/bridge_generated.dart \
  --dart-decl-output flutter-ui/lib/bridge_definitions.dart

# Verify generation
ls flutter-ui/lib/bridge_generated.dart
```

### 5. Configure API Keys

Create `.env` file in project root:

```bash
# Azure Speech Services
AZURE_SPEECH_KEY=your_azure_speech_key_here
AZURE_SPEECH_REGION=eastus

# Anthropic Claude API
ANTHROPIC_API_KEY=your_anthropic_api_key_here

# Optional: OpenAI API (fallback)
OPENAI_API_KEY=your_openai_api_key_here
```

**IMPORTANT**: Add `.env` to `.gitignore` to prevent committing secrets.

### 6. Initialize Database

```bash
cd rust-core

# Run database migrations
cargo run --bin migrate

# Verify database created
ls ~/.talkute/talkute.db  # Linux/macOS
ls %APPDATA%\Talkute\talkute.db  # Windows
```

### 7. Run Tests

```bash
# Rust unit tests
cd rust-core
cargo test

# Rust integration tests
cargo test --test '*'

# Flutter widget tests
cd ../flutter-ui
flutter test

# Flutter integration tests
flutter test integration_test/
```

## Running the Application

### Desktop (Development)

```bash
cd flutter-ui

# Windows
flutter run -d windows

# macOS
flutter run -d macos

# Linux
flutter run -d linux
```

### Mobile (Development)

```bash
cd flutter-ui

# iOS (requires macOS + Xcode)
flutter run -d ios

# Android (requires Android Studio + emulator/device)
flutter run -d android
```

### Hot Reload

While the app is running, press `r` in the terminal to hot reload Flutter UI changes. Rust changes require a full rebuild.

## Development Workflow

### 1. TDD Workflow (Required)

```bash
# Write test first
cd rust-core
cargo test test_name -- --nocapture

# Implement feature
# Edit src/...

# Run test again
cargo test test_name

# Verify coverage (80%+ required)
cargo tarpaulin --out Html
open tarpaulin-report.html
```

### 2. Watch Mode (Optional)

```bash
# Auto-rebuild Rust on file changes
cd rust-core
cargo watch -x build

# Auto-rebuild Flutter on file changes
cd flutter-ui
flutter run --hot
```

### 3. Debugging

#### Rust Debugging

```bash
# Enable debug logging
export RUST_LOG=debug
cargo run

# Use rust-lldb (macOS/Linux) or rust-gdb (Linux)
rust-lldb target/debug/talkute-core
```

#### Flutter Debugging

```bash
# Run with DevTools
flutter run --observatory-port=9200
# Open http://localhost:9200 in browser

# Enable verbose logging
flutter run -v
```

### 4. Code Formatting

```bash
# Rust formatting
cd rust-core
cargo fmt --all

# Rust linting
cargo clippy --all-targets --all-features

# Flutter formatting
cd flutter-ui
dart format lib/ test/

# Flutter linting
flutter analyze
```

## Platform-Specific Setup

### Windows

```bash
# Install Visual Studio 2022 with C++ tools
# Install Windows SDK 10.0.19041.0+

# Build for Windows
cd flutter-ui
flutter build windows --release
```

### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Build for macOS
cd flutter-ui
flutter build macos --release

# Build for iOS
flutter build ios --release
```

### Linux

```bash
# Install GTK3 development libraries
sudo apt-get install libgtk-3-dev libblkid-dev liblzma-dev

# Build for Linux
cd flutter-ui
flutter build linux --release
```

### Android

```bash
# Install Android Studio
# Install Android SDK 33+
# Create emulator or connect device

# Build for Android
cd flutter-ui
flutter build apk --release
flutter build appbundle --release
```

### iOS

```bash
# Requires macOS + Xcode 15+
# Install CocoaPods
sudo gem install cocoapods

# Build for iOS
cd flutter-ui
flutter build ios --release
```

## Troubleshooting

### FFI Binding Generation Fails

```bash
# Ensure flutter_rust_bridge_codegen is installed
cargo install flutter_rust_bridge_codegen --force

# Clean and regenerate
rm flutter-ui/lib/bridge_generated.dart
flutter_rust_bridge_codegen ...
```

### Rust Build Fails

```bash
# Clean build artifacts
cd rust-core
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --release
```

### Flutter Build Fails

```bash
# Clean Flutter build
cd flutter-ui
flutter clean
flutter pub get

# Rebuild
flutter build <platform>
```

### Database Migration Fails

```bash
# Delete existing database
rm ~/.talkute/talkute.db  # Linux/macOS
del %APPDATA%\Talkute\talkute.db  # Windows

# Re-run migrations
cd rust-core
cargo run --bin migrate
```

### API Key Errors

```bash
# Verify .env file exists
cat .env

# Verify API keys are valid
curl -H "x-api-key: $AZURE_SPEECH_KEY" \
  https://$AZURE_SPEECH_REGION.api.cognitive.microsoft.com/sts/v1.0/issuetoken

curl -H "x-api-key: $ANTHROPIC_API_KEY" \
  https://api.anthropic.com/v1/messages
```

## Performance Profiling

### Rust Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cd rust-core
cargo flamegraph --bin talkute-core

# Open flamegraph.svg in browser
```

### Flutter Profiling

```bash
# Run with profiling
cd flutter-ui
flutter run --profile

# Open DevTools
flutter pub global activate devtools
flutter pub global run devtools
```

## CI/CD Setup

### GitHub Actions (Example)

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: subosito/flutter-action@v2
        with:
          flutter-version: '3.16.0'
      - name: Run Rust tests
        run: cd rust-core && cargo test
      - name: Run Flutter tests
        run: cd flutter-ui && flutter test
```

## Next Steps

1. **Read the Specification**: [spec.md](./spec.md)
2. **Review the Implementation Plan**: [plan.md](./plan.md)
3. **Check the Data Model**: [data-model.md](./data-model.md)
4. **Review FFI Contracts**: [contracts/ffi-interface.md](./contracts/ffi-interface.md)
5. **Review AI Prompts**: [contracts/ai-prompts.md](./contracts/ai-prompts.md)
6. **Start Implementing**: Follow TDD workflow with 80%+ coverage

## Resources

- **Rust Documentation**: [doc.rust-lang.org](https://doc.rust-lang.org)
- **Flutter Documentation**: [flutter.dev/docs](https://flutter.dev/docs)
- **flutter_rust_bridge**: [github.com/fzyzcjy/flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge)
- **Azure Speech SDK**: [docs.microsoft.com/azure/cognitive-services/speech-service](https://docs.microsoft.com/azure/cognitive-services/speech-service)
- **Anthropic API**: [docs.anthropic.com](https://docs.anthropic.com)
- **Project Constitution**: [.specify/memory/constitution.md](../../.specify/memory/constitution.md)

## Support

For questions or issues:
1. Check existing GitHub issues
2. Review troubleshooting section above
3. Create new issue with detailed description and logs
