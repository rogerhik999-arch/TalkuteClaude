/// Talkute AI Voice Input Assistant - Flutter State Management
///
/// This module provides Riverpod state management for the Flutter UI.

import 'package:flutter/foundation.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:riverpod/riverpod.dart';

// ============================================================================
// State Models
// ============================================================================

/// Voice session state
class VoiceSessionState {
  final String sessionId;
  final SessionStatus status;
  final String rawTranscription;
  final String polishedText;
  final int wordCount;
  final DateTime? endedAt;

  VoiceSessionState({
    required this.sessionId,
    required this.status,
    this.rawTranscription = '',
    this.polishedText = '',
    this.wordCount = 0,
    this.endedAt,
  });

  VoiceSessionState copyWith({
    String? sessionId,
    SessionStatus? status,
    String? rawTranscription,
    String? polishedText,
    int? wordCount,
    DateTime? endedAt,
  }) {
    return VoiceSessionState(
      sessionId: sessionId ?? this.sessionId,
      status: status ?? this.status,
      rawTranscription: rawTranscription ?? this.rawTranscription,
      polishedText: polishedText ?? this.polishedText,
      wordCount: wordCount ?? this.wordCount,
      endedAt: endedAt ?? this.endedAt,
    );
  }
}

/// Device profile state
class DeviceProfileState {
  final String deviceId;
  final String preferredLanguage;
  final double voiceSpeedPreference;
  final bool autoPunctuationEnabled;
  final bool fillerRemovalEnabled;
  final bool selfCorrectionEnabled;
  final bool crashReportingEnabled;

  DeviceProfileState({
    required this.deviceId,
    this.preferredLanguage = 'en-US',
    this.voiceSpeedPreference = 1.0,
    this.autoPunctuationEnabled = true,
    this.fillerRemovalEnabled = true,
    this.selfCorrectionEnabled = true,
    this.crashReportingEnabled = false,
  });

  DeviceProfileState copyWith({
    String? deviceId,
    String? preferredLanguage,
    double? voiceSpeedPreference,
    bool? autoPunctuationEnabled,
    bool? fillerRemovalEnabled,
    bool? selfCorrectionEnabled,
    bool? crashReportingEnabled,
  }) {
    return DeviceProfileState(
      deviceId: deviceId ?? this.deviceId,
      preferredLanguage: preferredLanguage ?? this.preferredLanguage,
      voiceSpeedPreference: voiceSpeedPreference ?? this.voiceSpeedPreference,
      autoPunctuationEnabled: autoPunctuationEnabled ?? this.autoPunctuationEnabled,
      fillerRemovalEnabled: fillerRemovalEnabled ?? this.fillerRemovalEnabled,
      selfCorrectionEnabled: selfCorrectionEnabled ?? this.selfCorrectionEnabled,
      crashReportingEnabled: crashReportingEnabled ?? this.crashReportingEnabled,
    );
  }
}

/// Quota state
class QuotaState {
  final int wordsUsedThisWeek;
  final int weeklyLimit;
  final double percentageUsed;

  QuotaState({
    this.wordsUsedThisWeek = 0,
    this.weeklyLimit = 4000,
    this.percentageUsed = 0.0,
  });

  QuotaState copyWith({
    int? wordsUsedThisWeek,
    int? weeklyLimit,
    double? percentageUsed,
  }) {
    return QuotaState(
      wordsUsedThisWeek: wordsUsedThisWeek ?? this.wordsUsedThisWeek,
      weeklyLimit: weeklyLimit ?? this.weeklyLimit,
      percentageUsed: percentageUsed ?? this.percentageUsed,
    );
  }

  bool get isLimitExceeded => wordsUsedThisWeek >= weeklyLimit;
  bool get isApproachingLimit => percentageUsed >= 90.0;
}

/// Application context state
class ApplicationContextState {
  final String contextId;
  final String applicationName;
  final String? applicationTitle;
  final String applicationCategory;
  final String? preferredTone;

  ApplicationContextState({
    required this.contextId,
    required this.applicationName,
    this.applicationTitle,
    this.applicationCategory = 'other',
    this.preferredTone,
  });
}

// ============================================================================
// State Providers
// ============================================================================

/// Voice session state provider
final voiceSessionStateProvider = StateNotifierProvider<VoiceSessionNotifier, VoiceSessionState>((ref) {
  return VoiceSessionNotifier();
});

/// Device profile state provider
final deviceProfileStateProvider = StateNotifierProvider<DeviceProfileNotifier, DeviceProfileState>((ref) {
  return DeviceProfileNotifier();
});

/// Quota state provider
final quotaStateProvider = StateNotifierProvider<QuotaNotifier, QuotaState>((ref) {
  return QuotaNotifier();
});

/// Application context provider
final applicationContextProvider = StateNotifierProvider<ApplicationContextNotifier, ApplicationContextState?>((ref) {
  return ApplicationContextNotifier();
});

// ============================================================================
// State Notifiers
// ============================================================================

/// Voice session state notifier
class VoiceSessionNotifier extends StateNotifier<VoiceSessionState> {
  VoiceSessionNotifier() : super(VoiceSessionState(sessionId: ''));

  void startSession(String sessionId) {
    state = state.copyWith(sessionId: sessionId, status: SessionStatus.idle);
  }

  void updateRawTranscription(String text) {
    state = state.copyWith(rawTranscription: text);
  }

  void updatePolishedText(String text) {
    state = state.copyWith(polishedText: text);
  }

  void setWordCount(int count) {
    state = state.copyWith(wordCount: count);
  }

  void stopSession() {
    state = state.copyWith(status: SessionStatus.completed, endedAt: DateTime.now());
  }

  void cancelSession() {
    state = state.copyWith(status: SessionStatus.cancelled, endedAt: DateTime.now());
  }

  void failSession(String errorMessage) {
    state = state.copyWith(status: SessionStatus.failed, endedAt: DateTime.now());
  }
}

/// Device profile state notifier
class DeviceProfileNotifier extends StateNotifier<DeviceProfileState> {
  DeviceProfileNotifier() : super(DeviceProfileState(deviceId: ''));

  void setDeviceId(String deviceId) {
    state = state.copyWith(deviceId: deviceId);
  }

  void updateSettings({
    String? preferredLanguage,
    double? voiceSpeedPreference,
    bool? autoPunctuationEnabled,
    bool? fillerRemovalEnabled,
    bool? selfCorrectionEnabled,
    bool? crashReportingEnabled,
  }) {
    state = state.copyWith(
      preferredLanguage: preferredLanguage,
      voiceSpeedPreference: voiceSpeedPreference,
      autoPunctuationEnabled: autoPunctuationEnabled,
      fillerRemovalEnabled: fillerRemovalEnabled,
      selfCorrectionEnabled: selfCorrectionEnabled,
      crashReportingEnabled: crashReportingEnabled,
    );
  }
}

/// Quota state notifier
class QuotaNotifier extends StateNotifier<QuotaState> {
  QuotaNotifier() : super(QuotaState());

  void setQuota(int wordsUsed, int weeklyLimit) {
    final percentageUsed = (wordsUsed / weeklyLimit * 100).clamp(0.0, 100.0);
    state = state.copyWith(
      wordsUsedThisWeek: wordsUsed,
      weeklyLimit: weeklyLimit,
      percentageUsed: percentageUsed,
    );
  }

  void addWords(int count) {
    final newWords = state.wordsUsedThisWeek + count;
    final percentageUsed = (newWords / state.weeklyLimit * 100).clamp(0.0, 100.0);
    state = state.copyWith(
      wordsUsedThisWeek: newWords,
      percentageUsed: percentageUsed,
    );
  }

  void reset() {
    state = QuotaState();
  }
}

/// Application context state notifier
class ApplicationContextNotifier extends StateNotifier<ApplicationContextState?> {
  ApplicationContextNotifier() : super(null);

  void updateContext(ApplicationContextState context) {
    state = context;
  }

  void clearContext() {
    state = null;
  }
}
