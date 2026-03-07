# Task Breakdown: Product Experience Design Implementation

**Feature**: 002-product-experience-ux
**Branch**: `002-product-experience-ux`
**Created**: 2026-03-07
**Plan**: [plan.md](./plan.md) | **Spec**: [spec.md](./spec.md)

## Overview

This document provides a complete, actionable task breakdown for implementing the Product Experience Design feature. Tasks are organized by user story to enable independent implementation and testing of each feature increment.

**Total Tasks**: 179
**User Stories**: 10 (3 P1, 5 P2, 2 P3)
**Parallel Opportunities**: 45 tasks marked [P]

---

## Implementation Strategy

### MVP Scope (Recommended First Delivery)
Complete **Phase 1-3** (Setup + Foundational + US1-US3) to deliver:
- Working floating capsule with voice input
- System tray status indicators
- Push-to-talk and toggle modes
- Text injection at cursor

This delivers the core desktop experience and validates the architecture.

### Incremental Delivery
Each user story phase is independently testable:
- **P1 Stories (US1-US3)**: Desktop core experience - critical path
- **P2 Stories (US4-US8)**: Settings, history, dictionary, translation
- **P3 Stories (US9-US10)**: Mobile keyboard, polishing intensity

---

## Phase 1: Setup & Dependencies

**Goal**: Initialize project structure and add required dependencies

- [X] T001 Add Rust dependencies to rust-core/Cargo.toml (tray-item, global-hotkey, enigo, x11, winapi, cocoa, core-graphics)
- [X] T002 Add Flutter dependencies to flutter-ui/pubspec.yaml (window_manager, flutter_local_notifications)
- [X] T003 [P] Create database migration file rust-core/migrations/002_product_experience.sql with user_preferences, dictionary_entries, history_entries tables
- [X] T004 [P] Create platform module structure rust-core/src/platform/mod.rs
- [X] T005 [P] Create platform tray module rust-core/src/platform/tray.rs with PlatformTray trait
- [X] T006 [P] Create platform hotkey module rust-core/src/platform/hotkey.rs with PlatformHotkey trait
- [X] T007 [P] Create platform text_injection module rust-core/src/platform/text_injection.rs with TextInjector trait
- [X] T008 [P] Create platform window module rust-core/src/platform/window.rs with WindowManager trait
- [X] T009 Update rust-core/src/lib.rs to export new platform modules
- [X] T010 Run cargo check to verify Rust dependencies compile

---

## Phase 2: Foundational Infrastructure

**Goal**: Implement shared infrastructure required by all user stories

### Tests (TDD: Write First)

- [X] T160 [P] Write unit tests for session state machine transitions in rust-core/tests/unit/session_test.rs
- [X] T164 [P] Write integration tests for FFI functions in rust-core/tests/integration/ffi_test.rs

### Session State Machine

- [X] T011 Create SessionState enum in rust-core/src/state/mod.rs (Idle, Recording, Processing, Error)
- [X] T012 Create TranscriptionSession struct in rust-core/src/state/session.rs
- [X] T013 Create SessionManager with state transitions in rust-core/src/state/manager.rs
- [X] T014 [P] Create session_state_provider in flutter-ui/lib/state/session_state.dart

### Storage Layer Extensions

- [X] T015 [P] Create PreferencesRepository trait in rust-core/src/storage/preferences.rs
- [X] T016 [P] Implement SqlitePreferencesRepository in rust-core/src/storage/preferences.rs
- [X] T017 [P] Create DictionaryRepository trait in rust-core/src/storage/dictionary.rs
- [X] T018 [P] Implement SqliteDictionaryRepository in rust-core/src/storage/dictionary.rs
- [X] T019 [P] Create HistoryRepository trait in rust-core/src/storage/history.rs
- [X] T020 [P] Implement SqliteHistoryRepository in rust-core/src/storage/history.rs
- [X] T021 Create data cleanup module rust-core/src/storage/cleanup.rs with retention logic

### FFI Bridge Foundation

- [X] T022 Create FFI types in rust-core/src/ffi/types.rs (SessionStateEvent, DictionaryEntry, HistoryEntry, ApplicationInfo, TrayMenuItem, HotkeyEvent)
- [X] T023 Create FFI session functions in rust-core/src/ffi/session.rs (start_recording, stop_recording, cancel_recording, retry_processing)
- [X] T024 Create FFI state stream functions in rust-core/src/ffi/state_stream.rs (session_state_stream, audio_level_stream)
- [X] T025 Update rust-core/src/ffi/bridge.rs to export all FFI modules
- [X] T026 Run flutter_rust_bridge_codegen to generate Dart bindings

### Flutter Service Layer

- [X] T027 [P] Create EngineService in flutter-ui/lib/services/engine_service.dart (wraps session FFI)
- [X] T028 [P] Create ConfigService in flutter-ui/lib/services/config_service.dart (wraps preferences FFI)
- [X] T029 [P] Create TrayService in flutter-ui/lib/services/tray_service.dart (wraps tray FFI)
- [X] T030 [P] Create ShortcutsService in flutter-ui/lib/services/shortcuts.dart (wraps hotkey FFI)

---

## Phase 3: User Story 1 - Desktop Voice Input with Floating Capsule (P1)

**Goal**: Floating capsule appears on hotkey, shows waveform, inserts polished text at cursor
**Independent Test**: Press hotkey, speak, verify text appears at cursor

### Tests (TDD: Write First)

- [X] T165 [P] [US1] Write widget tests for FloatingCapsule states in flutter-ui/test/widgets/floating_capsule_test.dart

### Rust Core - Session Management

- [X] T031 [US1] Implement start_recording FFI function in rust-core/src/ffi/session.rs
- [X] T032 [US1] Implement stop_recording FFI function in rust-core/src/ffi/session.rs
- [X] T033 [US1] Implement session_state_stream FFI function in rust-core/src/ffi/state_stream.rs
- [X] T034 [US1] Implement audio_level_stream FFI function in rust-core/src/ffi/state_stream.rs

### Rust Core - Text Injection

- [X] T035 [P] [US1] Implement Windows text injection in rust-core/src/platform/windows/text_injection.rs
- [X] T036 [P] [US1] Implement macOS text injection in rust-core/src/platform/macos/text_injection.rs
- [X] T037 [P] [US1] Implement Linux text injection in rust-core/src/platform/linux/text_injection.rs
- [X] T038 [US1] Create cross-platform inject_text FFI function in rust-core/src/ffi/text_injection.rs
- [X] T039 [US1] Create copy_to_clipboard FFI function in rust-core/src/ffi/text_injection.rs

### Rust Core - Window Management

- [X] T040 [P] [US1] Implement Windows floating window in rust-core/src/platform/windows/window.rs
- [X] T041 [P] [US1] Implement macOS floating window in rust-core/src/platform/macos/window.rs
- [X] T042 [P] [US1] Implement Linux floating window in rust-core/src/platform/linux/window.rs
- [X] T043 [US1] Create show_floating_capsule FFI function in rust-core/src/ffi/window.rs
- [X] T044 [US1] Create hide_floating_capsule FFI function in rust-core/src/ffi/window.rs
- [X] T045 [US1] Create set_capsule_state FFI function in rust-core/src/ffi/window.rs

### Flutter UI - Floating Capsule

- [X] T046 [US1] Create FloatingCapsule widget in flutter-ui/lib/widgets/floating_capsule.dart
- [X] T047 [US1] Create AudioVisualizer widget in flutter-ui/lib/widgets/audio_visualizer.dart
- [X] T048 [US1] Implement capsule state transitions with animations in flutter-ui/lib/widgets/floating_capsule.dart
- [X] T049 [US1] Create session state Riverpod providers in flutter-ui/lib/state/providers.dart
- [X] T050 [US1] Wire hotkey to show/hide capsule in flutter-ui/lib/services/shortcuts.dart
- [X] T051 [US1] Connect audio level stream to visualizer in flutter-ui/lib/widgets/audio_visualizer.dart

### Integration

- [X] T052 [US1] Wire recording flow: hotkey → capsule → recording → processing → injection
- [X] T053 [US1] Implement error handling with retry/dismiss in floating capsule
- [ ] T054 [US1] Test end-to-end voice input flow on each platform

---

## Phase 4: User Story 2 - System Tray Status Management (P1)

**Goal**: Tray icon shows application state (idle, recording, processing, error)
**Independent Test**: Observe tray icon changes through session states

### Rust Core - System Tray

- [X] T055 [P] [US2] Implement Windows tray in rust-core/src/platform/windows/tray.rs
- [X] T056 [P] [US2] Implement macOS tray in rust-core/src/platform/macos/tray.rs
- [X] T057 [P] [US2] Implement Linux tray in rust-core/src/platform/linux/tray.rs
- [X] T058 [US2] Create TrayManager singleton in rust-core/src/platform/tray.rs
- [X] T059 [US2] Create set_tray_icon FFI function in rust-core/src/ffi/tray.rs
- [X] T060 [US2] Create set_tray_menu FFI function in rust-core/src/ffi/tray.rs
- [X] T061 [US2] Add tray icon assets (idle, recording, processing, error) to rust-core/assets/icons/

### Flutter Integration

- [X] T062 [US2] Initialize tray on app startup in flutter-ui/lib/main.dart
- [X] T063 [US2] Subscribe to session state and update tray icon in flutter-ui/lib/services/tray_service.dart
- [ ] T064 [US2] Test tray icon updates across all session states

---

## Phase 5: User Story 3 - Push-to-Talk vs Toggle Mode (P1)

**Goal**: Support both push-to-talk and toggle input modes
**Independent Test**: Switch modes in settings, verify each mode's behavior

### Rust Core - Hotkey Management

- [X] T065 [P] [US3] Implement Windows hotkey in rust-core/src/platform/windows/hotkey.rs
- [X] T066 [P] [US3] Implement macOS hotkey in rust-core/src/platform/macos/hotkey.rs
- [X] T067 [P] [US3] Implement Linux hotkey in rust-core/src/platform/linux/hotkey.rs
- [X] T068 [US3] Create HotkeyManager with press/release event handling in rust-core/src/platform/hotkey.rs
- [X] T069 [US3] Create register_hotkey FFI function in rust-core/src/ffi/hotkey.rs
- [X] T070 [US3] Create hotkey_event_stream FFI function in rust-core/src/ffi/hotkey.rs

### Flutter Integration

- [X] T071 [US3] Create InputMode enum in flutter-ui/lib/models/preferences.dart
- [X] T072 [US3] Subscribe to hotkey events in flutter-ui/lib/services/shortcuts.dart
- [X] T073 [US3] Implement push-to-talk logic (press starts, release stops) in flutter-ui/lib/services/shortcuts.dart
- [X] T074 [US3] Implement toggle logic (press toggles, idle timeout stops) in flutter-ui/lib/services/shortcuts.dart
- [X] T075 [US3] Add input_mode preference with default push_to_talk in flutter-ui/lib/services/config_service.dart
- [ ] T076 [US3] Test both modes work correctly with hotkey

---

## Phase 6: User Story 4 - System Tray Quick Menu (P2)

**Goal**: Right-click tray shows quota, hotkey, idle timeout slider, settings/history access
**Independent Test**: Right-click tray, verify menu items and slider functionality

- [X] T077 [US4] Create TrayMenuItem Dart class in flutter-ui/lib/models/tray_menu_item.dart
- [X] T078 [US4] Build tray menu items list with quota display in flutter-ui/lib/services/tray_service.dart
- [X] T079 [US4] Add idle timeout slider to tray menu in flutter-ui/lib/services/tray_service.dart
- [X] T080 [US4] Wire Settings menu item to open settings screen
- [X] T081 [US4] Wire History menu item to open history screen
- [ ] T082 [US4] Test tray menu displays correctly and actions work

---

## Phase 7: User Story 5 - Settings Interface (P2)

**Goal**: Comprehensive settings with grouped configuration (Voice, AI, Language, Dictionary, Shortcuts, Privacy)
**Independent Test**: Open settings, verify all groups, test persistence

### Tests (TDD: Write First)

- [X] T161 [P] [US5] Write unit tests for preferences repository in rust-core/tests/unit/preferences_test.rs
- [X] T166 [P] [US5] Write widget tests for SettingsScreen in flutter-ui/test/screens/settings_screen_test.dart

### Flutter UI - Settings Screen

- [X] T083 [US5] Create SettingsScreen scaffold in flutter-ui/lib/screens/settings_screen.dart
- [X] T084 [US5] Create General settings section (auto-process, filler removal, context-aware) in flutter-ui/lib/screens/settings_screen.dart
- [X] T085 [US5] Create Voice settings section (input mode, microphone, noise cancellation) in flutter-ui/lib/screens/settings_screen.dart
- [X] T086 [US5] Create Language settings section (input language, output language, translation toggle) in flutter-ui/lib/screens/settings_screen.dart
- [X] T087 [US5] Create Data settings section (export, clear, retention) in flutter-ui/lib/screens/settings_screen.dart
- [X] T088 [US5] Create Privacy settings section (crash reporting, analytics) in flutter-ui/lib/screens/settings_screen.dart
- [X] T089 [US5] Create About section (version, privacy policy, terms) in flutter-ui/lib/screens/settings_screen.dart

### FFI Integration

- [X] T090 [US5] Create get_preference FFI function in rust-core/src/ffi/preferences.rs
- [X] T091 [US5] Create set_preference FFI function in rust-core/src/ffi/preferences.rs
- [X] T092 [US5] Create get_all_preferences FFI function in rust-core/src/ffi/preferences.rs
- [X] T093 [US5] Wire each settings toggle to ConfigService in flutter-ui/lib/screens/settings_screen.dart
- [ ] T094 [US5] Test settings persist between app restarts

---

## Phase 8: User Story 6 - Custom Dictionary Management (P2)

**Goal**: Add/edit/delete dictionary entries for improved transcription accuracy
**Independent Test**: Add entry, speak term, verify substitution applied

### Tests (TDD: Write First)

- [X] T162 [P] [US6] Write unit tests for dictionary repository in rust-core/tests/unit/dictionary_test.rs
- [X] T168 [P] [US6] Write widget tests for DictionaryScreen in flutter-ui/test/screens/dictionary_screen_test.dart

### Rust Core - Dictionary

- [X] T095 [US6] Create list_dictionary_entries FFI function in rust-core/src/ffi/dictionary.rs
- [X] T096 [US6] Create add_dictionary_entry FFI function in rust-core/src/ffi/dictionary.rs
- [X] T097 [US6] Create update_dictionary_entry FFI function in rust-core/src/ffi/dictionary.rs
- [X] T098 [US6] Create delete_dictionary_entry FFI function in rust-core/src/ffi/dictionary.rs
- [X] T099 [US6] Integrate dictionary lookup into transcription pipeline in rust-core/src/processing/mod.rs

### Flutter UI - Dictionary Screen

- [X] T100 [US6] Create DictionaryScreen in flutter-ui/lib/screens/dictionary_screen.dart
- [X] T101 [US6] Create dictionary entry list with two-column format in flutter-ui/lib/screens/dictionary_screen.dart
- [X] T102 [US6] Create add/edit dictionary entry dialog in flutter-ui/lib/widgets/dictionary_entry_dialog.dart
- [X] T103 [US6] Wire dictionary screen to settings navigation
- [ ] T104 [US6] Test dictionary CRUD operations and transcription integration

---

## Phase 9: User Story 7 - History Recording and Review (P2)

**Goal**: Access transcription history with timestamps, app context, copy functionality
**Independent Test**: Make transcriptions, verify history entries, test copy

### Tests (TDD: Write First)

- [X] T163 [P] [US7] Write unit tests for history repository in rust-core/tests/unit/history_test.rs
- [X] T167 [P] [US7] Write widget tests for HistoryScreen in flutter-ui/test/screens/history_screen_test.dart

### Rust Core - History

- [X] T105 [US7] Create list_history FFI function in rust-core/src/ffi/history.rs
- [X] T106 [US7] Create list_history_by_date FFI function in rust-core/src/ffi/history.rs
- [X] T107 [US7] Create delete_history_entry FFI function in rust-core/src/ffi/history.rs
- [X] T108 [US7] Create clear_all_history FFI function in rust-core/src/ffi/history.rs
- [X] T109 [US7] Auto-save transcription to history on completion in rust-core/src/state/manager.rs
- [X] T110 [US7] Implement run_cleanup FFI function for retention policy in rust-core/src/ffi/cleanup.rs

### Flutter UI - History Screen

- [X] T111 [US7] Create HistoryScreen in flutter-ui/lib/screens/history_screen.dart
- [X] T112 [US7] Create history entry list grouped by date in flutter-ui/lib/screens/history_screen.dart
- [X] T113 [US7] Create history entry tile with copy button in flutter-ui/lib/widgets/history_entry_tile.dart
- [X] T114 [US7] Implement history deletion and clear all in flutter-ui/lib/screens/history_screen.dart
- [ ] T115 [US7] Test history recording and retrieval

---

## Phase 10: User Story 8 - Language Detection and Translation (P2)

**Goal**: Auto-detect spoken language, optional translation to different output language
**Independent Test**: Speak in different languages, enable translation, verify output

### Rust Core - Language

- [X] T116 [US8] Create Language struct and SUPPORTED_LANGUAGES in rust-core/src/language/mod.rs
- [X] T117 [US8] Integrate Azure Speech language detection in rust-core/src/speech/client.rs
- [X] T118 [US8] Add translation support to translator service in rust-core/src/ai/translator.rs
- [X] T119 [US8] Create get_active_application FFI function in rust-core/src/ffi/context.rs

### Flutter UI - Language Selector

- [X] T120 [US8] Create Language model in flutter-ui/lib/models/language.dart
- [X] T121 [US8] Create LanguageSelector widget in flutter-ui/lib/widgets/language_selector.dart
- [X] T122 [US8] Add translation toggle to settings language section
- [X] T123 [US8] Display detected language in floating capsule
- [ ] T124 [US8] Test language detection and translation end-to-end

---

## Phase 11: User Story 9 - Mobile Keyboard Input Method (P3)

**Goal**: Custom keyboard on iOS/Android with microphone button and voice input
**Independent Test**: Install keyboard, use voice input in various apps

### iOS Keyboard Extension

- [X] T125 [US9] Create iOS keyboard extension target in platform/ios/KeyboardExtension/
- [X] T126 [US9] Create keyboard UI with microphone button in platform/ios/KeyboardExtension/KeyboardViewController.swift
- [ ] T127 [US9] Configure shared container for main app communication
- [ ] T128 [US9] Integrate Rust FFI bridge into keyboard extension
- [ ] T129 [US9] Implement recording and text insertion flow for iOS keyboard

### Android IME

- [X] T130 [US9] Create Android IME service in platform/android/app/src/main/java/com/talkute/ime/
- [X] T131 [US9] Create keyboard layout XML with microphone button
- [X] T132 [US9] Implement InputMethodService with voice input in platform/android/app/src/main/java/com/talkute/ime/TalkuteIME.kt
- [ ] T133 [US9] Integrate Rust FFI bridge into IME service
- [ ] T134 [US9] Implement recording and text insertion flow for Android keyboard

### Shared Mobile Components

- [X] T135 [US9] Create MobileKeyboardWidget in flutter-ui/lib/widgets/mobile_keyboard.dart
- [X] T136 [US9] Create mobile audio visualizer in flutter-ui/lib/widgets/mobile_audio_visualizer.dart
- [ ] T137 [US9] Test keyboard installation and voice input on both platforms

---

## Phase 12: User Story 10 - AI Polishing Intensity Selection (P3)

**Goal**: Three intensity levels (Light, Standard, Deep) for user control over AI modification
**Independent Test**: Speak same phrase with each level, verify different output

### Rust Core - Polishing Intensity

- [X] T138 [US10] Create PolishingIntensity enum in rust-core/src/ai/polisher.rs
- [X] T139 [US10] Implement Light polishing (filler removal only) in rust-core/src/ai/polisher.rs
- [X] T140 [US10] Implement Standard polishing (grammar + logic) in rust-core/src/ai/polisher.rs
- [X] T141 [US10] Implement Deep polishing (full rewrite) in rust-core/src/ai/polisher.rs
- [X] T142 [US10] Wire intensity preference to polishing pipeline

### Flutter UI - Intensity Selector

- [X] T143 [US10] Create PolishingIntensitySelector widget in flutter-ui/lib/widgets/intensity_selector.dart
- [X] T144 [US10] Add intensity selector to settings AI section
- [X] T145 [US10] Add quick intensity toggle to floating capsule
- [ ] T146 [US10] Test all three intensity levels produce different output

---

## Phase 13: Polish & Cross-Cutting Concerns

**Goal**: Error handling, performance optimization, accessibility, E2E testing

### Error Handling

- [X] T147 Implement comprehensive error messages in floating capsule for all error states
- [X] T148 Add offline detection and cached retry queue in rust-core/src/network/offline_handler.rs
- [ ] T149 Implement clipboard fallback UI when text injection fails
- [ ] T150 Add network error recovery with retry button

### Data Export/Import

- [X] T151 [P] Create export_data FFI function in rust-core/src/ffi/export.rs
- [X] T152 [P] Create import_dictionary FFI function in rust-core/src/ffi/export.rs
- [X] T153 Implement data export UI in settings
- [X] T154 Implement dictionary import UI in settings

### Performance

- [ ] T155 Profile and optimize context detection to <50ms p95
- [ ] T156 Profile and optimize end-to-end latency to <3s
- [ ] T157 Verify 60fps UI rendering during recording
- [ ] T158 Verify memory usage <100MB idle, <300MB active
- [ ] T159 Verify CPU usage <5% idle, <30% processing

### Testing

- [ ] T169 Write E2E test for complete voice input flow

### Documentation

- [X] T170 Update CLAUDE.md with new platform modules
- [ ] T171 Document platform-specific setup requirements (permissions, etc.)
- [ ] T172 Document mobile keyboard installation instructions

### Accessibility Verification (FR-025)

- [ ] T173 [P] Verify floating capsule respects system accessibility settings (high contrast, screen reader) on Windows
- [ ] T174 [P] Verify floating capsule respects system accessibility settings on macOS
- [ ] T175 [P] Verify floating capsule respects system accessibility settings on Linux

### Quota Management (FR-026)

- [X] T176 Implement quota tracking in rust-core/src/storage/quota.rs
- [X] T177 Add grace period logic (10% overage with warning) in rust-core/src/storage/quota.rs
- [ ] T178 Show quota warning notification when entering grace period
- [ ] T179 Show hard block UI with upgrade prompt when quota fully exceeded

---

## Dependencies

### Story Dependencies (Blocking Order)

```
Phase 1 (Setup) ──► Phase 2 (Foundational)
                           │
                           ▼
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
      US1 (Capsule)    US2 (Tray)      US3 (Hotkey)
         │                 │                 │
         └────────┬────────┴────────┬────────┘
                  │                 │
                  ▼                 ▼
              US4 (Tray Menu)  US5 (Settings)
                  │                 │
                  ▼                 ▼
             US6 (Dictionary) ◄────┤
                  │                 │
                  ▼                 ▼
             US7 (History)    US8 (Language)
                  │                 │
                  └────────┬────────┘
                           │
                           ▼
                    US9 (Mobile) ──► US10 (Polishing)
                           │
                           ▼
                    Phase 13 (Polish)
```

### Independent Stories (Can Run in Parallel)
- US1, US2, US3 can start after Phase 2 (no interdependencies)
- US4 requires US2 (tray)
- US5 is independent after Phase 2
- US6, US7, US8 can run in parallel after Phase 2
- US9 is independent (mobile platform)
- US10 requires US1 (polishing used in capsule flow)

---

## Parallel Execution Examples

### After Phase 2 (Maximum Parallelization)
```bash
# Run these user stories in parallel (different developers):
Developer A: Phase 3 (US1 - Floating Capsule)
Developer B: Phase 4 (US2 - System Tray)
Developer C: Phase 5 (US3 - Hotkey Modes)
```

### Within User Stories
```bash
# Phase 3 (US1) - Platform implementations can run in parallel:
T035 [P] Windows text injection
T036 [P] macOS text injection
T037 [P] Linux text injection

T040 [P] Windows floating window
T041 [P] macOS floating window
T042 [P] Linux floating window
```

### Phase 2 Foundational Tasks
```bash
# Storage repositories can run in parallel:
T015-T016 Preferences repository
T017-T018 Dictionary repository
T019-T020 History repository

# Flutter services can run in parallel:
T027 EngineService
T028 ConfigService
T029 TrayService
T030 ShortcutsService
```

---

## Verification Checklist

After completing all phases, verify:

- [ ] Floating capsule appears on hotkey, shows waveform, inserts text
- [ ] System tray icon reflects session state correctly
- [ ] Push-to-talk and toggle modes both work
- [ ] Tray right-click menu shows quota and settings access
- [ ] Settings screen has all groups and persists correctly
- [ ] Dictionary CRUD works and affects transcription
- [ ] History records and displays transcriptions by date
- [ ] Language detection and translation work
- [ ] Mobile keyboard installs and works on iOS/Android
- [ ] Three polishing intensity levels produce different output
- [ ] All error states show retry/dismiss options
- [ ] Performance targets met (<3s end-to-end, <100MB idle)
- [ ] All tests passing (Rust unit, Rust integration, Flutter widget, E2E)
