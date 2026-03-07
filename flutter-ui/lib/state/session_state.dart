/// Session state provider for Flutter
///
/// Manages the current session state using Riverpod.

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'session_state.g.dart';

/// Session state enum matching Rust core
enum SessionState {
  idle,
  recording,
  processing,
  error,
}

/// Session state data class
class SessionData {
  final String? sessionId;
  final SessionState state;
  final String? errorMessage;
  final double? progress;

  const SessionData({
    this.sessionId,
    this.state = SessionState.idle,
    this.errorMessage,
    this.progress,
  });

  SessionData copyWith({
    String? sessionId,
    SessionState? state,
    String? errorMessage,
    double? progress,
  }) {
    return SessionData(
      sessionId: sessionId ?? this.sessionId,
      state: state ?? this.state,
      errorMessage: errorMessage ?? this.errorMessage,
      progress: progress ?? this.progress,
    );
  }

  bool get isIdle => state == SessionState.idle;
  bool get isRecording => state == SessionState.recording;
  bool get isProcessing => state == SessionState.processing;
  bool get isError => state == SessionState.error;
}

/// Session state notifier
@riverpod
class SessionStateNotifier extends _$SessionStateNotifier {
  @override
  SessionData build() {
    return const SessionData();
  }

  /// Start a new recording session
  void startRecording(String sessionId) {
    state = SessionData(
      sessionId: sessionId,
      state: SessionState.recording,
    );
  }

  /// Stop recording and start processing
  void startProcessing() {
    state = state.copyWith(
      state: SessionState.processing,
      progress: 0.0,
    );
  }

  /// Update processing progress
  void updateProgress(double progress) {
    state = state.copyWith(progress: progress);
  }

  /// Complete the session successfully
  void complete() {
    state = const SessionData();
  }

  /// Set error state
  void setError(String errorMessage) {
    state = state.copyWith(
      state: SessionState.error,
      errorMessage: errorMessage,
    );
  }

  /// Retry from error state
  void retry() {
    state = state.copyWith(
      state: SessionState.processing,
      errorMessage: null,
      progress: 0.0,
    );
  }

  /// Dismiss error and return to idle
  void dismissError() {
    state = const SessionData();
  }

  /// Cancel the current session
  void cancel() {
    state = const SessionData();
  }
}

/// Convenience provider for session state
@riverpod
SessionState sessionState(SessionStateRef ref) {
  return ref.watch(sessionStateNotifierProvider).state;
}

/// Convenience provider for session ID
@riverpod
String? sessionId(SessionIdRef ref) {
  return ref.watch(sessionStateNotifierProvider).sessionId;
}

/// Convenience provider for error message
@riverpod
String? sessionError(SessionErrorRef ref) {
  return ref.watch(sessionStateNotifierProvider).errorMessage;
}

/// Convenience provider for processing progress
@riverpod
double? sessionProgress(SessionProgressRef ref) {
  return ref.watch(sessionStateNotifierProvider).progress;
}