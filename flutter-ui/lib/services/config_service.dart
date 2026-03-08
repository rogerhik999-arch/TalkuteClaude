/// Configuration Service
///
/// Manages app preferences and settings persistence.

import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Config service provider
final configServiceProvider = Provider<ConfigService>((ref) {
  return ConfigService(ref);
});

/// Service for managing app configuration
class ConfigService {
  final Ref _ref;

  // In-memory cache of preferences
  final Map<String, String> _preferences = {};

  ConfigService(this._ref);

  /// Get a preference value
  Future<String?> getPreference(String key) async {
    // Check cache first
    if (_preferences.containsKey(key)) {
      return _preferences[key];
    }

    // TODO: Call Rust FFI get_preference when bridge is ready
    // For now, return null
    return null;
  }

  /// Set a preference value
  Future<void> setPreference(String key, String value) async {
    _preferences[key] = value;

    // TODO: Call Rust FFI set_preference when bridge is ready
  }

  /// Get a boolean preference
  Future<bool> getBool(String key, {bool defaultValue = false}) async {
    final value = await getPreference(key);
    if (value == null) return defaultValue;
    return value.toLowerCase() == 'true';
  }

  /// Set a boolean preference
  Future<void> setBool(String key, bool value) async {
    await setPreference(key, value.toString());
  }

  /// Get a double preference
  Future<double> getDouble(String key, {double defaultValue = 0.0}) async {
    final value = await getPreference(key);
    if (value == null) return defaultValue;
    return double.tryParse(value) ?? defaultValue;
  }

  /// Set a double preference
  Future<void> setDouble(String key, double value) async {
    await setPreference(key, value.toString());
  }

  /// Get a string preference
  Future<String> getString(String key, {String defaultValue = ''}) async {
    final value = await getPreference(key);
    return value ?? defaultValue;
  }

  /// Set a string preference
  Future<void> setString(String key, String value) async {
    await setPreference(key, value);
  }

  // ============================================================
  // Convenience methods for specific preferences
  // ============================================================

  // Input mode
  Future<String> getInputMode() => getString('input_mode', defaultValue: 'push_to_talk');
  Future<void> setInputMode(String mode) => setString('input_mode', mode);

  // Idle timeout
  Future<double> getIdleTimeout() => getDouble('idle_timeout', defaultValue: 3.0);
  Future<void> setIdleTimeout(double seconds) => setDouble('idle_timeout', seconds);

  // Auto-process
  Future<bool> getAutoProcess() => getBool('auto_process', defaultValue: true);
  Future<void> setAutoProcess(bool enabled) => setBool('auto_process', enabled);

  // Filler removal
  Future<bool> getFillerRemoval() => getBool('filler_removal', defaultValue: true);
  Future<void> setFillerRemoval(bool enabled) => setBool('filler_removal', enabled);

  // Context-aware mode
  Future<bool> getContextAware() => getBool('context_aware', defaultValue: true);
  Future<void> setContextAware(bool enabled) => setBool('context_aware', enabled);

  // Noise cancellation
  Future<bool> getNoiseCancellation() => getBool('noise_cancellation', defaultValue: true);
  Future<void> setNoiseCancellation(bool enabled) => setBool('noise_cancellation', enabled);

  // Auto-translate
  Future<bool> getAutoTranslate() => getBool('auto_translate', defaultValue: false);
  Future<void> setAutoTranslate(bool enabled) => setBool('auto_translate', enabled);

  // Languages
  Future<String> getInputLanguage() => getString('input_language', defaultValue: 'en');
  Future<void> setInputLanguage(String code) => setString('input_language', code);

  Future<String> getTranslationLanguage() => getString('translation_language', defaultValue: 'zh');
  Future<void> setTranslationLanguage(String code) => setString('translation_language', code);

  // Privacy
  Future<bool> getCrashReporting() => getBool('crash_reporting', defaultValue: false);
  Future<void> setCrashReporting(bool enabled) => setBool('crash_reporting', enabled);

  Future<bool> getAnalytics() => getBool('analytics', defaultValue: false);
  Future<void> setAnalytics(bool enabled) => setBool('analytics', enabled);

  // Hotkey
  Future<String> getHotkey() => getString('hotkey', defaultValue: 'Ctrl+Space');
  Future<void> setHotkey(String hotkey) => setString('hotkey', hotkey);

  // Polishing intensity
  Future<String> getPolishingIntensity() => getString('polishing_intensity', defaultValue: 'standard');
  Future<void> setPolishingIntensity(String intensity) => setString('polishing_intensity', intensity);

  // Data retention
  Future<int> getRetentionDays() async {
    final value = await getPreference('retention_days');
    if (value == null) return 30;
    return int.tryParse(value) ?? 30;
  }
  Future<void> setRetentionDays(int days) => setPreference('retention_days', days.toString());
}