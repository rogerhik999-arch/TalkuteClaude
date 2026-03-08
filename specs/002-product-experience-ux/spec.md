# Feature Specification: Product Experience Design Implementation

**Feature Branch**: `002-product-experience-ux`
**Created**: 2026-03-06
**Status**: Draft
**Input**: Based on docs/Talkute_Product_Experience_Design.md - Comprehensive UX/UI design specification for Talkute voice-to-text application

## User Scenarios & Testing

### User Story 1 - Desktop Voice Input with Floating Capsule (Priority: P1)

As a desktop user (Windows/macOS/Linux), I want to press a global hotkey and have a minimal floating capsule appear on screen center, speak naturally, and have my words automatically transcribed, AI-polished, and inserted at my cursor position without any additional clicks or windows.

**Why this priority**: This is the core desktop interaction that defines the Talkute's "invisible assistant" experience. Without this, the desktop product has no primary interface.

**Independent Test**: Can be fully tested by pressing the global hotkey, speaking text, and verifying the polished text appears at the cursor position. Delivers immediate value without any other features.

**Acceptance Scenarios**:

1. **Given** user is in any application with a text field, **When** user presses `Ctrl+Shift+Space` (or platform equivalent), **Then** floating capsule appears at screen center with recording indicator
2. **Given** capsule is visible and recording, **When** user speaks natural language, **Then** real-time audio waveform visualization shows in capsule
3. **Given** user stops speaking for idle timeout (1.5s default), **When** AI processing completes, **Then** polished text is automatically inserted at cursor position and capsule shows success state then disappears

---

### User Story 2 - System Tray Status Management (Priority: P1)

As a desktop user, I want to see Talkute's status in the system tray at all times, so I can quickly verify if it application is running and what state it is in (idle, recording, processing, error) without opening any windows.

**Why this priority**: The system tray is the persistent anchor point for the desktop experience. Users need to know Talkute is active and responsive without intrusive UI.

**Independent Test**: Can be tested by observing tray icon changes through different states (idle → recording → processing → done). Delivers persistent presence indicator.

**Acceptance Scenarios**:

1. **Given** Talkute is running, **When** user looks at system tray, **Then** Talkute icon is visible with idle state indicator
2. **Given** user starts recording via hotkey, **When** recording begins, **Then** tray icon shows recording state (red pulse)
3. **Given** AI is processing text, **When** processing is active, **Then** tray icon shows processing state (spinning indicator)
4. **Given** an error occurs, **When** error state is active, **Then** tray icon shows error state (yellow warning)

---

### User Story 3 - Push-to-Talk vs Toggle Mode (Priority: P1)

As a user, I want to choose between two input modes - hold-to-record (push-to-talk) or click-to-toggle - so I can use the input style that fits my workflow (short phrases vs long dictation).

**Why this priority**: Different users have different speaking patterns. Push-to-talk suits quick phrases; toggle mode suits longer mon More detailed content. This choice is fundamental to the core experience.

**Independent Test**: Can be tested by switching between modes in settings and verifying each mode's recording behavior works correctly. Delivers flexibility for different user preferences.

**Acceptance Scenarios**:

1. **Given** push-to-talk mode is enabled, **When** user holds hotkey down, **Then** recording starts immediately
2. **Given** push-to-talk mode is enabled and recording is active, **When** user releases hotkey, **Then** recording stops and AI processing begins
3. **Given** toggle mode is enabled, **When** user presses hotkey once, **Then** recording starts and continues until second press or idle timeout
4. **Given** toggle mode is enabled and recording is active, **When** user presses hotkey again, **Then** recording stops

---

### User Story 4 - System Tray Quick Menu (Priority: P2)

As a desktop user, I want to right-click the system tray icon to see a quick menu with today's usage quota, current hotkey, idle timeout setting, and quick access to settings and history, so I can check status and adjust key settings without opening full windows.

**Why this priority**: Provides convenient access to frequently-used settings and information without disrupting workflow. The idle timeout slider is critical for user experience tuning.

**Independent Test**: Can be tested by right-clicking tray icon and verifying all menu items are accessible and functional. Delivers quick access to key controls.

**Acceptance Scenarios**:

1. **Given** user right-clicks tray icon, **When** menu appears, **Then** today's usage quota is displayed with progress bar
2. **Given** tray menu is open, **When** user adjusts idle timeout slider, **Then** value updates in real-time with preview
3. **Given** tray menu is open, **When** user clicks Settings, **Then** settings window opens
4. **Given** tray menu is open, **When** user clicks History, **Then** history window opens

---

### User Story 5 - Settings Interface (Priority: P2)

As a user, I want to access a comprehensive settings interface organized by functional groups (Voice Input, AI Polishing, Language/Translation, Custom Dictionary, Shortcuts, Privacy), so I can configure all aspects of Talkute to one place.

**Why this priority**: Centralized configuration is essential for user customization. Grouped layout makes finding settings intuitive. Required for users to personalize their experience.

**Independent Test**: Can be tested by opening settings and verifying each group contains expected controls and all settings persist correctly. Delivers complete configuration capability.

**Acceptance Scenarios**:

1. **Given** user opens Settings, **When** settings window appears, **Then** all setting groups are visible (Voice Input, AI Polishing, Language, Dictionary, Shortcuts, Privacy, About)
2. **Given** settings is open, **When** user changes input mode, **Then** setting is saved and immediately active
3. **Given** settings is open, **When** user adjusts idle timeout, **Then** slider shows real-time value preview
4. **Given** settings is open, **When** user enables/disables noise cancellation, **Then** setting is saved

---

### User Story 6 - Custom Dictionary Management (Priority: P2)

As a user, I want to maintain a personal dictionary of voice-to-text mappings (nicknames, technical terms, abbreviations), so that transcription accuracy improves for words I frequently use that ASR might misinterpret.

**Why this priority**: Personal dictionary significantly improves transcription accuracy for domain-specific terms and names. Users expect voice recognition to learn their vocabulary.

**Independent Test**: Can be tested by adding/editing/deleting dictionary entries and verifying they improve transcription of those terms. Delivers personalized accuracy improvement.

**Acceptance Scenarios**:

1. **Given** user opens Dictionary settings, **When** dictionary interface appears, **Then** existing entries are shown in two-column format (Voice/Nickname → Standard Term)
2. **Given** dictionary interface is open, **When** user adds new entry "泰普勒斯 → Talkute", **Then** entry is saved and appears in list
3. **Given** dictionary has existing entry, **When** user edits the entry, **Then** changes are saved
4. **Given** dictionary interface is open, **When** user deletes an entry, **Then** entry is removed from list
5. **Given** dictionary has entries, **When** user speaks a dictionary term, **Then** transcription uses the standard term

---

### User Story 7 - History Recording and Review (Priority: P2)

As a user, I want to access a history of my transcriptions with timestamps, target applications, and polished text, so I can review past work and copy previously transcribed text.

**Why this priority**: History provides valuable reference and recovery capability. Users may need to retrieve previously transcribed content or review their usage patterns.

**Independent Test**: Can be tested by making several transcriptions and verifying they all appear in history with correct metadata. Delivers reference and recovery capability.

**Acceptance Scenarios**:

1. **Given** user makes a transcription, **When** transcription completes, **Then** entry is added to history with timestamp, app context, and text
2. **Given** history has multiple entries, **When** user opens history, **Then** entries are grouped by date (Today, Yesterday, etc.)
3. **Given** history is open, **When** user clicks copy on an entry, **Then** text is copied to clipboard
4. **Given** history is open, **When** user deletes an entry, **Then** entry is removed from history

---

### User Story 8 - Language Detection and Translation (Priority: P2)

As a user, I want Talkute to automatically detect my spoken language or manually specify it and optionally translate to a different output language, so I can speak in one language and output text in another.

**Why this priority**: Multi-language support is essential for international users. Automatic detection reduces friction; manual override provides control. Translation enables cross-language workflows.

**Independent Test**: Can be tested by speaking in different languages and verifying correct detection, then enabling translation and verifying output language. Delivers multi-language capability.

**Acceptance Scenarios**:

1. **Given** language is set to auto-detect, **When** user speaks in Chinese, **Then** system detects Chinese and displays "中文" in language indicator
2. **Given** language is set to auto-detect, **When** user speaks in English, **Then** system detects English and displays "English" in language indicator
3. **Given** translation mode is enabled with Chinese → English, **When** user speaks Chinese, **Then** output text is in English
4. **Given** translation mode is off, **When** user speaks any language, **Then** output is in same language as input

---

### User Story 9 - Mobile Keyboard Input Method (Priority: P3)

As a mobile user (iOS/Android), I want Talkute to appear as a custom keyboard with a prominent microphone button, real-time audio visualization, and quick access to settings, so I can use voice input in any app that accepts text.

**Why this priority**: Mobile is a secondary platform after desktop. Keyboard input method is the only way to provide system-wide voice input on mobile. Requires platform-specific implementation (iOS Extension/Android IME).

**Independent Test**: Can be tested by installing keyboard and using voice input in various apps. Delivers mobile voice input capability.

**Acceptance Scenarios**:

1. **Given** Talkute keyboard is installed, **When** user selects Talkute as input method, **Then** Talkute keyboard appears with microphone button and tool bar
2. **Given** Talkute keyboard is active, **When** user taps microphone button, **Then** recording starts with visual feedback (button changes color, ripple animation)
3. **Given** recording is active on mobile, **When** user speaks, **Then** audio waveform visualization shows real-time response
4. **Given** AI processing completes on mobile, **When** text is ready, **Then** text is inserted into the active text field and success feedback is shown

---

### User Story 10 - AI Polishing Intensity Selection (Priority: P3)

As a user, I want to choose between three polishing intensity levels (Light: remove filler words only, Standard: grammar + logic, Deep: full rewrite), so I can control how much the AI modifies my speech.

**Why this priority**: Different contexts require different levels of formality. Quick messages need light touch; formal documents need deep polishing. User control over AI intervention is important.

**Independent Test**: Can be tested by speaking the same phrase with each intensity level and verifying different output styles. Delivers user control over AI behavior.

**Acceptance Scenarios**:

1. **Given** intensity is set to Light, **When** user speaks with filler words, **Then** output has fillers removed but minimal other changes
2. **Given** intensity is set to Standard, **When** user speaks, **Then** output has corrected grammar and improved logic
3. **Given** intensity is set to Deep, **When** user speaks casually, **Then** output is fully rewritten in formal style

---

### Edge Cases

- **No microphone detected / permission denied**: Capsule displays error message with "Retry" and "Dismiss" buttons; tray icon shows warning state
- **Network connectivity loss during AI processing**: Capsule displays "Network error - Retry?" with retry button; transcription is cached locally for retry when connectivity restored
- **Text injection fails (no active text field, permissions denied)**: Capsule displays "Cannot insert text" with "Copy to clipboard" alternative action button
- **Very long recordings (memory, performance)**: System caps recording at 5 minutes; warning shown at 4 minutes; processing streams to avoid memory issues
- **User switches applications during recording**: Recording continues; text injection targets the active field at time of completion (may differ from recording start)
- **Multiple rapid hotkey presses**: Debounced with 300ms cooldown; second press during recording stops it (toggle mode) or is ignored (push-to-talk mode)

## Requirements

### Functional Requirements

- **FR-001**: System MUST display a floating capsule window at screen center when global hotkey is pressed
- **FR-002**: System MUST show real-time audio waveform visualization during recording
- **FR-003**: System MUST automatically detect when user stops speaking and trigger AI processing after configurable idle timeout
- **FR-004**: System MUST inject polished text at cursor position in the active application
- **FR-005**: System MUST display system tray icon with status indicators (idle, recording, processing, error)
- **FR-006**: System MUST support both push-to-talk and toggle input modes
- **FR-007**: System MUST persist user preferences including input mode, idle timeout, and language settings
- **FR-008**: System MUST provide settings interface with grouped configuration options
- **FR-009**: System MUST allow users to manage custom dictionary entries (add, edit, delete)
- **FR-010**: System MUST record transcription history with timestamp, app context, and text
- **FR-011**: System MUST automatically detect spoken language or allow manual language selection
- **FR-012**: System MUST support optional translation to a different output language
- **FR-013**: System MUST provide three AI polishing intensity levels (Light, Standard, Deep)
- **FR-014**: System MUST show success/error feedback after text injection attempt
- **FR-015**: System MUST provide mobile keyboard input method for iOS and Android
- **FR-016**: System MUST apply custom dictionary substitutions during transcription
- **FR-017**: System MUST provide right-click menu from system tray with quick settings access
- **FR-018**: System MUST display usage quota in tray menu
- **FR-019**: System MUST allow idle timeout adjustment from tray menu slider
- **FR-020**: System MUST support platform-specific visual effects (Windows acrylic, macOS vibrancy, etc.)
- **FR-021**: System MUST display error messages in floating capsule with retry and dismiss action buttons when errors occur
- **FR-022**: System MUST provide clipboard fallback when text injection fails
- **FR-023**: System MUST provide configurable idle timeout with range 0.5s - 5.0s and default of 1.5s
- **FR-024**: System MUST retain transcription history entries for 30 days by default with user-configurable retention options (7 days, 30 days, 90 days, forever, or off)
- **FR-025**: System MUST rely on platform-level accessibility features only (no additional accessibility requirements beyond OS support)
- **FR-026**: System MUST allow 10% grace period overage when daily quota is exceeded with warning, then hard block with upgrade prompt

### Key Entities

- **TranscriptionSession**: Represents a single voice-to-text interaction; contains raw audio, transcription, polished text, timestamp, source application context, language pair
- **UserPreferences**: Contains all user-configurable settings including input mode, idle timeout, polishing intensity, language preferences, dictionary reference
- **DictionaryEntry**: Maps a spoken form (nickname/pronunciation) to a standard written form; belongs to user's custom dictionary
- **HistoryEntry**: Persistent record of completed transcription; contains timestamp, app context, original text, polished text, language pair

## Success Criteria

### Measurable Outcomes

- **SC-001**: Users can complete voice-to-text input in under 3 seconds from end of speech to text insertion
- **SC-002**: System supports daily operation across Windows, macOS, and Linux desktop platforms
- **SC-003**: 95% of transcriptions complete successfully without user intervention after speaking
- **SC-004**: Custom dictionary entries improve transcription accuracy by measurable percentage for domain-specific terms
- **SC-005**: Users can switch between input modes and have changes take effect immediately
- **SC-006**: All settings persist between sessions and survive application updates
- **SC-007**: System tray provides instant visibility into application state without opening windows
- **SC-008**: Mobile keyboard installs successfully and appears in system keyboard selector on both iOS and Android
- **SC-009**: Language auto-detection correctly identifies spoken language in over 90% of cases for supported languages
- **SC-010**: Translation feature produces grammatically correct output in target language for supported language pairs

## Clarifications

### Session 2026-03-07

- Q: When an error occurs (no microphone, network failure, text injection failed), how should the error be presented to the user and what actions can they take? → A: Error message shown in floating capsule with retry/dismiss buttons
- Q: What is the default retention period for transcription history entries? → A: 30 days - balanced retention for typical reference needs
- Q: What is the valid range and default value for the idle timeout setting? → A: Range 0.5s - 5.0s, default 1.5s
- Q: What accessibility features should be included as baseline requirements? → A: Basic only - System accessibility settings, no additional requirements
- Q: What should happen when the user attempts to record but has exceeded their daily quota? → A: Grace period - Allow 10% overage with warning, then hard block with upgrade prompt

## Assumptions

- **Platform availability**: Desktop features (system tray, global hotkey, floating window, text injection) are supported on Windows, macOS, and Linux
- **API availability**: Azure Speech Services API and Anthropic Claude API are available and accessible
- **Language support**: Initial release supports Chinese (Simplified/Traditional), English, Japanese, Korean with capability to extend
- **Audio input**: System has functional microphone with appropriate permissions granted
- **Text injection**: Target applications accept simulated keyboard input for text insertion
- **Storage**: Local SQLite database is available for preferences and history storage
- **Network**: Internet connectivity is required for cloud-based transcription and AI polishing features
- **Mobile scope**: Mobile keyboard implementation is platform-specific (iOS Custom Keyboard Extension, Android InputMethodService) and may have platform limitations

## Out of Scope

The following items are explicitly out of scope for this feature:

- Voice assistant commands (weather, reminders, etc.) - deferred to future version
- Cloud sync of preferences and history across devices
- Team/enterprise features with shared dictionaries
- Voice training/custom model training for specific users
- Offline transcription without cloud services
- Integration with specific applications beyond text injection
- Accessibility features beyond standard platform support
- Multi-user support on same device
