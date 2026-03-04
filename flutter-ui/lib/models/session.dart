/// Voice session model

import 'package:uuid/uuid.dart';

class VoiceSession {
  final String sessionId;
  final String deviceId;
  final String? contextId;
  final DateTime startedAt;
  final DateTime? endedAt;
  final int? durationSeconds;
  final String? rawTranscription;
  final String? polishedText;
  final int wordCount;
  final String status;
  final String? errorMessage;
  final String? aiModelUsed;
  final String? speechApiUsed;

  VoiceSession({
    required this.sessionId,
    required this.deviceId,
    this.contextId,
    required this.startedAt,
    this.endedAt,
    this.durationSeconds,
    this.rawTranscription,
    this.polishedText,
    this.wordCount = 0,
    this.status = 'RECORDING',
    this.errorMessage,
    this.aiModelUsed,
    this.speechApiUsed,
  });

  factory VoiceSession.create(String deviceId) {
    return VoiceSession(
      sessionId: const Uuid().v4(),
      deviceId: deviceId,
      startedAt: DateTime.now(),
    );
  }

  VoiceSession copyWith({
    String? sessionId,
    String? deviceId,
    String? contextId,
    DateTime? startedAt,
    DateTime? endedAt,
    int? durationSeconds,
    String? rawTranscription,
    String? polishedText,
    int? wordCount,
    String? status,
    String? errorMessage,
    String? aiModelUsed,
    String? speechApiUsed,
  }) {
    return VoiceSession(
      sessionId: sessionId ?? this.sessionId,
      deviceId: deviceId ?? this.deviceId,
      contextId: contextId ?? this.contextId,
      startedAt: startedAt ?? this.startedAt,
      endedAt: endedAt ?? this.endedAt,
      durationSeconds: durationSeconds ?? this.durationSeconds,
      rawTranscription: rawTranscription ?? this.rawTranscription,
      polishedText: polishedText ?? this.polishedText,
      wordCount: wordCount ?? this.wordCount,
      status: status ?? this.status,
      errorMessage: errorMessage ?? this.errorMessage,
      aiModelUsed: aiModelUsed ?? this.aiModelUsed,
      speechApiUsed: speechApiUsed ?? this.speechApiUsed,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'sessionId': sessionId,
      'deviceId': deviceId,
      'contextId': contextId,
      'startedAt': startedAt.toIso8601String(),
      'endedAt': endedAt?.toIso8601String(),
      'durationSeconds': durationSeconds,
      'rawTranscription': rawTranscription,
      'polishedText': polishedText,
      'wordCount': wordCount,
      'status': status,
      'errorMessage': errorMessage,
      'aiModelUsed': aiModelUsed,
      'speechApiUsed': speechApiUsed,
    };
  }

  factory VoiceSession.fromJson(Map<String, dynamic> json) {
    return VoiceSession(
      sessionId: json['sessionId'] as String,
      deviceId: json['deviceId'] as String,
      contextId: json['contextId'] as String?,
      startedAt: DateTime.parse(json['startedAt'] as String),
      endedAt: json['endedAt'] != null ? DateTime.parse(json['endedAt'] as String) : null,
      durationSeconds: json['durationSeconds'] as int?,
      rawTranscription: json['rawTranscription'] as String?,
      polishedText: json['polishedText'] as String?,
      wordCount: json['wordCount'] as int? ?? 0,
      status: json['status'] as String? ?? 'RECORDING',
      errorMessage: json['errorMessage'] as String?,
      aiModelUsed: json['aiModelUsed'] as String?,
      speechApiUsed: json['speechApiUsed'] as String?,
    );
  }
}

/// Session status enum
enum SessionStatus {
  idle,
  recording,
  transcribing,
  polishing,
  completed,
  failed,
  cancelled,
}

extension SessionStatusExtension on SessionStatus {
  String get value {
    switch (this) {
      case SessionStatus.idle:
        return 'IDLE';
      case SessionStatus.recording:
        return 'RECORDING';
      case SessionStatus.transcribing:
        return 'TRANSCRIBING';
      case SessionStatus.polishing:
        return 'POLISHING';
      case SessionStatus.completed:
        return 'COMPLETED';
      case SessionStatus.failed:
        return 'FAILED';
      case SessionStatus.cancelled:
        return 'CANCELLED';
    }
  }
}
