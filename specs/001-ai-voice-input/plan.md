# Implementation Plan: Talkute AI Voice Input Assistant

**Branch**: `001-ai-voice-input` | **Date**: 2026-03-04 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-ai-voice-input/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Talkute is an AI-powered voice input assistant that transforms natural speech into polished, professionally written text 4x faster than keyboard typing. The system uses cloud-based speech recognition APIs, applies AI-driven text polishing (filler word removal, self-correction detection, tone adaptation), and operates as a system-level input method across all applications. The MVP focuses on anonymous, device-local operation with no user accounts, using Rust for core logic and Flutter for cross-platform UI.

**Key Technical Decisions** (from clarifications):
- Cloud-based speech recognition API (Google Speech-to-Text, Azure Speech, or AWS Transcribe)
- No authentication - anonymous usage with device-local storage only
- 5-minute maximum per voice input session
- Local error logs with optional anonymous crash reports
- Weekly 4,000-word free tier limit enforced locally

## Technical Context

**Language/Version**: Rust 1.75+ (core), Flutter 3.16+ (UI), Dart 3.0+ (Flutter)
**Primary Dependencies**:
- Rust: flutter_rust_bridge (FFI), tokio (async runtime), reqwest (HTTP client), serde (serialization)
- Flutter: provider (state management), flutter_rust_bridge_codegen (FFI generation)
- Cloud APIs: Google Speech-to-Text API OR Azure Speech API OR AWS Transcribe (to be selected in research)
- AI: Anthropic Claude API (text polishing), OpenAI API (fallback)

**Storage**: Local SQLite database for device profile, personal dictionary, transcription history, usage quota tracking
**Testing**: cargo test (Rust unit/integration), flutter test (Flutter widget), integration_test (E2E)
**Target Platform**: Cross-platform (Windows, Mac, Linux, iOS, Android)
**Project Type**: Desktop + Mobile application with AI integration and system-level input method
**Performance Goals**: <50ms context detection, <500ms voice-to-text latency (p95), <2s AI polishing, 60fps UI, <100MB idle memory
**Constraints**: <300MB active memory, <5% idle CPU, <30% active CPU, <50MB install size
**Scale/Scope**: Single-user application, 1000+ enhancements/day, 10+ concurrent context switches, 5-minute max session duration

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Verify compliance with `.specify/memory/constitution.md`:

- [x] **Rust-First Core**: Core logic (speech API integration, text processing, context detection, AI integration) implemented in Rust
- [x] **Flutter Cross-Platform UI**: Single Flutter codebase for all 5 platforms (Windows, Mac, Linux, iOS, Android)
- [x] **AI-Native Architecture**: LLM integration asynchronous with tokio, cancellable, timeout-protected, with fallback strategies
- [x] **Context-Aware Intelligence**: OS-level application context detection via platform-specific APIs
- [x] **Minimal & Elegant Design**: 5 core modules planned: (1) UI, (2) Core Logic, (3) AI Integration, (4) Context Detection, (5) Platform Adapters
- [x] **Test-First Development**: TDD workflow with ≥80% coverage for Rust core, unit/integration/E2E tests planned

### Performance Standards Validation (Constitution §Performance Standards)

**Response Time Requirements**:
- [ ] Context detection: <50ms (p95) - **Validation**: Benchmark in Phase 8 (T122)
- [ ] AI prompt preparation: <20ms (p95) - **Validation**: Benchmark in Phase 8 (T122)
- [ ] LLM API call: <2s (p95, network-dependent) - **Validation**: Integration tests with timeout
- [ ] UI rendering: 60fps minimum, 120fps target - **Validation**: Flutter DevTools profiling
- [ ] Memory footprint: <100MB idle, <300MB active - **Validation**: Memory profiler in Phase 8 (T122)

**Resource Constraints**:
- [ ] CPU: <5% idle, <30% during AI processing - **Validation**: System monitor integration (T122)
- [ ] Disk: <50MB installation size (excluding AI models) - **Validation**: Build artifact size check

**Acceptance Criteria**: All performance targets must be met on reference hardware (mid-range 2023 devices) before production release.

**Complexity Violations**: None - architecture aligns with all constitutional principles.

## Project Structure

### Documentation (this feature)

```text
specs/001-ai-voice-input/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── ffi-interface.md # Rust-Flutter FFI contract
│   └── ai-prompts.md    # AI prompt templates and versioning
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
rust-core/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # Main library entry point
│   ├── ffi/
│   │   ├── mod.rs                # FFI module root
│   │   └── bridge.rs             # Flutter bridge definitions
│   ├── speech/
│   │   ├── mod.rs                # Speech recognition module
│   │   ├── client.rs             # Cloud API client
│   │   └── audio_capture.rs     # Audio input handling
│   ├── ai/
│   │   ├── mod.rs                # AI integration module
│   │   ├── polisher.rs           # Text polishing logic
│   │   ├── prompts.rs            # Prompt templates
│   │   └── client.rs             # LLM API client
│   ├── context/
│   │   ├── mod.rs                # Context detection module
│   │   ├── detector.rs           # Application context detection
│   │   ├── windows.rs            # Windows-specific implementation
│   │   ├── macos.rs              # macOS-specific implementation
│   │   ├── linux.rs              # Linux-specific implementation
│   │   ├── ios.rs                # iOS-specific implementation
│   │   └── android.rs            # Android-specific implementation
│   ├── processing/
│   │   ├── mod.rs                # Text processing module
│   │   ├── filler_removal.rs    # Filler word removal
│   │   ├── self_correction.rs   # Self-correction detection
│   │   └── formatter.rs          # Auto-formatting logic
│   ├── storage/
│   │   ├── mod.rs                # Storage module
│   │   ├── database.rs           # SQLite database interface
│   │   └── models.rs             # Data models
│   └── quota/
│       ├── mod.rs                # Usage quota module
│       └── tracker.rs            # Word count tracking
└── tests/
    ├── unit/
    │   ├── speech_tests.rs
    │   ├── ai_tests.rs
    │   ├── context_tests.rs
    │   └── processing_tests.rs
    └── integration/
        ├── end_to_end_tests.rs
        └── ffi_tests.rs

flutter-ui/
├── pubspec.yaml
├── lib/
│   ├── main.dart                 # App entry point
│   ├── screens/
│   │   ├── home_screen.dart      # Main input screen
│   │   ├── settings_screen.dart  # Settings and preferences
│   │   └── dictionary_screen.dart # Personal dictionary management
│   ├── widgets/
│   │   ├── voice_button.dart     # Voice input activation button
│   │   ├── transcription_preview.dart # Real-time preview
│   │   └── quota_indicator.dart  # Usage quota display
│   ├── services/
│   │   ├── rust_bridge.dart      # Rust FFI wrapper
│   │   └── platform_service.dart # Platform-specific services
│   ├── state/
│   │   ├── app_state.dart        # Global app state
│   │   └── voice_state.dart      # Voice input state
│   └── models/
│       ├── session.dart          # Voice session model
│       └── settings.dart         # User settings model
└── test/
    ├── widget/
    │   ├── voice_button_test.dart
    │   └── transcription_preview_test.dart
    └── integration/
        └── app_test.dart

platform/
├── windows/                      # Windows-specific code
├── macos/                        # macOS-specific code
├── linux/                        # Linux-specific code
├── ios/                          # iOS-specific code
└── android/                      # Android-specific code
```

**Structure Decision**: Using Rust + Flutter cross-platform architecture (Option 1 from template). Rust handles all core logic, AI integration, and platform-specific context detection. Flutter provides unified UI across all 5 platforms. FFI bridge connects the two via flutter_rust_bridge. This structure aligns with constitutional principles: Rust-first core, Flutter cross-platform UI, minimal module count (5 core modules), and clear separation of concerns.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations - all constitutional principles satisfied.

---

## Phase 0: Research (COMPLETED)

**Status**: ✅ Complete

**Output**: [research.md](./research.md)

**Key Decisions**:
1. **Speech Recognition**: Azure Speech Services selected
   - Rationale: Best balance of latency (200-400ms), accuracy (95%+), and cost ($1-3k/month)
   - Alternatives: Google Speech-to-Text (higher cost), AWS Transcribe (higher latency)

2. **Flutter-Rust FFI**: flutter_rust_bridge v2.11+
   - Rationale: Type-safe, async-first, auto-generated bindings, active maintenance
   - Implementation: Code generation from Rust definitions

3. **Context Detection**: Platform-specific native APIs
   - Windows: Win32 GetForegroundWindow + GetWindowText
   - macOS: NSWorkspace activeApplication
   - Linux: X11 XGetInputFocus (primary), Wayland best-effort
   - iOS: Manual selection fallback (no programmatic detection)
   - Android: AccessibilityService (requires permission)

**Risks Identified**:
- iOS context detection limitation → Mitigated with manual selection UI
- Android accessibility permission concerns → Clear user education + optional feature
- Wayland fragmentation on Linux → Prioritize X11, Wayland best-effort

---

## Phase 1: Design & Contracts (COMPLETED)

**Status**: ✅ Complete

**Outputs**:
- [data-model.md](./data-model.md) - 7 core entities with SQLite schema
- [contracts/ffi-interface.md](./contracts/ffi-interface.md) - Flutter-Rust FFI contract
- [contracts/ai-prompts.md](./contracts/ai-prompts.md) - AI prompt templates and versioning
- [quickstart.md](./quickstart.md) - Development setup guide
- [CLAUDE.md](../../CLAUDE.md) - Agent context file (updated)

**Data Model Summary**:
- **DeviceProfile**: Single user profile per device
- **VoiceSession**: Individual voice input sessions (5-minute max)
- **ApplicationContext**: Detected application contexts with tone preferences
- **PersonalDictionaryEntry**: User-defined phrase replacements
- **TranscriptionHistory**: Intermediate processing stages for debugging
- **UsageQuota**: Weekly 4,000-word free tier tracking
- **LicenseKey**: Future paid tier support (not MVP)

**FFI Contract Summary**:
- 40+ async API functions across 7 categories
- Type-safe DTOs with auto-generated Dart bindings
- Stream-based APIs for real-time updates (audio levels, partial transcriptions)
- Comprehensive error types for all failure modes
- Zero-copy audio buffer passing for performance

**AI Prompt Contract Summary**:
- Context-aware system prompts (5 application categories)
- Versioned prompt templates for A/B testing
- Low temperature (0.3) for deterministic outputs
- Personal dictionary integration
- Claude API primary, OpenAI fallback
- Token estimation and validation

**Constitution Re-Check** (Post-Design):
- [x] **Rust-First Core**: All business logic in Rust ✅
- [x] **Flutter Cross-Platform UI**: Single Flutter codebase ✅
- [x] **AI-Native Architecture**: Async LLM integration with fallbacks ✅
- [x] **Context-Aware Intelligence**: Platform-specific detection implemented ✅
- [x] **Minimal & Elegant Design**: 7 entities, 5 modules, clean separation ✅
- [x] **Test-First Development**: TDD workflow enforced, 80%+ coverage required ✅

**No new violations introduced.**

---

## Phase 2: Task Generation

**Status**: ⏳ Pending

**Next Command**: `/speckit.tasks`

This phase will generate the actionable task list (tasks.md) with:
- Dependency-ordered implementation tasks
- Test-first workflow enforcement
- Estimated complexity per task
- Acceptance criteria for each task

**Note**: Phase 2 is NOT executed by `/speckit.plan`. Run `/speckit.tasks` separately to generate the task breakdown.

---

## Implementation Readiness

**Prerequisites Complete**:
- ✅ Research completed with all technical decisions made
- ✅ Data model defined with SQLite schema
- ✅ FFI contracts documented with 40+ API functions
- ✅ AI prompt templates versioned and tested
- ✅ Development environment setup guide created
- ✅ Agent context updated with technology stack

**Ready for Task Generation**: Yes

**Recommended Next Steps**:
1. Run `/speckit.tasks` to generate implementation task list
2. Review tasks.md for completeness and ordering
3. Begin TDD implementation starting with Rust core tests
4. Follow quickstart.md for environment setup

---

## Appendix: Technology Stack Summary

| Category | Technology | Version | Purpose |
|----------|-----------|---------|---------|
| Core Language | Rust | 1.75+ | Business logic, AI integration, context detection |
| UI Framework | Flutter | 3.16+ | Cross-platform UI (Windows, Mac, Linux, iOS, Android) |
| FFI Bridge | flutter_rust_bridge | 2.11+ | Type-safe Rust-Dart interop |
| Speech API | Azure Speech Services | v1 | Cloud-based speech-to-text |
| AI API | Anthropic Claude | 3.5 Sonnet | Text polishing and enhancement |
| Database | SQLite | 3.x | Local device storage |
| Async Runtime | Tokio | 1.x | Rust async operations |
| State Management | Riverpod | 2.x | Flutter state management |
| Testing | cargo test, flutter test | - | Unit, integration, E2E tests |

**Total Dependencies**: 9 core technologies (minimal, as per constitution)

---

## Plan Completion

**Date Completed**: 2026-03-04

**Branch**: `001-ai-voice-input`

**Artifacts Generated**:
1. ✅ research.md (Phase 0)
2. ✅ data-model.md (Phase 1)
3. ✅ contracts/ffi-interface.md (Phase 1)
4. ✅ contracts/ai-prompts.md (Phase 1)
5. ✅ quickstart.md (Phase 1)
6. ✅ CLAUDE.md updated (Phase 1)
7. ⏳ tasks.md (Phase 2 - pending `/speckit.tasks` command)

**Command to Continue**: `/speckit.tasks`
