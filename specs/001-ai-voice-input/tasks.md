# Tasks: Talkute AI Voice Input Assistant

**Input**: Design documents from `/specs/001-ai-voice-input/`
**Prerequisites**: plan.md, spec.md, data-model.md, contracts/, research.md, quickstart.md

**Tests**: Tests are NOT explicitly requested in the specification, so test tasks are EXCLUDED per template guidelines.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

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

- [ ] T001 Create project directory structure per plan.md (rust-core/, flutter-ui/, platform/, specs/)
- [ ] T002 Initialize Rust workspace with Cargo.toml in rust-core/
- [ ] T003 [P] Initialize Flutter project with pubspec.yaml in flutter-ui/
- [ ] T004 [P] Configure rustfmt and clippy in rust-core/.rustfmt.toml and rust-core/.clippy.toml
- [ ] T005 [P] Configure Dart formatting in flutter-ui/analysis_options.yaml
- [ ] T006 Create .env.template file with required API keys (AZURE_SPEECH_KEY, ANTHROPIC_API_KEY)
- [ ] T007 [P] Add .env to .gitignore
- [ ] T008 Install flutter_rust_bridge_codegen tool via cargo install

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T009 Create SQLite database schema with migrations in rust-core/migrations/001_initial_schema.sql
- [ ] T010 [P] Implement database connection module in rust-core/src/storage/database.rs
- [ ] T011 [P] Create DeviceProfile model in rust-core/src/storage/models.rs
- [ ] T012 [P] Create UsageQuota model in rust-core/src/storage/models.rs
- [ ] T013 Implement database migration runner in rust-core/src/storage/migrations.rs
- [ ] T014 [P] Setup tokio async runtime configuration in rust-core/src/lib.rs
- [ ] T015 [P] Create error types module in rust-core/src/error.rs
- [ ] T016 [P] Implement logging infrastructure with env_logger in rust-core/src/lib.rs
- [ ] T017 Create FFI bridge skeleton in rust-core/src/ffi/bridge.rs
- [ ] T018 Generate initial Flutter bindings with flutter_rust_bridge_codegen
- [ ] T019 [P] Create Flutter app state management structure with Riverpod in flutter-ui/lib/state/app_state.dart
- [ ] T020 [P] Implement platform-specific initialization for Windows in platform/windows/
- [ ] T021 [P] Implement platform-specific initialization for macOS in platform/macos/
- [ ] T022 [P] Implement platform-specific initialization for Linux in platform/linux/
- [ ] T023 [P] Implement platform-specific initialization for iOS in platform/ios/
- [ ] T024 [P] Implement platform-specific initialization for Android in platform/android/
- [ ] T025 Create device profile initialization logic in rust-core/src/storage/profile.rs
- [ ] T026 Implement usage quota tracking module in rust-core/src/quota/tracker.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Real-time Voice to Polished Text (Priority: P1) 🎯 MVP

**Goal**: Enable users to speak naturally and receive clean, polished text output with filler words removed and self-corrections applied

**Independent Test**: User can activate voice input, speak naturally (including filler words and self-corrections), and receive clean, polished text output within 500ms

### Implementation for User Story 1

- [ ] T027 [P] [US1] Create VoiceSession model in rust-core/src/storage/models.rs
- [ ] T028 [P] [US1] Create TranscriptionHistory model in rust-core/src/storage/models.rs
- [ ] T029 [P] [US1] Implement Azure Speech API client in rust-core/src/speech/client.rs
- [ ] T030 [P] [US1] Implement audio capture module in rust-core/src/speech/audio_capture.rs
- [ ] T031 [US1] Implement speech recognition service in rust-core/src/speech/mod.rs (depends on T029, T030)
- [ ] T032 [P] [US1] Create filler word removal module in rust-core/src/processing/filler_removal.rs
- [ ] T033 [P] [US1] Create self-correction detection module in rust-core/src/processing/self_correction.rs
- [ ] T034 [P] [US1] Implement text formatter in rust-core/src/processing/formatter.rs
- [ ] T035 [US1] Implement text processing pipeline in rust-core/src/processing/mod.rs (depends on T032, T033, T034)
- [ ] T036 [P] [US1] Create AI prompt templates in rust-core/src/ai/prompts.rs
- [ ] T037 [P] [US1] Implement Claude API client in rust-core/src/ai/client.rs
- [ ] T038 [US1] Implement text polishing service in rust-core/src/ai/polisher.rs (depends on T036, T037)
- [ ] T039 [US1] Create session management FFI functions in rust-core/src/ffi/bridge.rs (start_voice_session, stop_voice_session, cancel_voice_session)
- [ ] T040 [US1] Create transcription FFI functions in rust-core/src/ffi/bridge.rs (transcribe_audio, polish_text)
- [ ] T041 [US1] Regenerate Flutter bindings with flutter_rust_bridge_codegen
- [ ] T042 [P] [US1] Create VoiceSession Dart model in flutter-ui/lib/models/session.dart
- [ ] T043 [P] [US1] Implement voice state management in flutter-ui/lib/state/voice_state.dart
- [ ] T044 [P] [US1] Create voice button widget in flutter-ui/lib/widgets/voice_button.dart
- [ ] T045 [P] [US1] Create transcription preview widget in flutter-ui/lib/widgets/transcription_preview.dart
- [ ] T046 [US1] Implement home screen with voice input in flutter-ui/lib/screens/home_screen.dart (depends on T044, T045)
- [ ] T047 [US1] Add audio level visualization to voice button widget in flutter-ui/lib/widgets/voice_button.dart
- [ ] T048 [US1] Implement session timeout handling (5-minute max) in rust-core/src/ffi/bridge.rs
- [ ] T049 [US1] Add error handling and user feedback for API failures in flutter-ui/lib/screens/home_screen.dart
- [ ] T050 [US1] Integrate usage quota checking before session start in rust-core/src/ffi/bridge.rs

**Checkpoint**: At this point, User Story 1 should be fully functional - users can speak and receive polished text

---

## Phase 4: User Story 2 - Context-Aware Tone Adaptation (Priority: P2)

**Goal**: Automatically adjust output tone based on detected application context (formal for email, casual for chat)

**Independent Test**: User can speak the same casual phrase in different applications (Slack vs Gmail) and receive appropriately adapted output

### Implementation for User Story 2

- [ ] T051 [P] [US2] Create ApplicationContext model in rust-core/src/storage/models.rs
- [ ] T052 [P] [US2] Implement Windows context detection in rust-core/src/context/windows.rs
- [ ] T053 [P] [US2] Implement macOS context detection in rust-core/src/context/macos.rs
- [ ] T054 [P] [US2] Implement Linux context detection in rust-core/src/context/linux.rs
- [ ] T055 [P] [US2] Implement iOS context detection (manual selection fallback) in rust-core/src/context/ios.rs
- [ ] T056 [P] [US2] Implement Android context detection in rust-core/src/context/android.rs
- [ ] T057 [US2] Create unified context detector in rust-core/src/context/detector.rs (depends on T052-T056)
- [ ] T058 [P] [US2] Create context-specific AI prompts (email, chat, document, code, browser) in rust-core/src/ai/prompts.rs
- [ ] T059 [US2] Update text polishing service to use context-aware prompts in rust-core/src/ai/polisher.rs
- [ ] T060 [US2] Create context detection FFI functions in rust-core/src/ffi/bridge.rs (detect_application_context, get_all_contexts)
- [ ] T061 [US2] Regenerate Flutter bindings with flutter_rust_bridge_codegen
- [ ] T062 [P] [US2] Create ApplicationContext Dart model in flutter-ui/lib/models/context.dart
- [ ] T063 [US2] Update voice state to include detected context in flutter-ui/lib/state/voice_state.dart
- [ ] T064 [US2] Add context indicator to transcription preview widget in flutter-ui/lib/widgets/transcription_preview.dart
- [ ] T065 [US2] Implement automatic context detection on session start in flutter-ui/lib/screens/home_screen.dart

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - tone adapts based on application context

---

## Phase 5: User Story 3 - Personal Dictionary for Professional Terms (Priority: P3)

**Goal**: Enable users to add custom terms to personal dictionary for 100% accurate recognition of specialized vocabulary

**Independent Test**: User can add custom terms to personal dictionary, then speak sentences containing these terms and receive 100% accurate recognition

### Implementation for User Story 3

- [ ] T066 [P] [US3] Create PersonalDictionaryEntry model in rust-core/src/storage/models.rs
- [ ] T067 [P] [US3] Implement dictionary storage operations in rust-core/src/storage/dictionary.rs
- [ ] T068 [US3] Implement dictionary application logic in rust-core/src/processing/dictionary.rs
- [ ] T069 [US3] Update text processing pipeline to apply dictionary in rust-core/src/processing/mod.rs
- [ ] T070 [US3] Create dictionary management FFI functions in rust-core/src/ffi/bridge.rs (add_dictionary_entry, remove_dictionary_entry, get_all_dictionary_entries, update_dictionary_entry)
- [ ] T071 [US3] Regenerate Flutter bindings with flutter_rust_bridge_codegen
- [ ] T072 [P] [US3] Create PersonalDictionaryEntry Dart model in flutter-ui/lib/models/dictionary.dart
- [ ] T073 [P] [US3] Create dictionary screen UI in flutter-ui/lib/screens/dictionary_screen.dart
- [ ] T074 [US3] Implement add dictionary entry dialog in flutter-ui/lib/screens/dictionary_screen.dart
- [ ] T075 [US3] Implement edit dictionary entry dialog in flutter-ui/lib/screens/dictionary_screen.dart
- [ ] T076 [US3] Implement delete dictionary entry confirmation in flutter-ui/lib/screens/dictionary_screen.dart
- [ ] T077 [US3] Add dictionary import/export functionality in rust-core/src/storage/dictionary.rs
- [ ] T078 [US3] Add navigation to dictionary screen from settings in flutter-ui/lib/screens/settings_screen.dart

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work - users can manage custom vocabulary

---

## Phase 6: User Story 4 - Multi-language and Real-time Translation (Priority: P4)

**Goal**: Support voice input in multiple languages with optional real-time translation to target languages

**Independent Test**: User can speak in Chinese, receive polished Chinese text, then optionally translate to English with natural phrasing

### Implementation for User Story 4

- [ ] T079 [P] [US4] Add multi-language support to Azure Speech API client in rust-core/src/speech/client.rs
- [ ] T080 [P] [US4] Implement automatic language detection in rust-core/src/speech/client.rs
- [ ] T081 [P] [US4] Add multi-language filler word lists in rust-core/src/processing/filler_removal.rs
- [ ] T082 [P] [US4] Create translation service using Claude API in rust-core/src/ai/translator.rs
- [ ] T083 [US4] Add translation FFI functions in rust-core/src/ffi/bridge.rs (translate_text, detect_language)
- [ ] T084 [US4] Regenerate Flutter bindings with flutter_rust_bridge_codegen
- [ ] T085 [P] [US4] Add language selection to device profile in flutter-ui/lib/models/settings.dart
- [ ] T086 [P] [US4] Create language selector widget in flutter-ui/lib/widgets/language_selector.dart
- [ ] T087 [US4] Add translation toggle to transcription preview in flutter-ui/lib/widgets/transcription_preview.dart
- [ ] T088 [US4] Update settings screen with language preferences in flutter-ui/lib/screens/settings_screen.dart

**Checkpoint**: At this point, User Stories 1-4 should all work - multi-language support is functional

---

## Phase 7: User Story 5 - AI Assistant Commands (Priority: P5)

**Goal**: Enable voice commands for text editing (shorten, expand, change tone, summarize) without switching to keyboard

**Independent Test**: User can speak commands like "make this shorter", "change tone to formal", "summarize this paragraph" and receive appropriate text transformations

### Implementation for User Story 5

- [ ] T089 [P] [US5] Create command parser module in rust-core/src/ai/commands.rs
- [ ] T090 [P] [US5] Implement command detection logic in rust-core/src/ai/commands.rs
- [ ] T091 [P] [US5] Create command-specific AI prompts in rust-core/src/ai/prompts.rs
- [ ] T092 [US5] Implement command execution service in rust-core/src/ai/command_executor.rs (depends on T089, T090, T091)
- [ ] T093 [US5] Add command execution FFI functions in rust-core/src/ffi/bridge.rs (execute_command, get_available_commands)
- [ ] T094 [US5] Regenerate Flutter bindings with flutter_rust_bridge_codegen
- [ ] T095 [P] [US5] Create command palette widget in flutter-ui/lib/widgets/command_palette.dart
- [ ] T096 [US5] Add command mode toggle to voice button in flutter-ui/lib/widgets/voice_button.dart
- [ ] T097 [US5] Implement command history in flutter-ui/lib/state/voice_state.dart
- [ ] T098 [US5] Add command suggestions to transcription preview in flutter-ui/lib/widgets/transcription_preview.dart

**Checkpoint**: All user stories should now be independently functional - full feature set complete

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T099 [P] Create settings screen with all preferences in flutter-ui/lib/screens/settings_screen.dart
- [ ] T100 [P] Implement usage quota indicator widget in flutter-ui/lib/widgets/quota_indicator.dart
- [ ] T101 [P] Add quota warning notifications in flutter-ui/lib/screens/home_screen.dart
- [ ] T102 [P] Implement crash reporting opt-in/opt-out in flutter-ui/lib/screens/settings_screen.dart
- [ ] T103 [P] Add local transcription history view in flutter-ui/lib/screens/history_screen.dart
- [ ] T104 [P] Implement data export functionality in rust-core/src/storage/export.rs
- [ ] T105 [P] Implement data deletion (factory reset) in rust-core/src/storage/cleanup.rs
- [ ] T106 [P] Add keyboard shortcuts for voice activation in flutter-ui/lib/services/shortcuts.dart
- [ ] T107 [P] Implement push-to-talk mode in flutter-ui/lib/widgets/voice_button.dart
- [ ] T108 [P] Add microphone selection UI in flutter-ui/lib/screens/settings_screen.dart
- [ ] T109 [P] Implement noise cancellation toggle in rust-core/src/speech/audio_capture.rs
- [ ] T110 [P] Add usage analytics display in flutter-ui/lib/screens/settings_screen.dart
- [ ] T111 Code cleanup and refactoring across all modules
- [ ] T112 Performance optimization for <500ms latency target
- [ ] T113 Memory optimization for <100MB idle target
- [ ] T114 [P] Update README.md with project overview and setup instructions
- [ ] T115 [P] Validate quickstart.md setup instructions
- [ ] T116 Security audit for data encryption and privacy compliance

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

**Total Tasks**: 116
- **Phase 1 (Setup)**: 8 tasks
- **Phase 2 (Foundational)**: 18 tasks
- **Phase 3 (US1 - Real-time Voice to Polished Text)**: 24 tasks
- **Phase 4 (US2 - Context-Aware Tone)**: 15 tasks
- **Phase 5 (US3 - Personal Dictionary)**: 13 tasks
- **Phase 6 (US4 - Multi-language Translation)**: 10 tasks
- **Phase 7 (US5 - AI Assistant Commands)**: 10 tasks
- **Phase 8 (Polish)**: 18 tasks

**Parallel Opportunities**: 67 tasks marked [P] can run in parallel within their phase

**MVP Scope**: Phases 1-3 (50 tasks) deliver User Story 1 - Real-time Voice to Polished Text

**Independent Test Criteria**:
- **US1**: User can speak naturally and receive polished text within 500ms
- **US2**: Same phrase produces different tones in different applications
- **US3**: Custom dictionary terms are recognized with 100% accuracy
- **US4**: User can speak in multiple languages with optional translation
- **US5**: Voice commands transform text without keyboard input

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- FFI bindings must be regenerated after each Rust API change
- Platform-specific implementations can be developed in parallel
- Tests are NOT included per specification (TDD not explicitly requested)
