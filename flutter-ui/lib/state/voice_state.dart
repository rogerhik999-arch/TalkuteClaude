/// Voice state management for Talkute
///
/// Manages the voice recording, transcription, and polishing workflow.

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:uuid/uuid.dart';

/// Voice recording state
enum VoiceRecordingStatus {
  idle,
  recording,
  transcribing,
  polishing,
  completed,
  error,
}

/// Voice state
class VoiceState {
  final VoiceRecordingStatus status;
  final String sessionId;
  final String rawTranscription;
  final String polishedText;
  final double audioLevel;
  final String? errorMessage;
  final int wordCount;

  const VoiceState({
    this.status = VoiceRecordingStatus.idle,
    this.sessionId = '',
    this.rawTranscription = '',
    this.polishedText = '',
    this.audioLevel = 0.0,
    this.errorMessage,
    this.wordCount = 0,
  });

  VoiceState copyWith({
    VoiceRecordingStatus? status,
    String? sessionId,
    String? rawTranscription,
    String? polishedText,
    double? audioLevel,
    String? errorMessage,
    int? wordCount,
  }) {
    return VoiceState(
      status: status ?? this.status,
      sessionId: sessionId ?? this.sessionId,
      rawTranscription: rawTranscription ?? this.rawTranscription,
      polishedText: polishedText ?? this.polishedText,
      audioLevel: audioLevel ?? this.audioLevel,
      errorMessage: errorMessage ?? this.errorMessage,
      wordCount: wordCount ?? this.wordCount,
    );
  }

  bool get isRecording => status == VoiceRecordingStatus.recording;
  bool get isProcessing => status == VoiceRecordingStatus.transcribing ||
      status == VoiceRecordingStatus.polishing;
  bool get hasError => errorMessage != null || status == VoiceRecordingStatus.error;
  bool get hasTranscription => rawTranscription.isNotEmpty;
  bool get hasPolishedText => polishedText.isNotEmpty;
}

/// Voice state notifier
class VoiceStateNotifier extends StateNotifier<VoiceState> {
  VoiceStateNotifier() : super(const VoiceState());

  /// Start a new voice recording session
  Future<void> startRecording() async {
    final sessionId = const Uuid().v4();
    state = state.copyWith(
      status: VoiceRecordingStatus.recording,
      sessionId: sessionId,
      rawTranscription: '',
      polishedText: '',
      errorMessage: null,
    );
  }

  /// Stop recording and start transcription
  Future<void> stopRecording() async {
    state = state.copyWith(
      status: VoiceRecordingStatus.transcribing,
      audioLevel: 0.0,
    );
  }

  /// Cancel the current session
  void cancelSession() {
    state = state.copyWith(
      status: VoiceRecordingStatus.idle,
      audioLevel: 0.0,
    );
  }

  /// Update the audio level (0.0 - 1.0)
  void updateAudioLevel(double level) {
    if (state.isRecording) {
      state = state.copyWith(audioLevel: level.clamp(0.0, 1.0));
    }
  }

  /// Set the raw transcription result
  void setRawTranscription(String text) {
    state = state.copyWith(
      rawTranscription: text,
      wordCount: text.split(' ').where((s) => s.isNotEmpty).length,
    );
  }

  /// Set the polished text result
  void setPolishedText(String text) {
    state = state.copyWith(
      polishedText: text,
      status: VoiceRecordingStatus.completed,
    );
  }

  /// Start polishing the text
  void startPolishing() {
    state = state.copyWith(status: VoiceRecordingStatus.polishing);
  }

  /// Set an error
  void setError(String message) {
    state = state.copyWith(
      status: VoiceRecordingStatus.error,
      errorMessage: message,
    );
  }

  /// Reset to idle state
  void reset() {
    state = const VoiceState();
  }
}

/// Voice state provider
final voiceStateProvider =
    StateNotifierProvider<VoiceStateNotifier, VoiceState>((ref) {
  return VoiceStateNotifier();
});

/// Provider for checking if voice input is available
final voiceInputAvailableProvider = Provider<bool>((ref) {
  // TODO: Check actual device capabilities
  return true;
});

/// Provider for the current transcription text
final transcriptionTextProvider = Provider<String>((ref) {
  final state = ref.watch(voiceStateProvider);
  return state.rawTranscription;
});

/// Provider for the current polished text
final polishedTextProvider = Provider<String>((ref) {
  final state = ref.watch(voiceStateProvider);
  return state.polishedText;
});

/// Provider for the audio level
final audioLevelProvider = Provider<double>((ref) {
  final state = ref.watch(voiceStateProvider);
  return state.audioLevel;
});
