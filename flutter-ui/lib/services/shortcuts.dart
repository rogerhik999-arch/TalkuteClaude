/// Keyboard shortcuts service for Talkute
///
/// Provides global keyboard shortcuts for voice activation and other actions.

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Provider for keyboard shortcuts service
final shortcutsServiceProvider = Provider<ShortcutsService>((ref) {
  return ShortcutsService(ref);
});

/// Keyboard shortcuts configuration
class ShortcutConfig {
  final String name;
  final String description;
  final ShortcutActivator activator;
  final VoidCallback action;

  const ShortcutConfig({
    required this.name,
    required this.description,
    required this.activator,
    required this.action,
  });
}

/// Service for managing keyboard shortcuts
class ShortcutsService {
  final Ref _ref;

  ShortcutsService(this._ref);

  /// Default keyboard shortcuts using SingleActivator
  static ShortcutActivator get defaultVoiceActivation =>
      const SingleActivator(LogicalKeyboardKey.space, control: true);

  static ShortcutActivator get defaultToggleTranslation =>
      const SingleActivator(LogicalKeyboardKey.keyT, control: true);

  static ShortcutActivator get defaultToggleAutoProcess =>
      const SingleActivator(LogicalKeyboardKey.keyP, control: true);

  static ShortcutActivator get defaultOpenDictionary =>
      const SingleActivator(LogicalKeyboardKey.keyD, control: true);

  static ShortcutActivator get defaultOpenHistory =>
      const SingleActivator(LogicalKeyboardKey.keyH, control: true);

  /// Get default shortcuts map
  Map<ShortcutActivator, String> getDefaultShortcuts() {
    return {
      defaultVoiceActivation: 'voice_activation',
      defaultToggleTranslation: 'toggle_translation',
      defaultToggleAutoProcess: 'toggle_auto_process',
      defaultOpenDictionary: 'open_dictionary',
      defaultOpenHistory: 'open_history',
    };
  }

  /// Get shortcut descriptions for UI display
  List<ShortcutDescription> getShortcutDescriptions() {
    return [
      ShortcutDescription(
        name: 'Voice Activation',
        keys: 'Ctrl + Space',
        description: 'Start/Stop recording',
      ),
      ShortcutDescription(
        name: 'Toggle Translation',
        keys: 'Ctrl + T',
        description: 'Toggle translation on/off',
      ),
      ShortcutDescription(
        name: 'Toggle Auto-Process',
        keys: 'Ctrl + P',
        description: 'Toggle auto-process on/off',
      ),
      ShortcutDescription(
        name: 'Open Dictionary',
        keys: 'Ctrl + D',
        description: 'Open personal dictionary',
      ),
      ShortcutDescription(
        name: 'Open History',
        keys: 'Ctrl + H',
        description: 'Open transcription history',
      ),
    ];
  }

  /// Parse a key combination string to ShortcutActivator
  ShortcutActivator? parseShortcut(String shortcut) {
    final parts = shortcut.split('+').map((s) => s.trim()).toList();
    LogicalKeyboardKey? trigger;
    bool control = false;
    bool alt = false;
    bool shift = false;
    bool meta = false;

    for (final part in parts) {
      final key = _parseKey(part);
      if (key == LogicalKeyboardKey.control) {
        control = true;
      } else if (key == LogicalKeyboardKey.alt) {
        alt = true;
      } else if (key == LogicalKeyboardKey.shift) {
        shift = true;
      } else if (key == LogicalKeyboardKey.meta) {
        meta = true;
      } else if (key != null) {
        trigger = key;
      }
    }

    if (trigger == null) return null;
    return SingleActivator(trigger, control: control, alt: alt, shift: shift, meta: meta);
  }

  LogicalKeyboardKey? _parseKey(String key) {
    switch (key.toLowerCase()) {
      case 'ctrl':
      case 'control':
        return LogicalKeyboardKey.control;
      case 'alt':
        return LogicalKeyboardKey.alt;
      case 'shift':
        return LogicalKeyboardKey.shift;
      case 'meta':
      case 'cmd':
      case 'command':
        return LogicalKeyboardKey.meta;
      case 'space':
        return LogicalKeyboardKey.space;
      case 'a':
        return LogicalKeyboardKey.keyA;
      case 'b':
        return LogicalKeyboardKey.keyB;
      case 'c':
        return LogicalKeyboardKey.keyC;
      case 'd':
        return LogicalKeyboardKey.keyD;
      case 'e':
        return LogicalKeyboardKey.keyE;
      case 'f':
        return LogicalKeyboardKey.keyF;
      case 'g':
        return LogicalKeyboardKey.keyG;
      case 'h':
        return LogicalKeyboardKey.keyH;
      case 'i':
        return LogicalKeyboardKey.keyI;
      case 'j':
        return LogicalKeyboardKey.keyJ;
      case 'k':
        return LogicalKeyboardKey.keyK;
      case 'l':
        return LogicalKeyboardKey.keyL;
      case 'm':
        return LogicalKeyboardKey.keyM;
      case 'n':
        return LogicalKeyboardKey.keyN;
      case 'o':
        return LogicalKeyboardKey.keyO;
      case 'p':
        return LogicalKeyboardKey.keyP;
      case 'q':
        return LogicalKeyboardKey.keyQ;
      case 'r':
        return LogicalKeyboardKey.keyR;
      case 's':
        return LogicalKeyboardKey.keyS;
      case 't':
        return LogicalKeyboardKey.keyT;
      case 'u':
        return LogicalKeyboardKey.keyU;
      case 'v':
        return LogicalKeyboardKey.keyV;
      case 'w':
        return LogicalKeyboardKey.keyW;
      case 'x':
        return LogicalKeyboardKey.keyX;
      case 'y':
        return LogicalKeyboardKey.keyY;
      case 'z':
        return LogicalKeyboardKey.keyZ;
      case '0':
        return LogicalKeyboardKey.digit0;
      case '1':
        return LogicalKeyboardKey.digit1;
      case '2':
        return LogicalKeyboardKey.digit2;
      case '3':
        return LogicalKeyboardKey.digit3;
      case '4':
        return LogicalKeyboardKey.digit4;
      case '5':
        return LogicalKeyboardKey.digit5;
      case '6':
        return LogicalKeyboardKey.digit6;
      case '7':
        return LogicalKeyboardKey.digit7;
      case '8':
        return LogicalKeyboardKey.digit8;
      case '9':
        return LogicalKeyboardKey.digit9;
      case 'f1':
        return LogicalKeyboardKey.f1;
      case 'f2':
        return LogicalKeyboardKey.f2;
      case 'f3':
        return LogicalKeyboardKey.f3;
      case 'f4':
        return LogicalKeyboardKey.f4;
      case 'f5':
        return LogicalKeyboardKey.f5;
      case 'f6':
        return LogicalKeyboardKey.f6;
      case 'f7':
        return LogicalKeyboardKey.f7;
      case 'f8':
        return LogicalKeyboardKey.f8;
      case 'f9':
        return LogicalKeyboardKey.f9;
      case 'f10':
        return LogicalKeyboardKey.f10;
      case 'f11':
        return LogicalKeyboardKey.f11;
      case 'f12':
        return LogicalKeyboardKey.f12;
      case 'escape':
      case 'esc':
        return LogicalKeyboardKey.escape;
      case 'enter':
      case 'return':
        return LogicalKeyboardKey.enter;
      case 'tab':
        return LogicalKeyboardKey.tab;
      default:
        return null;
    }
  }
}

/// Description of a keyboard shortcut for UI display
class ShortcutDescription {
  final String name;
  final String keys;
  final String description;

  const ShortcutDescription({
    required this.name,
    required this.keys,
    required this.description,
  });
}
