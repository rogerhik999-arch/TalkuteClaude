# Tasks: Talkute AI Voice Input Assistant

**Input**: Design documents from `/specs/001-ai-voice-input/`
**Prerequisites**: plan.md, spec.md, data-model.md, contracts/, research.md, quickstart.md

**TDD Workflow**: All tasks follow Red-Green-Refactor cycle per constitution principle VI (Test-First Development)
- **[RED]**: Write test first (expect failure)
- **[GREEN]**: Implement minimal code to pass test
- **[IMPROVE]**: Refactor for quality without breaking tests

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [RED/GREEN/IMPROVE] [P?] [Story] Description`

- **[RED/GREEN/IMPROVE]**: TDD phase marker (required for all implementation tasks)
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Rust core**: `rust-core/src/`
- **Flutter UI**: `flutter-ui/lib/`
- **Platform-specific**: `platform/[windows|macos|linux|ios|android]/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create project directory structure per plan.md (rust-core/, flutter-ui/, platform/, specs/)
- [X] T002 Initialize Rust workspace with Cargo.toml in rust-core/
- [X] T003 [P] Initialize Flutter project with pubspec.yaml in flutter-ui/
- [X] T004 [P] Configure rustfmt and clippy in rust-core/.rustfmt.toml and rust-core/.clippy.toml
- [X] T005 [P] Configure Dart formatting in flutter-ui/analysis_options.yaml
- [X] T006 Create .env.template file with required API keys (AZURE_SPEECH_KEY, ANTHROPIC_API_KEY)
- [X] T007 [P] Add .env to .gitignore
- [X] T008 Install flutter_rust_bridge_codegen tool via cargo install

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T009 Create SQLite database schema with migrations in rust-core/migrations/001_initial_schema.sql
- [X] T010 [P] Implement database connection module in rust-core/src/storage/database.rs
- [X] T011 [P] Create DeviceProfile model in rust-core/src/storage/models.rs
- [X] T012 [P] Create UsageQuota model in rust-core/src/storage/models.rs
- [X] T013 Implement database migration runner in rust-core/src/storage/migrations.rs
- [X] T014 [P] Setup tokio async runtime configuration in rust-core/src/lib.rs
- [X] T015 [P] Create error types module in rust-core/src/error.rs
- [X] T016 [P] Implement logging infrastructure with env_logger in rust-core/src/lib.rs
- [X] T017 Create FFI bridge skeleton in rust-core/src/ffi/bridge.rs
- [ ] T018 Generate initial Flutter bindings with flutter_rust_bridge_codegen
- [X] T019 [P] Create Flutter app state management structure with Riverpod in flutter-ui/lib/state/app_state.dart
- [X] T020 [P] Implement platform-specific initialization for Windows in platform/windows/
- [X] T021 [P] Implement platform-specific initialization for macOS in platform/macos/
- [X] T022 [P] Implement platform-specific initialization for Linux in platform/linux/
- [X] T023 [P] Implement platform-specific initialization for iOS in platform/ios/
- [X] T024 [P] Implement platform-specific initialization for Android in platform/android/
- [X] T025 Create device profile initialization logic in rust-core/src/storage/profile.rs
- [X] T026 Implement usage quota tracking module in rust-core/src/quota/tracker.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

Note: Phase 1 and Phase 2 are mostly complete. The project structure is in place with:
- Directory structure created
- Rust workspace and Flutter project initialized
- Code formatting tools configured
- Database schema and models implemented
- Platform adapters created for all 5 platforms
- Device profile and quota services implemented
- Speech recognition service implemented
- Text processing pipeline implemented
- AI polishing service implemented

Remaining Phase 2 tasks:
- T018: Generate initial Flutter bindings with flutter_rust_bridge_codegen (blocked by network issues)

---

## Phase 3: User Story 1 - Real-time Voice to Polished Text (Priority: P1) 🎯 MVP

**Goal**: Enable users to speak naturally and receive clean, polished text output with filler words removed and self-corrections applied

**Independent Test**: User can activate voice input, speak naturally (including filler words and self-corrections), and receive clean, polished text output within 500ms

### Implementation for User Story 1

#### Data Models (Test-First)
- [X] T027 [RED] [P] [US1] Write tests for VoiceSession model in rust-core/tests/storage/session_test.rs
- [X] T028 [RED] [P] [US1] Write tests for TranscriptionHistory model in rust-core/tests/storage/history_test.rs
- [X] T029 [GREEN] [P] [US1] Create VoiceSession model in rust-core/src/storage/models.rs (make T027 pass)
- [X] T030 [GREEN] [P] [US1] Create TranscriptionHistory model in rust-core/src/storage/models.rs (make T028 pass)

#### Speech Recognition (Test-First)
- [X] T031 [RED] [P] [US1] Write tests for Azure Speech API client in rust-core/tests/speech/client_test.rs
- [X] T032 [RED] [P] [US1] Write tests for audio capture in rust-core/tests/speech/audio_capture_test.rs
- [X] T033 [GREEN] [P] [US1] Implement Azure Speech API client in rust-core/src/speech/client.rs (make T031 pass)
- [X] T034 [GREEN] [P] [US1] Implement audio capture module in rust-core/src/speech/audio_capture.rs (make T032 pass)
- [X] T035 [RED] [US1] Write tests for speech recognition service in rust-core/tests/speech/service_test.rs
- [X] T036 [GREEN] [US1] Implement speech recognition service in rust-core/src/speech/mod.rs (make T035 pass, depends on T033, T034)

#### Text Processing (Test-First)
- [X] T037 [RED] [P] [US1] Write tests for filler word removal in rust-core/tests/processing/filler_removal_test.rs
- [X] T038 [RED] [P] [US1] Write tests for self-correction detection in rust-core/tests/processing/self_correction_test.rs
- [X] T039 [RED] [P] [US1] Write tests for text formatter in rust-core/tests/processing/formatter_test.rs
- [X] T040 [GREEN] [P] [US1] Create filler word removal module in rust-core/src/processing/filler_removal.rs (make T037 pass)
- [X] T041 [GREEN] [P] [US1] Create self-correction detection module in rust-core/src/processing/self_correction.rs (make T038 pass)
- [X] T042 [GREEN] [P] [US1] Implement text formatter in rust-core/src/processing/formatter.rs (make T039 pass)
- [X] T043 [RED] [US1] Write tests for text processing pipeline in rust-core/tests/processing/pipeline_test.rs
- [X] T044 [GREEN] [US1] Implement text processing pipeline in rust-core/src/processing/mod.rs (make T043 pass, depends on T040, T041, T042)

#### AI Polishing (Test-First)
- [X] T045 [RED] [P] [US1] Write tests for AI prompt templates in rust-core/tests/ai/prompts_test.rs
- [X] T046 [RED] [P] [US1] Write tests for Claude API client in rust-core/tests/ai/client_test.rs
- [X] T047 [GREEN] [P] [US1] Create AI prompt templates in rust-core/src/ai/prompts.rs (make T045 pass)
- [X] T048 [GREEN] [P] [US1] Implement Claude API client in rust-core/src/ai/client.rs (make T046 pass)
- [X] T049 [RED] [US1] Write tests for text polishing service in rust-core/tests/ai/polisher_test.rs
- [X] T050 [GREEN] [US1] Implement text polishing service in rust-core/src/ai/polisher.rs (make T049 pass, depends on T047, T048)

#### FFI Bridge (Test-First)
- [X] T051 [RED] [US1] Write tests for session management FFI in rust-core/tests/ffi/session_test.rs
- [X] T052 [RED] [US1] Write tests for transcription FFI in rust-core/tests/ffi/transcription_test.rs
- [X] T053 [GREEN] [US1] Create session management FFI functions in rust-core/src/ffi/bridge.rs (start_voice_session, stop_voice_session, cancel_voice_session, make T051 pass)
- [X] T054 [GREEN] [US1] Create transcription FFI functions in rust-core/src/ffi/bridge.rs (transcribe_audio, polish_text, make T052 pass)
- [ ] T055 [GREEN] [US1] Regenerate Flutter bindings with flutter_rust_bridge_codegen

#### Flutter UI (Test-First)
- [X] T056 [RED] [P] [US1] Write widget tests for voice button in flutter-ui/test/widgets/voice_button_test.dart
- [X] T057 [RED] [P] [US1] Write widget tests for transcription preview in flutter-ui/test/widgets/transcription_preview_test.dart
- [X] T058 [GREEN] [P] [US1] Create VoiceSession Dart model in flutter-ui/lib/models/session.dart
- [X] T059 [GREEN] [P] [US1] Implement voice state management in flutter-ui/lib/state/voice_state.dart
- [X] T060 [GREEN] [P] [US1] Create voice button widget in flutter-ui/lib/widgets/voice_button.dart (make T056 pass)
- [X] T061 [GREEN] [P] [US1] Create transcription preview widget in flutter-ui/lib/widgets/transcription_preview.dart (make T057 pass)
- [X] T062 [RED] [US1] Write integration tests for home screen in flutter-ui/integration_test/home_screen_test.dart
- [X] T063 [GREEN] [US1] Implement home screen with voice input in flutter-ui/lib/screens/home_screen.dart (make T062 pass, depends on T060, T061)

#### Additional Features (Test-First)
- [X] T064 [RED] [US1] Write tests for audio level visualization in flutter-ui/test/widgets/voice_button_audio_test.dart
- [X] T065 [GREEN] [US1] Add audio level visualization to voice button widget in flutter-ui/lib/widgets/voice_button.dart (make T064 pass)
- [X] T066 [RED] [US1] Write tests for session timeout handling in rust-core/tests/ffi/timeout_test.rs
- [X] T067 [GREEN] [US1] Implement session timeout handling (5-minute max) in rust-core/src/ffi/bridge.rs (make T066 pass)
- [X] T068 [RED] [US1] Write tests for error handling in flutter-ui/test/screens/home_screen_error_test.dart
- [X] T069 [GREEN] [US1] Add error handling and user feedback for API failures in flutter-ui/lib/screens/home_screen.dart (make T068 pass)
- [X] T070 [RED] [US1] Write tests for usage quota checking in rust-core/tests/quota/check_test.rs
- [X] T071 [GREEN] [US1] Integrate usage quota checking before session start in rust-core/src/ffi/bridge.rs (make T070 pass)

#### Refactoring & Optimization
- [ ] T072 [IMPROVE] [US1] Refactor speech recognition module for clarity and performance
- [ ] T073 [IMPROVE] [US1] Refactor text processing pipeline for maintainability
- [ ] T074 [IMPROVE] [US1] Refactor AI polishing service for error resilience

#### Integration Testing
- [ ] T075 [US1] End-to-end integration test: Complete voice-to-text flow (activate → speak → transcribe → polish → display)

**Checkpoint**: At this point, User Story 1 should be fully functional - users can speak and receive polished text

---

## Phase 4: User Story 2 - Context-Aware Tone Adaptation (Priority: P2)

**Goal**: Automatically adjust output tone based on detected application context (formal for email, casual for chat)

**Independent Test**: User can speak the same casual phrase in different applications (Slack vs Gmail) and receive appropriately adapted output

### Implementation for User Story 2

#### Context Detection (Test-First)
- [ ] T076 [RED] [P] [US2] Write tests for ApplicationContext model in rust-core/tests/storage/context_test.rs
- [ ] T077 [GREEN] [P] [US2] Create ApplicationContext model in rust-core/src/storage/models.rs (make T076 pass)
- [ ] T078 [RED] [P] [US2] Write tests for Windows context detection in rust-core/tests/context/windows_test.rs
- [ ] T079 [RED] [P] [US2] Write tests for macOS context detection in rust-core/tests/context/macos_test.rs
- [ ] T080 [RED] [P] [US2] Write tests for Linux context detection in rust-core/tests/context/linux_test.rs
- [ ] T081 [RED] [P] [US2] Write tests for iOS context detection in rust-core/tests/context/ios_test.rs
- [ ] T082 [RED] [P] [US2] Write tests for Android context detection in rust-core/tests/context/android_test.rs
- [ ] T083 [GREEN] [P] [US2] Implement Windows context detection in rust-core/src/context/windows.rs (make T078 pass)
- [ ] T084 [GREEN] [P] [US2] Implement macOS context detection in rust-core/src/context/macos.rs (make T079 pass)
- [ ] T085 [GREEN] [P] [US2] Implement Linux context detection in rust-core/src/context/linux.rs (make T080 pass)
- [ ] T086 [GREEN] [P] [US2] Implement iOS context detection (manual selection fallback) in rust-core/src/context/ios.rs (make T081 pass)
- [ ] T087 [GREEN] [P] [US2] Implement Android context detection in rust-core/src/context/android.rs (make T082 pass)
- [ ] T088 [RED] [US2] Write tests for unified context detector in rust-core/tests/context/detector_test.rs
- [ ] T089 [GREEN] [US2] Create unified context detector in rust-core/src/context/detector.rs (make T088 pass, depends on T083-T087)

#### Context-Aware AI (Test-First)
- [ ] T090 [RED] [P] [US2] Write tests for context-specific AI prompts in rust-core/tests/ai/context_prompts_test.rs
- [ ] T091 [GREEN] [P] [US2] Create context-specific AI prompts (email, chat, document, code, browser) in rust-core/src/ai/prompts.rs (make T090 pass)
- [ ] T092 [RED] [US2] Write tests for context-aware polishing in rust-core/tests/ai/context_polisher_test.rs
- [ ] T093 [GREEN] [US2] Update text polishing service to use context-aware prompts in rust-core/src/ai/polisher.rs (make T092 pass)

#### FFI & UI (Test-First)
- [ ] T094 [RED] [US2] Write tests for context detection FFI in rust-core/tests/ffi/context_test.rs
- [ ] T095 [GREEN] [US2] Create context detection FFI functions in rust-core/src/ffi/bridge.rs (detect_application_context, get_all_contexts, make T094 pass)
- [ ] T096 [GREEN] [US2] Regenerate Flutter bindings with flutter_rust_bridge_codegen
- [ ] T097 [GREEN] [P] [US2] Create ApplicationContext Dart model in flutter-ui/lib/models/context.dart
- [ ] T098 [RED] [US2] Write tests for context-aware voice state in flutter-ui/test/state/voice_state_context_test.dart
- [ ] T099 [GREEN] [US2] Update voice state to include detected context in flutter-ui/lib/state/voice_state.dart (make T098 pass)
- [ ] T100 [RED] [US2] Write tests for context indicator widget in flutter-ui/test/widgets/context_indicator_test.dart
- [ ] T101 [GREEN] [US2] Add context indicator to transcription preview widget in flutter-ui/lib/widgets/transcription_preview.dart (make T100 pass)
- [ ] T102 [RED] [US2] Write tests for automatic context detection in flutter-ui/integration_test/context_detection_test.dart
- [ ] T103 [GREEN] [US2] Implement automatic context detection on session start in flutter-ui/lib/screens/home_screen.dart (make T102 pass)

#### Refactoring
- [ ] T104 [IMPROVE] [US2] Refactor context detection for cross-platform consistency

#### Integration Testing
- [ ] T105 [US2] End-to-end integration test: Context-aware tone adaptation (same phrase in Gmail vs Slack produces different tones)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - tone adapts based on application context

---

## Phase 5: User Story 3 - Personal Dictionary for Professional Terms (Priority: P3)

**Goal**: Enable users to add custom terms to personal dictionary for 100% accurate recognition of specialized vocabulary

**Independent Test**: User can add custom terms to personal dictionary, then speak sentences containing these terms and receive 100% accurate recognition

### Implementation for User Story 3

#### Data Model (Test-First)
- [ ] T151 [RED] [P] [US3] Write tests for PersonalDictionaryEntry model in rust-core/tests/storage/dictionary_test.rs
- [ ] T152 [GREEN] [P] [US3] Create PersonalDictionaryEntry model in rust-core/src/storage/models.rs (make T151 pass)

#### Dictionary Storage (Test-First)
- [ ] T153 [RED] [P] [US3] Write tests for dictionary storage operations in rust-core/tests/storage/dict_ops_test.rs
- [ ] T154 [GREEN] [P] [US3] Implement dictionary storage operations in rust-core/src/storage/dictionary.rs (make T153 pass)

#### Dictionary Application (Test-First)
- [ ] T155 [RED] [P] [US3] Write tests for dictionary application logic in rust-core/tests/processing/dict_app_test.rs
- [ ] T156 [GREEN] [US3] Implement dictionary application logic in rust-core/src/processing/dictionary.rs (make T155 pass)

#### Processing Pipeline Integration (Test-First)
- [ ] T157 [RED] [US3] Write tests for pipeline with dictionary in rust-core/tests/processing/pipeline_dict_test.rs
- [ ] T158 [GREEN] [US3] Update text processing pipeline to apply dictionary in rust-core/src/processing/mod.rs (make T157 pass, depends on T156)

#### FFI Functions (Test-First)
- [ ] T159 [RED] [US3] Write tests for dictionary FFI in rust-core/tests/ffi/dict_test.rs
- [ ] T160 [GREEN] [US3] Create dictionary management FFI functions in rust-core/src/ffi/bridge.rs (add_dictionary_entry, remove_dictionary_entry, get_all_dictionary_entries, update_dictionary_entry, make T159 pass)

#### Flutter Binding (Test-First)
- [ ] T161 [GREEN] [US3] Regenerate Flutter bindings with flutter_rust_bridge_codegen

#### Flutter UI (Test-First)
- [ ] T162 [GREEN] [P] [US3] Create PersonalDictionaryEntry Dart model in flutter-ui/lib/models/dictionary.dart
- [ ] T163 [RED] [P] [US3] Write tests for dictionary screen in flutter-ui/test/screens/dictionary_screen_test.dart
- [ ] T164 [GREEN] [P] [US3] Create dictionary screen UI in flutter-ui/lib/screens/dictionary_screen.dart (make T163 pass)
- [ ] T165 [RED] [US3] Write tests for add dictionary entry dialog in flutter-ui/test/screens/dict_add_dialog_test.dart
- [ ] T166 [GREEN] [US3] Implement add dictionary entry dialog in flutter-ui/lib/screens/dictionary_screen.dart (make T165 pass)
- [ ] T167 [RED] [US3] Write tests for edit dictionary entry dialog in flutter-ui/test/screens/dict_edit_dialog_test.dart
- [ ] T168 [GREEN] [US3] Implement edit dictionary entry dialog in flutter-ui/lib/screens/dictionary_screen.dart (make T167 pass)
- [ ] T169 [RED] [US3] Write tests for delete confirmation in flutter-ui/test/screens/dict_delete_test.dart
- [ ] T170 [GREEN] [US3] Implement delete dictionary entry confirmation in flutter-ui/lib/screens/dictionary_screen.dart (make T169 pass)

#### Import/Export & Navigation (Test-First)
- [ ] T171 [RED] [US3] Write tests for import/export in rust-core/tests/storage/dict_export_test.rs
- [ ] T172 [GREEN] [US3] Add dictionary import/export functionality in rust-core/src/storage/dictionary.rs (make T171 pass)
- [ ] T173 [RED] [US3] Write tests for settings navigation in flutter-ui/test/screens/settings_navigation_test.dart
- [ ] T174 [GREEN] [US3] Add navigation to dictionary screen from settings in flutter-ui/lib/screens/settings_screen.dart (make T173 pass)

#### Refactoring
- [ ] T175 [IMPROVE] [US3] Refactor dictionary module for clarity and performance

#### Integration Testing
- [ ] T176 [US3] End-to-end integration test: Personal dictionary workflow (add term → speak sentence → term recognized)

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work - users can manage custom vocabulary

---

## Phase 6: User Story 4 - Multi-language and Real-time Translation (Priority: P4)

**Goal**: Support voice input in multiple languages with optional real-time translation to target languages

**Independent Test**: User can speak in Chinese, receive polished Chinese text, then optionally translate to English with natural phrasing

### Implementation for User Story 4

#### Multi-language Speech (Test-First)
- [ ] T177 [RED] [P] [US4] Write tests for multi-language Azure Speech client in rust-core/tests/speech/multilang_test.rs
- [ ] T178 [RED] [P] [US4] Write tests for language detection in rust-core/tests/speech/lang_detect_test.rs
- [ ] T179 [GREEN] [P] [US4] Add multi-language support to Azure Speech API client in rust-core/src/speech/client.rs (make T177 pass)
- [ ] T180 [GREEN] [P] [US4] Implement automatic language detection in rust-core/src/speech/client.rs (make T178 pass)

#### Multi-language Processing (Test-First)
- [ ] T181 [RED] [P] [US4] Write tests for multi-language filler word lists in rust-core/tests/processing/filler_multilang_test.rs
- [ ] T182 [GREEN] [P] [US4] Add multi-language filler word lists in rust-core/src/processing/filler_removal.rs (make T181 pass)

#### Translation Service (Test-First)
- [ ] T183 [RED] [P] [US4] Write tests for translation service in rust-core/tests/ai/translation_test.rs
- [ ] T184 [GREEN] [P] [US4] Create translation service using Claude API in rust-core/src/ai/translator.rs (make T183 pass)

#### FFI Translation Functions (Test-First)
- [ ] T185 [RED] [US4] Write tests for translation FFI in rust-core/tests/ffi/translation_test.rs
- [ ] T186 [GREEN] [US4] Add translation FFI functions in rust-core/src/ffi/bridge.rs (translate_text, detect_language, make T185 pass)

#### Flutter Translation UI (Test-First)
- [ ] T187 [GREEN] [P] [US4] Create language model in flutter-ui/lib/models/language.dart
- [ ] T188 [RED] [P] [US4] Write tests for language selector widget in flutter-ui/test/widgets/language_selector_test.dart
- [ ] T189 [GREEN] [P] [US4] Create language selector widget in flutter-ui/lib/widgets/language_selector.dart (make T188 pass)
- [ ] T190 [RED] [US4] Write tests for translation toggle in flutter-ui/test/widgets/translation_toggle_test.dart
- [ ] T191 [GREEN] [US4] Add translation toggle to transcription preview in flutter-ui/lib/widgets/transcription_preview.dart (make T190 pass)
- [ ] T192 [RED] [US4] Write tests for settings language screen in flutter-ui/test/screens/settings_language_test.dart
- [ ] T193 [GREEN] [US4] Update settings screen with language preferences in flutter-ui/lib/screens/settings_screen.dart (make T192 pass)

#### Refactoring
- [ ] T194 [IMPROVE] [US4] Refactor translation service for Quality and error resilience

#### Integration Testing
- [ ] T195 [US4] End-to-end integration test: Multi-language workflow (select language → speak → transcribe → optional translate)

**Checkpoint**: At this point, User Stories 1-4 should all work - multi-language support is functional

---

## Phase 7: User Story 5 - AI Assistant Commands (Priority: P5) ⚠️ OUT OF SCOPE

**Status**: DEFERRED TO v2.0 - Not part of MVP

**Rationale**: Core voice-to-text enhancement (US1-US4) provides immediate value. Command execution requires OS-level permissions, extensive security review, and wake word detection infrastructure. Target for Q3 2026 release after MVP validation.

**Affected Requirements**: FR-013 (voice commands for text editing)

**Future Implementation** (when in scope):
- Wake word detection for command activation
- Command parser and recognition engine
- Command execution service with safety checks
- Custom command registry
- Command palette UI

**For MVP**: Skip to Phase 8 after completing Phase 6 (US4)

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories + Non-Functional Requirements validation

### Polish Tasks
- [ ] T196 [P] Create settings screen with all preferences in flutter-ui/lib/screens/settings_screen.dart
- [ ] T197 [P] Implement usage quota indicator widget in flutter-ui/lib/widgets/quota_indicator.dart
- [ ] T198 [P] Add quota warning notifications in flutter-ui/lib/screens/home_screen.dart
- [ ] T199 [P] Implement crash reporting opt-in/opt-out in flutter-ui/lib/screens/settings_screen.dart
- [ ] T200 [P] Add local transcription history view in flutter-ui/lib/screens/history_screen.dart
- [ ] T201 [P] Implement data export functionality in rust-core/src/storage/export.rs
- [ ] T202 [P] Implement data deletion (factory reset) in rust-core/src/storage/cleanup.rs
- [ ] T203 [P] Add keyboard shortcuts for voice activation in flutter-ui/lib/services/shortcuts.dart
- [ ] T204 [P] Implement push-to-talk mode in flutter-ui/lib/widgets/voice_button.dart
- [ ] T205 [P] Add microphone selection UI in flutter-ui/lib/screens/settings_screen.dart
- [ ] T206 [P] Implement noise cancellation toggle in rust-core/src/speech/audio_capture.rs
- [ ] T207 [P] Add usage analytics display in flutter-ui/lib/screens/settings_screen.dart
- [ ] T208 [RED] Write tests for offline mode handling in rust-core/tests/network/offline_test.rs
- [ ] T209 [GREEN] Implement offline detection and fallback logic in rust-core/src/network/offline_handler.rs (make T208 pass)

### Non-Functional Requirements Validation (Constitution Compliance)
- [ ] T210 [RED] Write performance benchmark tests in rust-core/tests/benchmarks/performance_test.rs
- [ ] T211 [GREEN] Implement performance profiler in rust-core/src/tools/profiler.rs
- [ ] T212 Run performance benchmarks and validate against constitution targets:
  - Context detection <50ms (p95)
  - Memory footprint <100MB idle, <300MB active
  - UI rendering 60fps minimum
  - CPU usage <5% idle, <30% during AI processing
- [ ] T213 [RED] Write security audit tests in rust-core/tests/security/audit_test.rs
- [ ] T214 Run security audit: cargo audit, FFI boundary review, prompt injection tests, encryption validation
- [ ] T215 [RED] Write accessibility tests in flutter-ui/test/accessibility/a11y_test.rs
- [ ] T216 Run accessibility validation: screen reader compatibility, keyboard navigation, WCAG 2.1 AA compliance
- [ ] T217 Document NFR validation results in docs/validation-report.md

### Code Quality & Documentation
- [ ] T218 Code cleanup and refactoring across all modules
- [ ] T219 Performance optimization for <200ms UI response target
- [ ] T220 Memory optimization for <100MB idle target
- [ ] T221 [P] Update README.md with project overview and setup instructions
- [ ] T222 [P] Validate quickstart.md setup instructions
- [ ] T223 Final security and privacy compliance review

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4 → P5)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Extends US1 but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Extends US1 but independently testable
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Extends US1 but independently testable
- **User Story 5 (P5)**: Can start after Foundational (Phase 2) - Extends US1 but independently testable

### Within Each User Story

- Models before services
- Services before FFI functions
- FFI functions before Flutter bindings regeneration
- Flutter bindings before Flutter UI components
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Models within a story marked [P] can run in parallel
- Platform-specific implementations marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all models for User Story 1 together:
Task T027: "Create VoiceSession model in rust-core/src/storage/models.rs"
Task T028: "Create TranscriptionHistory model in rust-core/src/storage/models.rs"

# Launch all processing modules for User Story 1 together:
Task T032: "Create filler word removal module in rust-core/src/processing/filler_removal.rs"
Task T033: "Create self-correction detection module in rust-core/src/processing/self_correction.rs"
Task T034: "Implement text formatter in rust-core/src/processing/formatter.rs"

# Launch all AI modules for User Story 1 together:
Task T036: "Create AI prompt templates in rust-core/src/ai/prompts.rs"
Task T037: "Implement Claude API client in rust-core/src/ai/client.rs"

# Launch all Flutter widgets for User Story 1 together:
Task T044: "Create voice button widget in flutter-ui/lib/widgets/voice_button.dart"
Task T045: "Create transcription preview widget in flutter-ui/lib/widgets/transcription_preview.dart"
```

---

## Parallel Example: User Story 2

```bash
# Launch all platform-specific context detection together:
Task T052: "Implement Windows context detection in rust-core/src/context/windows.rs"
Task T053: "Implement macOS context detection in rust-core/src/context/macos.rs"
Task T054: "Implement Linux context detection in rust-core/src/context/linux.rs"
Task T055: "Implement iOS context detection in rust-core/src/context/ios.rs"
Task T056: "Implement Android context detection in rust-core/src/context/android.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T008)
2. Complete Phase 2: Foundational (T009-T026) - CRITICAL - blocks all stories
3. Complete Phase 3: User Story 1 (T027-T050)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

**MVP Scope**: Real-time voice to polished text with filler removal and self-correction (User Story 1 only)

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo (Context-aware tone)
4. Add User Story 3 → Test independently → Deploy/Demo (Personal dictionary)
5. Add User Story 4 → Test independently → Deploy/Demo (Multi-language)
6. Add User Story 5 → Test independently → Deploy/Demo (AI commands)
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (P1)
   - Developer B: User Story 2 (P2)
   - Developer C: User Story 3 (P3)
3. Stories complete and integrate independently

---

## Task Summary

**Total Tasks**: 223
- **Phase 1 (Setup)**: 7 tasks (T001-T007)
- **Phase 2 (Foundational)**: 18 tasks (T009-T026)
- **Phase 3 (US1 - Real-time Voice to Polished Text)**: 49 tasks (T027-T075)
- **Phase 4 (US2 - Context-Aware Tone)**: 30 tasks (T076-T105)
- **Phase 5 (US3 - Personal Dictionary)**: 26 tasks (T151-T176)
- **Phase 6 (US4 - Multi-language Translation)**: 19 tasks (T177-T195)
- **Phase 7 (US5 - AI Assistant Commands)**: 0 tasks (OUT OF SCOPE)
- **Phase 8 (Polish)**: 28 tasks (T196-T223)

**Parallel Opportunities**: 100+ tasks marked [P] can run in parallel within their phase

**MVP Scope**: Phases 1-3 (56 tasks) deliver User Story 1 - Real-time Voice to Polished Text

**Independent Test Criteria**:
- **US1**: User can speak naturally and receive polished text within 200ms (p95)
- **US2**: Same phrase produces different tones in different applications
- **US3**: Custom dictionary terms are recognized with 100% accuracy
- **US4**: User can speak in multiple languages with optional translation
- **US5**: Voice commands transform text without keyboard input (deferred to v2.0)

---

## NOTES

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- FFI bindings must be regenerated after each Rust API change
- Platform-specific implementations can be developed in parallel
- TDD workflow: RED (write test) → GREEN (implement) → IMPROVE (refactor)
