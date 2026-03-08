# Quickstart Guide: Product Experience Design Implementation

**Feature**: 002-product-experience-ux
**Date**: 2026-03-07

## Overview

This guide provides step-by-step instructions for integrating the Product Experience Design components into the Talkute application. Follow these integration scenarios to implement the desktop floating capsule, system tray, settings, and supporting features.

## Prerequisites

Before starting, ensure you have:
- Rust 1.75+ with cargo
- Flutter 3.16+ with Dart 3.0+
- flutter_rust_bridge v2.11+ configured
- Platform-specific development tools (Visual Studio for Windows, Xcode for macOS)

## Integration Scenarios

### Scenario 1: Desktop Floating Capsule

**Goal**: Implement the floating voice input capsule with four states.

#### Step 1: Create Floating Capsule Widget

Create `flutter-ui/lib/widgets/floating_capsule.dart`:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../state/providers.dart';
import 'audio_visualizer.dart';

class FloatingCapsule extends ConsumerWidget {
  const FloatingCapsule({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final sessionState = ref.watch(sessionStateProvider);

    return AnimatedContainer(
      duration: const Duration(milliseconds: 200),
      width: _getWidth(sessionState),
      height: 64,
      decoration: BoxDecoration(
        color: _getBackgroundColor(sessionState),
        borderRadius: BorderRadius.circular(32),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.2),
            blurRadius: 16,
          ),
        ],
      ),
      child: _buildContent(sessionState, ref),
    );
  }

  Widget _buildContent(SessionState state, WidgetRef ref) {
    switch (state) {
      case SessionState.idle:
        return const _IdleContent();
      case SessionState.recording:
        return const _RecordingContent();
      case SessionState.processing:
        return const _ProcessingContent();
      case SessionState.error:
        return _ErrorContent(ref.read(sessionErrorProvider));
    }
  }

  double _getWidth(SessionState state) {
    switch (state) {
      case SessionState.idle:
      case SessionState.recording:
        return 200;
      case SessionState.processing:
        return 160;
      case SessionState.error:
        return 280;
    }
  }

  Color _getBackgroundColor(SessionState state) {
    switch (state) {
      case SessionState.idle:
        return Colors.grey[800]!;
      case SessionState.recording:
        return Colors.red[400]!;
      case SessionState.processing:
        return Colors.blue[400]!;
      case SessionState.error:
        return Colors.orange[400]!;
    }
  }
}
```

#### Step 2: Register Window Platform Adapter

Add to `rust-core/src/platform/window.rs`:

```rust
use flutter_rust_bridge::frb;

#[frb]
pub fn show_floating_capsule() -> Result<(), String> {
    // Platform-specific implementation
    #[cfg(target_os = "windows")]
    {
        crate::platform::windows::show_floating_window()
    }
    #[cfg(target_os = "macos")]
    {
        crate::platform::macos::show_floating_window()
    }
    #[cfg(target_os = "linux")]
    {
        crate::platform::linux::show_floating_window()
    }
}
```

#### Step 3: Wire State Management

Add to `flutter-ui/lib/state/providers.dart`:

```dart
final sessionStateProvider = StateProvider<SessionState>((ref) {
  return SessionState.idle;
});

final sessionErrorProvider = StateProvider<String?>((ref) => null);

// Listen to Rust state stream
final sessionStateStreamProvider = StreamProvider<SessionStateEvent>((ref) {
  return RustLib.api.sessionStateStream();
});
```

---

### Scenario 2: System Tray Integration

**Goal**: Add system tray with status icons and quick menu.

#### Step 1: Create Tray Service

Create `rust-core/src/platform/tray.rs`:

```rust
use tray_item::{TrayItem, IconSource};
use flutter_rust_bridge::frb;

pub struct TrayManager {
    tray: Option<TrayItem>,
}

impl TrayManager {
    pub fn new() -> Self {
        Self { tray: None }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        let tray = TrayItem::new(
            "Talkute",
            IconSource::Resource("icons/tray-idle"),
        ).map_err(|e| e.to_string())?;

        self.tray = Some(tray);
        Ok(())
    }

    pub fn set_icon(&mut self, state: &str) -> Result<(), String> {
        let icon_name = match state {
            "recording" => "icons/tray-recording",
            "processing" => "icons/tray-processing",
            "error" => "icons/tray-error",
            _ => "icons/tray-idle",
        };

        if let Some(tray) = &mut self.tray {
            tray.set_icon(IconSource::Resource(icon_name))
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

#[frb]
pub fn init_tray() -> Result<(), String> {
    TRAY_MANAGER.lock().unwrap().initialize()
}

#[frb]
pub fn set_tray_icon(state: String) -> Result<(), String> {
    TRAY_MANAGER.lock().unwrap().set_icon(&state)
}
```

#### Step 2: Add Tray Menu Items

```rust
pub fn add_menu_items(&mut self) -> Result<(), String> {
    let tray = self.tray.as_mut().ok_or("Tray not initialized")?;

    tray.add_label("Today's Usage: 0/100");

    tray.add_separator();

    tray.add_menu_item("Settings", || {
        // Open settings window
    });

    tray.add_menu_item("History", || {
        // Open history window
    });

    tray.add_separator();

    tray.add_menu_item("Quit", || {
        std::process::exit(0);
    });

    Ok(())
}
```

---

### Scenario 3: Global Hotkey Registration

**Goal**: Register and handle global hotkey for voice input.

#### Step 1: Create Hotkey Handler

Create `rust-core/src/platform/hotkey.rs`:

```rust
use global_hotkey::{GlobalHotKey, GlobalHotKeyEvent, HotKeyState};
use flutter_rust_bridge::{frb, StreamSink};
use std::sync::atomic::{AtomicBool, Ordering};

static HOTKEY_REGISTERED: AtomicBool = AtomicBool::new(false);

#[frb]
pub fn register_hotkey(hotkey: String) -> Result<(), String> {
    let hotkey = parse_hotkey(&hotkey)?;

    if HOTKEY_REGISTERED.load(Ordering::SeqCst) {
        unregister_hotkey()?;
    }

    hotkey.register().map_err(|e| e.to_string())?;
    HOTKEY_REGISTERED.store(true, Ordering::SeqCst);

    Ok(())
}

#[frb]
pub fn hotkey_event_stream(sink: StreamSink<HotkeyEvent>) {
    let receiver = GlobalHotKeyEvent::receiver();

    tokio::spawn(async move {
        while let Ok(event) = receiver.recv() {
            if event.state == HotKeyState::Pressed {
                sink.add(HotkeyEvent {
                    hotkey: format!("{:?}", event.hotkey),
                    action: "pressed".to_string(),
                });
            } else if event.state == HotKeyState::Released {
                sink.add(HotkeyEvent {
                    hotkey: format!("{:?}", event.hotkey),
                    action: "released".to_string(),
                });
            }
        }
    });
}

fn parse_hotkey(s: &str) -> Result<GlobalHotKey, String> {
    // Parse "Ctrl+Shift+Space" format
    let parts: Vec<&str> = s.split('+').collect();
    // ... parsing logic
}
```

#### Step 2: Handle Hotkey in Flutter

Add to `flutter-ui/lib/services/shortcuts.dart`:

```dart
class HotkeyService {
  final Ref _ref;

  HotkeyService(this._ref) {
    _listenToHotkey();
  }

  void _listenToHotkey() {
    RustLib.api.hotkeyEventStream().listen((event) {
      final inputMode = _ref.read(inputModeProvider);

      if (event.action == 'pressed') {
        if (inputMode == InputMode.pushToTalk) {
          _startRecording();
        } else {
          _toggleRecording();
        }
      } else if (event.action == 'released') {
        if (inputMode == InputMode.pushToTalk) {
          _stopRecording();
        }
      }
    });
  }

  void _startRecording() async {
    await RustLib.api.startRecording();
    _ref.read(sessionStateProvider.notifier).state = SessionState.recording;
  }

  void _stopRecording() async {
    final sessionId = _ref.read(sessionIdProvider);
    await RustLib.api.stopRecording(sessionId!);
    _ref.read(sessionStateProvider.notifier).state = SessionState.processing;
  }
}
```

---

### Scenario 4: Settings Screen

**Goal**: Implement comprehensive settings interface.

#### Step 1: Create Settings Screen

Create `flutter-ui/lib/screens/settings_screen.dart`:

```dart
class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: ListView(
        children: const [
          _GeneralSection(),
          _VoiceSection(),
          _LanguageSection(),
          _DataSection(),
          _PrivacySection(),
          _AboutSection(),
        ],
      ),
    );
  }
}

class _GeneralSection extends ConsumerWidget {
  const _GeneralSection();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return _SettingsSection(
      title: 'General',
      children: [
        _SettingsTile<bool>(
          title: 'Auto-process',
          subtitle: 'Automatically process after recording',
          provider: autoProcessProvider,
          onToggle: (value) => _setPreference('auto_process', value),
        ),
        _SettingsTile<bool>(
          title: 'Remove fillers',
          subtitle: 'Remove filler words from transcription',
          provider: fillerRemovalProvider,
          onToggle: (value) => _setPreference('filler_removal', value),
        ),
      ],
    );
  }

  Future<void> _setPreference(String key, bool value) async {
    await RustLib.api.setPreference(key: key, value: jsonEncode(value));
  }
}
```

---

### Scenario 5: History Screen

**Goal**: Display and manage transcription history.

#### Step 1: Create History Screen

Create `flutter-ui/lib/screens/history_screen.dart`:

```dart
class HistoryScreen extends ConsumerStatefulWidget {
  const HistoryScreen({super.key});

  @override
  ConsumerState<HistoryScreen> createState() => _HistoryScreenState();
}

class _HistoryScreenState extends ConsumerState<HistoryScreen> {
  List<HistoryEntry> _entries = [];
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadHistory();
  }

  Future<void> _loadHistory() async {
    final entries = await RustLib.api.listHistory(limit: 100, offset: 0);
    setState(() {
      _entries = entries;
      _isLoading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return const Scaffold(body: Center(child: CircularProgressIndicator()));
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('History'),
        actions: [
          IconButton(
            icon: const Icon(Icons.delete_sweep),
            onPressed: _clearAllHistory,
          ),
        ],
      ),
      body: ListView.builder(
        itemCount: _entries.length,
        itemBuilder: (context, index) {
          final entry = _entries[index];
          return _HistoryEntryTile(entry: entry);
        },
      ),
    );
  }
}
```

---

### Scenario 6: Dictionary Management

**Goal**: Allow users to add custom word mappings.

#### Step 1: Create Dictionary Service

```dart
class DictionaryService {
  Future<List<DictionaryEntry>> loadEntries() async {
    return await RustLib.api.listDictionaryEntries();
  }

  Future<void> addEntry(String voiceForm, String standardForm) async {
    await RustLib.api.addDictionaryEntry(
      voiceForm: voiceForm,
      standardForm: standardForm,
    );
  }

  Future<void> deleteEntry(int id) async {
    await RustLib.api.deleteDictionaryEntry(id: id);
  }
}
```

---

## Testing Checklist

After integration, verify:

- [ ] Floating capsule appears on hotkey press
- [ ] Capsule shows all four states correctly
- [ ] System tray icon updates with session state
- [ ] Right-click tray menu shows all options
- [ ] Hotkey triggers recording in both modes
- [ ] Settings persist between app restarts
- [ ] History entries are recorded and viewable
- [ ] Dictionary entries affect transcription
- [ ] Text injection works in target applications
- [ ] Error states show retry/dismiss buttons

## Common Issues

### Hotkey not registering
- Check if another application has registered the same hotkey
- Try a different hotkey combination
- Ensure app has accessibility permissions (macOS)

### Text injection fails
- Check if target app has focus
- Verify accessibility permissions (macOS/Linux)
- Try clipboard fallback

### Tray icon not showing
- Check system tray settings (some Linux DEs hide tray)
- Verify icon files exist in resources

## Next Steps

After completing these integration scenarios:

1. Run unit tests for each component
2. Perform integration testing across platforms
3. Validate accessibility (system-level)
4. Test error handling and edge cases
5. Optimize performance (latency targets)
