/// User preferences models
///
/// Data models for user configuration.

import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Input mode for voice recording
enum InputMode {
  pushToTalk,
  toggle,
}

extension InputModeExtension on InputMode {
  String get value {
    switch (this) {
      case InputMode.pushToTalk:
        return 'push_to_talk';
      case InputMode.toggle:
        return 'toggle';
    }
  }

  static InputMode fromString(String value) {
    switch (value) {
      case 'push_to_talk':
        return InputMode.pushToTalk;
      case 'toggle':
        return InputMode.toggle;
      default:
        return InputMode.pushToTalk;
    }
  }
}

/// Polishing intensity level
enum PolishingIntensity {
  light,
  standard,
  deep,
}

extension PolishingIntensityExtension on PolishingIntensity {
  String get value {
    switch (this) {
      case PolishingIntensity.light:
        return 'light';
      case PolishingIntensity.standard:
        return 'standard';
      case PolishingIntensity.deep:
        return 'deep';
    }
  }

  static PolishingIntensity fromString(String value) {
    switch (value) {
      case 'light':
        return PolishingIntensity.light;
      case 'deep':
        return PolishingIntensity.deep;
      default:
        return PolishingIntensity.standard;
    }
  }
}

/// User preferences data class
class UserPreferences {
  final InputMode inputMode;
  final double idleTimeoutSeconds;
  final PolishingIntensity polishingIntensity;
  final String inputLanguage;
  final String outputLanguage;
  final bool translationEnabled;
  final bool autoProcess;
  final bool fillerRemoval;
  final bool contextAware;
  final bool noiseCancellation;
  final String hotkey;
  final int historyRetentionDays;
  final bool crashReporting;
  final bool analytics;

  const UserPreferences({
    this.inputMode = InputMode.pushToTalk,
    this.idleTimeoutSeconds = 1.5,
    this.polishingIntensity = PolishingIntensity.standard,
    this.inputLanguage = 'auto',
    this.outputLanguage = 'same_as_input',
    this.translationEnabled = false,
    this.autoProcess = true,
    this.fillerRemoval = true,
    this.contextAware = true,
    this.noiseCancellation = true,
    this.hotkey = 'Ctrl+Shift+Space',
    this.historyRetentionDays = 30,
    this.crashReporting = true,
    this.analytics = false,
  });

  UserPreferences copyWith({
    InputMode? inputMode,
    double? idleTimeoutSeconds,
    PolishingIntensity? polishingIntensity,
    String? inputLanguage,
    String? outputLanguage,
    bool? translationEnabled,
    bool? autoProcess,
    bool? fillerRemoval,
    bool? contextAware,
    bool? noiseCancellation,
    String? hotkey,
    int? historyRetentionDays,
    bool? crashReporting,
    bool? analytics,
  }) {
    return UserPreferences(
      inputMode: inputMode ?? this.inputMode,
      idleTimeoutSeconds: idleTimeoutSeconds ?? this.idleTimeoutSeconds,
      polishingIntensity: polishingIntensity ?? this.polishingIntensity,
      inputLanguage: inputLanguage ?? this.inputLanguage,
      outputLanguage: outputLanguage ?? this.outputLanguage,
      translationEnabled: translationEnabled ?? this.translationEnabled,
      autoProcess: autoProcess ?? this.autoProcess,
      fillerRemoval: fillerRemoval ?? this.fillerRemoval,
      contextAware: contextAware ?? this.contextAware,
      noiseCancellation: noiseCancellation ?? this.noiseCancellation,
      hotkey: hotkey ?? this.hotkey,
      historyRetentionDays: historyRetentionDays ?? this.historyRetentionDays,
      crashReporting: crashReporting ?? this.crashReporting,
      analytics: analytics ?? this.analytics,
    );
  }
}

/// Preferences provider
final userPreferencesProvider = StateProvider<UserPreferences>((ref) {
  return const UserPreferences();
});