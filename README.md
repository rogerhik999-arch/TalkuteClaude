# Talkute - AI Voice Input Assistant

An AI-powered voice input assistant that transforms natural speech into polished, professionally written text 4x faster than keyboard typing.

## Features

- **Real-time Voice-to-Text**: Speak naturally and receive polished text instantly
- **AI Text Polishing**: Automatic filler word removal and self-correction detection
- **Context-Aware Tone**: Automatically adapts tone based on active application
- **Personal Dictionary**: 100% accurate recognition of custom terms
- **Multi-language Support**: English, Chinese, Japanese, Spanish, French, German
- **Real-time Translation**: Optional translation to target languages
- **Cross-Platform**: Windows, macOS, Linux, iOS, Android

## Quick Start

### Prerequisites

- Rust 1.75+
- Flutter 3.16+
- Azure Speech Services API key
- Anthropic Claude API key

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/rogerhik999-arch/TalkuteClaude.git
   cd TalkuteClaude
   ```

2. **Configure API keys**
   ```bash
   cp .env.template .env
   # Edit .env with your API keys
   ```

3. **Build Rust core**
   ```bash
   cd rust-core
   cargo build --release
   ```

4. **Build Flutter UI**
   ```bash
   cd flutter-ui
   flutter pub get
   flutter run
   ```

### Platform Setup

See [Platform Setup Guide](docs/PLATFORM_SETUP.md) for detailed platform-specific setup instructions.

### Mobile Keyboard

See [Mobile Keyboard Installation Guide](docs/MOBILE_KEYBOARD_INSTALLATION.md) for iOS and Android keyboard setup.

## Project Structure

```
TalkuteClaude/
├── rust-core/           # Core logic, AI integration, FFI bridge
│   ├── src/
│   │   ├── ai/          # AI polishing, prompts, translation
│   │   ├── context/     # Application context detection
│   │   ├── ffi/         # Flutter-Rust FFI bridge
│   │   ├── platform/    # Cross-platform abstractions
│   │   ├── processing/  # Text processing pipeline
│   │   ├── speech/      # Speech recognition
│   │   ├── state/       # Session state management
│   │   └── storage/     # Database, preferences, history
│   └── tests/           # Rust unit and integration tests
├── flutter-ui/          # Cross-platform UI
│   ├── lib/
│   │   ├── models/      # Data models
│   │   ├── screens/     # UI screens
│   │   ├── services/    # Business logic services
│   │   ├── state/       # Riverpod state management
│   │   └── widgets/     # Reusable UI components
│   └── test/            # Flutter widget tests
├── platform/            # Platform-specific code
│   ├── ios/             # iOS keyboard extension
│   └── android/         # Android IME
├── docs/                # Documentation
└── specs/               # Feature specifications
```

## Technology Stack

| Component | Technology |
|-----------|------------|
| Core Logic | Rust 1.75+ |
| UI Framework | Flutter 3.16+ |
| FFI Bridge | flutter_rust_bridge 2.11+ |
| Speech API | Azure Speech Services |
| AI API | Anthropic Claude |
| Database | SQLite (SQLCipher) |
| Async Runtime | Tokio |
| State Management | Riverpod |

## Development

### Run Tests

```bash
# Rust tests
cd rust-core
cargo test

# Flutter tests
cd flutter-ui
flutter test

# Integration tests
cd flutter-ui
flutter test integration_test/
```

### Code Quality

```bash
# Rust linting
cd rust-core
cargo clippy
cargo fmt --check

# Flutter analysis
cd flutter-ui
flutter analyze
```

### Generate FFI Bindings

```bash
flutter_rust_bridge_codegen generate \
  --rust-input rust-core/src/ffi/bridge.rs \
  --dart-output flutter-ui/lib/bridge_generated.dart
```

## Performance

Performance targets (validated by benchmarks):

| Metric | Target | P95 Result |
|--------|--------|------------|
| Context detection | <50ms | ~5ms |
| UI response time | <200ms | ~50ms |
| Speech-to-text | <2s | ~1.5s |
| Memory (idle) | <100MB | ~50MB |
| Memory (active) | <300MB | ~150MB |

See [Validation Report](docs/validation-report.md) for detailed benchmark results.

## Security

- **Data Encryption**: AES-256-CBC at rest via SQLCipher
- **Key Storage**: Platform keychain integration
- **Zero Retention**: Voice data not stored on servers
- **Privacy First**: All data stored locally, no accounts required

## License

MIT License - See LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Support

- **FAQ**: talkute.app/faq
- **Email**: support@talkute.app
- **Issues**: [GitHub Issues](https://github.com/rogerhik999-arch/TalkuteClaude/issues)

---

Built with ❤️ using Rust and Flutter