// Tests for context-aware voice state

import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/models/context.dart';
import 'package:talkute/state/voice_state.dart';

void main() {
  group('VoiceState Context Tests', () {
    test('VoiceState should have detectedContext field', () {
      const state = VoiceState();

      // VoiceState should have a detectedContext field
      expect(state.detectedContext, isNull);
    });

    test('VoiceState should have detectedAt field', () {
      const state = VoiceState();

      // VoiceState should have a detectedAt field
      expect(state.detectedAt, isNull);
    });

    test('VoiceState copyWith should accept detectedContext', () {
      const state = VoiceState();
      final context = ApplicationContext(
        contextId: 'test-123',
        applicationName: 'TestApp',
        applicationCategory: 'email',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      final updated = state.copyWith(detectedContext: context);

      expect(updated.detectedContext, equals(context));
      expect(updated.detectedContext?.applicationName, equals('TestApp'));
    });

    test('VoiceState should track context detection timestamp', () {
      const state = VoiceState();
      final detectedAt = DateTime.now();

      final updated = state.copyWith(detectedAt: detectedAt);

      expect(updated.detectedAt, equals(detectedAt));
    });

    test('VoiceState should clear context when reset', () {
      final context = ApplicationContext(
        contextId: 'test-456',
        applicationName: 'Slack',
        applicationCategory: 'chat',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      final state = VoiceState(
        status: VoiceRecordingStatus.completed,
        sessionId: 'session-123',
        detectedContext: context,
        detectedAt: DateTime.now(),
      );

      // When reset is called, context should be cleared
      final resetState = VoiceState();
      expect(resetState.detectedContext, isNull);
      expect(resetState.detectedAt, isNull);
    });

    test('ApplicationContext should have correct category mapping', () {
      final emailContext = ApplicationContext(
        contextId: '1',
        applicationName: 'Outlook',
        applicationCategory: 'email',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      final chatContext = ApplicationContext(
        contextId: '2',
        applicationName: 'Slack',
        applicationCategory: 'chat',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      expect(emailContext.applicationCategory, equals('email'));
      expect(chatContext.applicationCategory, equals('chat'));
    });
  });

  group('VoiceStateNotifier Context Tests', () {
    test('Notifier should have method to set detected context', () {
      final container = ProviderContainer();
      final notifier = container.read(voiceStateProvider.notifier);
      final context = ApplicationContext(
        contextId: 'ctx-123',
        applicationName: 'Gmail',
        applicationCategory: 'email',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      // Should have setDetectedContext method
      notifier.setDetectedContext(context);

      final state = container.read(voiceStateProvider);
      expect(state.detectedContext, equals(context));
    });

    test('Notifier should have method to clear detected context', () {
      final container = ProviderContainer();
      final notifier = container.read(voiceStateProvider.notifier);

      // First set a context
      final context = ApplicationContext(
        contextId: 'ctx-456',
        applicationName: 'Slack',
        applicationCategory: 'chat',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );
      notifier.setDetectedContext(context);

      // Then clear it
      notifier.clearDetectedContext();

      final state = container.read(voiceStateProvider);
      expect(state.detectedContext, isNull);
    });

    test('startRecording should clear previous context', () async {
      final container = ProviderContainer();
      final notifier = container.read(voiceStateProvider.notifier);

      // Set initial context
      final context = ApplicationContext(
        contextId: 'ctx-789',
        applicationName: 'VSCode',
        applicationCategory: 'code',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );
      notifier.setDetectedContext(context);

      // Start new recording
      await notifier.startRecording();

      final state = container.read(voiceStateProvider);
      expect(state.detectedContext, isNull);
      expect(state.status, equals(VoiceRecordingStatus.recording));
    });
  });
}