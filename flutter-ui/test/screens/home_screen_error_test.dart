/// Tests for error handling in home screen
///
/// Tests the error display and recovery flows.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/screens/home_screen.dart';
import 'package:talkute/state/voice_state.dart';
import 'package:talkute/widgets/voice_button.dart';
import 'package:talkute/widgets/transcription_preview.dart';

void main() {
  group('HomeScreen error handling', () {
    testWidgets('displays error message when error occurs', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set error state
      container.read(voiceStateProvider.notifier).setError('Network error occurred');
      await tester.pump();

      // Should show error view
      expect(find.text('Something went wrong'), findsOneWidget);
      expect(find.text('Network error occurred'), findsOneWidget);
      expect(find.byIcon(Icons.error_outline), findsWidgets);
    });

    testWidgets('shows retry option after error', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set error state
      container.read(voiceStateProvider.notifier).setError('Test error');
      await tester.pump();

      // Voice button should show error state
      expect(find.text('Error - Tap to retry'), findsOneWidget);

      // Tap to retry should reset error
      await tester.tap(find.byType(VoiceButton));
      await tester.pump();

      // Should transition to recording state
      expect(container.read(voiceStateProvider).isRecording, isTrue);
    });

    testWidgets('clears error when starting new recording', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set error state
      container.read(voiceStateProvider.notifier).setError('Previous error');
      await tester.pump();
      expect(container.read(voiceStateProvider).hasError, isTrue);

      // Start new recording - the startRecording method should clear error
      container.read(voiceStateProvider.notifier).startRecording();
      await tester.pump();

      // Error should be cleared (startRecording sets errorMessage to null)
      expect(container.read(voiceStateProvider).errorMessage, isNull);
      expect(container.read(voiceStateProvider).isRecording, isTrue);
    });

    testWidgets('handles transcription failure gracefully', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Start recording
      container.read(voiceStateProvider.notifier).startRecording();
      await tester.pump();

      // Simulate transcription failure
      container.read(voiceStateProvider.notifier).stopRecording();
      await tester.pump();

      // Set error during transcription
      container.read(voiceStateProvider.notifier).setError('Transcription failed');
      await tester.pump();

      // Should show error view
      expect(find.text('Transcription failed'), findsOneWidget);
    });

    testWidgets('handles polishing failure gracefully', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Simulate having transcription but failing during polish
      container.read(voiceStateProvider.notifier).setRawTranscription('Test transcription');
      container.read(voiceStateProvider.notifier).startPolishing();
      await tester.pump();

      // Simulate polishing failure
      container.read(voiceStateProvider.notifier).setError('AI polishing failed');
      await tester.pump();

      // Should show error view
      expect(find.text('AI polishing failed'), findsOneWidget);
    });

    testWidgets('preserves raw transcription on polishing failure', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set up a session with transcription
      container.read(voiceStateProvider.notifier).setRawTranscription('Important text');
      container.read(voiceStateProvider.notifier).startPolishing();
      await tester.pump();

      // Simulate failure
      container.read(voiceStateProvider.notifier).setError('Polishing failed');
      await tester.pump();

      // Raw transcription should still be available
      expect(container.read(voiceStateProvider).rawTranscription, equals('Important text'));
    });
  });

  group('HomeScreen error recovery', () {
    testWidgets('can retry after network error', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Simulate network error
      container.read(voiceStateProvider.notifier).setError('Network connection lost');
      await tester.pump();

      // Verify error state
      expect(find.text('Network connection lost'), findsOneWidget);

      // Tap retry
      await tester.tap(find.byType(VoiceButton));
      await tester.pump();

      // Should be in recording state
      expect(container.read(voiceStateProvider).isRecording, isTrue);
    });

    testWidgets('can cancel recording and start over', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Start recording
      container.read(voiceStateProvider.notifier).startRecording();
      await tester.pump();

      // Long press to cancel
      await tester.longPress(find.byType(VoiceButton));
      await tester.pump();

      // Should be back to idle
      expect(container.read(voiceStateProvider).status, equals(VoiceRecordingStatus.idle));

      // Can start again
      await tester.tap(find.byType(VoiceButton));
      await tester.pump();
      expect(container.read(voiceStateProvider).isRecording, isTrue);
    });

    testWidgets('error state shows appropriate icon color', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set error state
      container.read(voiceStateProvider.notifier).setError('Test error');
      await tester.pump();

      // Find error icon
      final errorIcon = tester.widget<Icon>(
        find.descendant(
          of: find.byType(Center),
          matching: find.byIcon(Icons.error_outline),
        ).first,
      );

      // Icon should use error color
      expect(errorIcon.color, isNotNull);
    });
  });

  group('HomeScreen status messages during errors', () {
    testWidgets('shows correct status message on error', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set error state
      container.read(voiceStateProvider.notifier).setError('Test error');
      await tester.pump();

      // Should show retry message
      expect(find.text('Tap to try again'), findsOneWidget);
    });

    testWidgets('voice button shows error state correctly', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set error state
      container.read(voiceStateProvider.notifier).setError('Test error');
      await tester.pump();

      // Voice button should show error icon
      expect(find.byIcon(Icons.error_outline), findsWidgets);

      // Voice button label should indicate error
      expect(find.text('Error - Tap to retry'), findsOneWidget);
    });
  });
}