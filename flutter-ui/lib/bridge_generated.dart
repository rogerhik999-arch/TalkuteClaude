/// Auto-generated FFI bindings for Talkute Rust core
///
/// This file provides the Dart interface to the Rust FFI functions.
/// Generated from rust-core/src/ffi/bridge.rs

// ignore_for_file: non_constant_identifier_names, avoid_positional_boolean_parameters

import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:io';

import 'package:flutter/foundation.dart';

/// Session status enum matching Rust SessionStatus
enum SessionStatus {
  idle,
  recording,
  transcribing,
  polishing,
  completed,
  failed,
  cancelled,
  stopped,
}

/// Voice session information
class VoiceSessionInfo {
  final String sessionId;
  final SessionStatus status;
  final String startedAt;
  final int durationSeconds;
  final int wordCount;

  VoiceSessionInfo({
    required this.sessionId,
    required this.status,
    required this.startedAt,
    required this.durationSeconds,
    required this.wordCount,
  });

  factory VoiceSessionInfo.fromJson(Map<String, dynamic> json) {
    return VoiceSessionInfo(
      sessionId: json['session_id'] as String,
      status: SessionStatus.values.firstWhere(
        (e) => e.name == json['status'],
        orElse: () => SessionStatus.idle,
      ),
      startedAt: json['started_at'] as String,
      durationSeconds: json['duration_seconds'] as int,
      wordCount: json['word_count'] as int,
    );
  }

  Map<String, dynamic> toJson() => {
    'session_id': sessionId,
    'status': status.name,
    'started_at': startedAt,
    'duration_seconds': durationSeconds,
    'word_count': wordCount,
  };
}

/// Application context information
class ApplicationContext {
  final String contextId;
  final String applicationName;
  final String? applicationTitle;
  final String applicationCategory;
  final String? preferredTone;

  ApplicationContext({
    required this.contextId,
    required this.applicationName,
    this.applicationTitle,
    required this.applicationCategory,
    this.preferredTone,
  });

  factory ApplicationContext.fromJson(Map<String, dynamic> json) {
    return ApplicationContext(
      contextId: json['context_id'] as String,
      applicationName: json['application_name'] as String,
      applicationTitle: json['application_title'] as String?,
      applicationCategory: json['application_category'] as String,
      preferredTone: json['preferred_tone'] as String?,
    );
  }

  Map<String, dynamic> toJson() => {
    'context_id': contextId,
    'application_name': applicationName,
    'application_title': applicationTitle,
    'application_category': applicationCategory,
    'preferred_tone': preferredTone,
  };
}

/// Dictionary entry
class DictionaryEntry {
  final String phrase;
  final String replacement;
  final bool caseSensitive;
  final String? category;

  DictionaryEntry({
    required this.phrase,
    required this.replacement,
    required this.caseSensitive,
    this.category,
  });

  factory DictionaryEntry.fromJson(Map<String, dynamic> json) {
    return DictionaryEntry(
      phrase: json['phrase'] as String,
      replacement: json['replacement'] as String,
      caseSensitive: json['case_sensitive'] as bool,
      category: json['category'] as String?,
    );
  }

  Map<String, dynamic> toJson() => {
    'phrase': phrase,
    'replacement': replacement,
    'case_sensitive': caseSensitive,
    'category': category,
  };
}

/// Usage quota information
class QuotaUsage {
  final int wordsUsedThisWeek;
  final int weeklyLimit;
  final double percentageUsed;

  QuotaUsage({
    required this.wordsUsedThisWeek,
    required this.weeklyLimit,
    required this.percentageUsed,
  });

  factory QuotaUsage.fromJson(Map<String, dynamic> json) {
    return QuotaUsage(
      wordsUsedThisWeek: json['words_used_this_week'] as int,
      weeklyLimit: json['weekly_limit'] as int,
      percentageUsed: (json['percentage_used'] as num).toDouble(),
    );
  }

  Map<String, dynamic> toJson() => {
    'words_used_this_week': wordsUsedThisWeek,
    'weekly_limit': weeklyLimit,
    'percentage_used': percentageUsed,
  };
}

/// FFI error type
class FfiError implements Exception {
  final String message;
  final int? code;

  FfiError(this.message, {this.code});

  @override
  String toString() => 'FfiError: $message';
}

/// Talkute FFI bridge
///
/// Provides access to Rust core functionality via FFI.
/// This is a singleton that wraps the native library.
class TalkuteBridge {
  static final TalkuteBridge _instance = TalkuteBridge._internal();
  factory TalkuteBridge() => _instance;
  TalkuteBridge._internal();

  // Simulated session storage for now (will be replaced by actual FFI calls)
  final Map<String, VoiceSessionInfo> _sessions = {};

  // Session Management
  Future<String> startVoiceSession(String deviceId, {String? contextId}) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    final sessionId = 'session_${DateTime.now().millisecondsSinceEpoch}';
    _sessions[sessionId] = VoiceSessionInfo(
      sessionId: sessionId,
      status: SessionStatus.recording,
      startedAt: DateTime.now().toIso8601String(),
      durationSeconds: 0,
      wordCount: 0,
    );
    return sessionId;
  }

  Future<VoiceSessionInfo> stopVoiceSession(String sessionId) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    final session = _sessions[sessionId];
    if (session == null) {
      throw FfiError('Session not found: $sessionId');
    }
    return VoiceSessionInfo(
      sessionId: sessionId,
      status: SessionStatus.completed,
      startedAt: session.startedAt,
      durationSeconds: DateTime.now().difference(DateTime.parse(session.startedAt)).inSeconds,
      wordCount: session.wordCount,
    );
  }

  Future<void> cancelVoiceSession(String sessionId, {String? reason}) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    _sessions.remove(sessionId);
  }

  Future<SessionStatus> getSessionStatus(String sessionId) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    final session = _sessions[sessionId];
    if (session == null) {
      throw FfiError('Session not found: $sessionId');
    }
    return session.status;
  }

  // Audio Processing
  Future<String> startAudioCapture(String deviceId) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    return startVoiceSession(deviceId);
  }

  Future<void> stopAudioCapture(String captureId) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    // Update session status
  }

  Future<double> getAudioLevel(String captureId) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
    return 0.5; // Mock audio level
  }

  // Transcription & AI
  Future<String> transcribeAudio(String sessionId, String audioPath, {String? language}) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 500));
    return 'Mock transcription for session $sessionId';
  }

  Future<String> polishText(String sessionId, String text, {String? contextId}) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 300));
    return text; // Return as-is for mock
  }

  Future<String> getRawTranscription(String sessionId) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    return 'Raw transcription';
  }

  Future<String> getPolishedText(String sessionId) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    return 'Polished text';
  }

  // Context Detection
  Future<ApplicationContext> detectApplicationContext() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 30));
    return ApplicationContext(
      contextId: 'unknown',
      applicationName: 'Unknown',
      applicationCategory: 'general',
    );
  }

  Future<List<ApplicationContext>> getAllContexts() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    return [];
  }

  // Device Profile
  Future<Map<String, dynamic>> getOrCreateDeviceProfile() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    return {
      'device_id': 'device_${DateTime.now().millisecondsSinceEpoch}',
      'created_at': DateTime.now().toIso8601String(),
      'settings': {},
    };
  }

  Future<void> updateDeviceProfile(Map<String, dynamic> settings) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
  }

  // Usage Quota
  Future<bool> checkQuotaAvailable(int wordsNeeded) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
    return true;
  }

  Future<QuotaUsage> getQuotaUsage() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
    return QuotaUsage(
      wordsUsedThisWeek: 1000,
      weeklyLimit: 4000,
      percentageUsed: 0.25,
    );
  }

  Future<void> addWordsToQuota(int wordCount) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
  }

  // Personal Dictionary
  Future<void> addDictionaryEntry(String phrase, String replacement, bool caseSensitive) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
  }

  Future<void> removeDictionaryEntry(String phrase) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
  }

  Future<List<DictionaryEntry>> getDictionaryEntries() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    return [];
  }

  // Migration & Setup
  Future<void> runMigrations() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 100));
  }

  Future<int> getSchemaVersion() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
    return 1;
  }

  // System Tray
  Future<void> setTrayIcon(String state) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
  }

  Future<void> showTrayNotification(String title, String message) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
  }

  // Hotkey
  Future<void> registerGlobalHotkey(String hotkey) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
  }

  Future<void> unregisterGlobalHotkey() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
  }

  Future<String?> getCurrentHotkey() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
    return 'Ctrl+Space';
  }

  // Floating Window
  Future<void> showFloatingCapsule() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
  }

  Future<void> hideFloatingCapsule() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
  }

  Future<void> setCapsuleState(String state) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
  }

  // Text Injection
  Future<String> injectTextAtCursor(String text) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
    return 'injected';
  }

  Future<void> copyToClipboard(String text) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
  }

  Future<String?> getFocusedApplication() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
    return 'Unknown';
  }

  // Preferences
  Future<String?> getPreference(String key) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
    return null;
  }

  Future<void> setPreference(String key, String value) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 10));
  }

  Future<Map<String, String>> getAllPreferences() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
    return {};
  }

  // History
  Future<List<Map<String, dynamic>>> listHistory(int limit, int offset) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
    return [];
  }

  Future<void> clearAllHistory() async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 50));
  }

  Future<void> deleteHistoryEntry(String id) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 20));
  }

  // Data Export
  Future<String> exportData(String format) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 100));
    return jsonEncode({'exported_at': DateTime.now().toIso8601String()});
  }

  Future<int> importDictionary(String jsonData) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 100));
    return 0;
  }

  // Cleanup
  Future<int> runCleanup(int retentionDays) async {
    // TODO: Replace with actual FFI call
    await Future.delayed(const Duration(milliseconds: 100));
    return 0;
  }
}

/// Global bridge instance
final bridge = TalkuteBridge();