/// Integration tests for home screen
///
/// Tests the complete voice input workflow from UI perspective.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/screens/home_screen.dart';
import 'package:talkute/state/voice_state.dart';
import 'package:talkute/widgets/voice_button.dart';
import 'package:talkute/widgets/transcription_preview.dart';

void main() {
  group('HomeScreen', () {
    testWidgets('renders all main components', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Should have app bar with title
      expect(find.text('Talkute'), findsOneWidget);

      // Should have settings and history buttons
      expect(find.byIcon(Icons.settings), findsOneWidget);
      expect(find.byIcon(Icons.history), findsOneWidget);

      // Should have voice button
      expect(find.byType(VoiceButton), findsOneWidget);

      // Should have idle view content
      expect(find.text('Tap the button below to start'), findsOneWidget);
    });

    testWidgets('shows context indicator', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Should show context indicator
      expect(find.byIcon(Icons.apps), findsOneWidget);
      expect(find.text('Ready'), findsOneWidget);
    });

    testWidgets('shows quota indicator', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Should show quota indicator
      expect(find.byIcon(Icons.data_usage), findsOneWidget);
      expect(find.text('0 / 4000 words this week'), findsOneWidget);
    });

    testWidgets('transitions to recording state when button tapped', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Tap voice button
      await tester.tap(find.byType(VoiceButton));
      await tester.pump();

      // State should be recording
      expect(container.read(voiceStateProvider).isRecording, isTrue);

      // Should show recording status
      expect(find.text('Recording... Long press to cancel'), findsOneWidget);
    });

    testWidgets('shows transcription preview after processing', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set completed state with transcription
      container.read(voiceStateProvider.notifier).setRawTranscription('Test transcription');
      container.read(voiceStateProvider.notifier).setPolishedText('Test transcription.');
      await tester.pump();

      // Should show transcription preview
      expect(find.byType(TranscriptionPreview), findsOneWidget);
      expect(find.text('Test transcription.'), findsOneWidget);
    });

    testWidgets('shows error view when error occurs', (tester) async {
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
      container.read(voiceStateProvider.notifier).setError('Test error message');
      await tester.pump();

      // Should show error view
      expect(find.byIcon(Icons.error_outline), findsWidgets);
      expect(find.text('Something went wrong'), findsOneWidget);
      expect(find.text('Test error message'), findsOneWidget);
    });

    testWidgets('settings button shows snackbar', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Tap settings button
      await tester.tap(find.byIcon(Icons.settings));
      await tester.pump();

      // Should show snackbar
      expect(find.text('Settings coming soon'), findsOneWidget);
    });

    testWidgets('history button shows snackbar', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Tap history button
      await tester.tap(find.byIcon(Icons.history));
      await tester.pump();

      // Should show snackbar
      expect(find.text('History coming soon'), findsOneWidget);
    });

    testWidgets('complete voice input workflow', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Initially idle
      expect(container.read(voiceStateProvider).status, VoiceRecordingStatus.idle);

      // Start recording
      await tester.tap(find.byType(VoiceButton));
      await tester.pump();
      expect(container.read(voiceStateProvider).isRecording, isTrue);

      // Stop recording (simulated)
      await tester.tap(find.byType(VoiceButton));
      await tester.pump(const Duration(seconds: 3));

      // Should eventually show completed state
      expect(
        container.read(voiceStateProvider).status == VoiceRecordingStatus.transcribing ||
        container.read(voiceStateProvider).status == VoiceRecordingStatus.completed,
        isTrue,
      );
    });

    testWidgets('cancel recording on long press', (tester) async {
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
      await tester.tap(find.byType(VoiceButton));
      await tester.pump();
      expect(container.read(voiceStateProvider).isRecording, isTrue);

      // Long press to cancel
      await tester.longPress(find.byType(VoiceButton));
      await tester.pump();

      // Should be idle again
      expect(container.read(voiceStateProvider).status, VoiceRecordingStatus.idle);
    });
  });

  group('HomeScreen accessibility', () {
    testWidgets('voice button has appropriate semantics', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Voice button should be tappable
      expect(find.byType(GestureDetector), findsWidgets);
    });

    testWidgets('copy buttons have tooltips', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      // Set transcription
      container.read(voiceStateProvider.notifier).setPolishedText('Test text');
      await tester.pump();

      // Copy button should have tooltip
      expect(find.byTooltip('Copy to clipboard'), findsWidgets);
    });
  });

  group('HomeScreen status messages', () {
    testWidgets('shows correct status for each state', (tester) async {
      final container = ProviderContainer();
      final statusMessages = {
        VoiceRecordingStatus.idle: 'Ready to record',
        VoiceRecordingStatus.recording: 'Recording... Long press to cancel',
        VoiceRecordingStatus.transcribing: 'Converting speech to text...',
        VoiceRecordingStatus.polishing: 'Polishing your text...',
        VoiceRecordingStatus.completed: 'Done! Tap to start new recording',
        VoiceRecordingStatus.error: 'Tap to try again',
      };

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      for (final entry in statusMessages.entries) {
        // Reset and set state
        container.read(voiceStateProvider.notifier).reset();

        if (entry.key == VoiceRecordingStatus.recording) {
          container.read(voiceStateProvider.notifier).startRecording();
        } else if (entry.key == VoiceRecordingStatus.transcribing) {
          container.read(voiceStateProvider.notifier).startRecording();
          container.read(voiceStateProvider.notifier).stopRecording();
        } else if (entry.key == VoiceRecordingStatus.polishing) {
          container.read(voiceStateProvider.notifier).startRecording();
          container.read(voiceStateProvider.notifier).stopRecording();
          container.read(voiceStateProvider.notifier).startPolishing();
        } else if (entry.key == VoiceRecordingStatus.completed) {
          container.read(voiceStateProvider.notifier).setPolishedText('Test');
        } else if (entry.key == VoiceRecordingStatus.error) {
          container.read(voiceStateProvider.notifier).setError('Test error');
        }

        await tester.pump();
        expect(find.text(entry.value), findsOneWidget, reason: 'Status message for ${entry.key}');
      }
    });
  });
}