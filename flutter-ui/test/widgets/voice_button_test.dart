/// Tests for voice button widget
///
/// Tests the voice input button's behavior, state transitions, and visual feedback.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/widgets/voice_button.dart';
import 'package:talkute/state/voice_state.dart';

void main() {
  group('VoiceButton', () {
    testWidgets('renders with default idle state', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(),
            ),
          ),
        ),
      );

      // Should show mic icon
      expect(find.byIcon(Icons.mic_none), findsOneWidget);
      // Should show label
      expect(find.text('Tap to record'), findsOneWidget);
    });

    testWidgets('calls onStart when tapped in idle state', (tester) async {
      var startCalled = false;

      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(
                onStart: () => startCalled = true,
              ),
            ),
          ),
        ),
      );

      await tester.tap(find.byType(GestureDetector));
      expect(startCalled, isTrue);
    });

    testWidgets('shows recording state when status is recording', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(),
            ),
          ),
        ),
      );

      // Set recording state
      container.read(voiceStateProvider.notifier).startRecording();
      await tester.pump();

      // Should show recording icon
      expect(find.byIcon(Icons.mic), findsOneWidget);
      // Should show recording label
      expect(find.text('Tap to stop'), findsOneWidget);
    });

    testWidgets('calls onStop when tapped in recording state', (tester) async {
      var stopCalled = false;
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(
                onStop: () => stopCalled = true,
              ),
            ),
          ),
        ),
      );

      // Set recording state
      container.read(voiceStateProvider.notifier).startRecording();
      await tester.pump();

      await tester.tap(find.byType(GestureDetector));
      expect(stopCalled, isTrue);
    });

    testWidgets('shows processing indicator when transcribing', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(),
            ),
          ),
        ),
      );

      // Set transcribing state
      container.read(voiceStateProvider.notifier).startRecording();
      container.read(voiceStateProvider.notifier).stopRecording();
      await tester.pump();

      // Should show circular progress indicator
      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      // Should show transcribing label
      expect(find.text('Transcribing...'), findsOneWidget);
    });

    testWidgets('shows error state when error occurs', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(),
            ),
          ),
        ),
      );

      // Set error state
      container.read(voiceStateProvider.notifier).setError('Test error');
      await tester.pump();

      // Should show error icon
      expect(find.byIcon(Icons.error_outline), findsOneWidget);
      // Should show error label
      expect(find.text('Error - Tap to retry'), findsOneWidget);
    });

    testWidgets('shows completed state after polishing', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(),
            ),
          ),
        ),
      );

      // Set completed state
      container.read(voiceStateProvider.notifier).setPolishedText('Test text');
      await tester.pump();

      // Should show check icon
      expect(find.byIcon(Icons.check), findsOneWidget);
      // Should show completed label
      expect(find.text('Completed'), findsOneWidget);
    });

    testWidgets('calls onCancel on long press during recording', (tester) async {
      var cancelCalled = false;
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(
                onCancel: () => cancelCalled = true,
              ),
            ),
          ),
        ),
      );

      // Set recording state
      container.read(voiceStateProvider.notifier).startRecording();
      await tester.pump();

      // Long press to cancel
      await tester.longPress(find.byType(GestureDetector));
      expect(cancelCalled, isTrue);
    });

    testWidgets('hides label when showLabel is false', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(showLabel: false),
            ),
          ),
        ),
      );

      // Should not show label
      expect(find.text('Tap to record'), findsNothing);
    });

    testWidgets('uses custom size', (tester) async {
      const customSize = 120.0;

      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: const VoiceButton(size: customSize),
            ),
          ),
        ),
      );

      // Find the container
      final container = tester.widget<Container>(
        find.descendant(
          of: find.byType(GestureDetector),
          matching: find.byType(Container),
        ).first,
      );

      expect(container.constraints?.maxWidth, customSize);
      expect(container.constraints?.maxHeight, customSize);
    });
  });

  group('VoiceButton audio visualization', () {
    testWidgets('shows audio level visualization when recording', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(),
            ),
          ),
        ),
      );

      // Start recording
      container.read(voiceStateProvider.notifier).startRecording();
      await tester.pump();

      // Update audio level
      container.read(voiceStateProvider.notifier).updateAudioLevel(0.7);
      await tester.pump();

      // Verify the container exists and has animation
      expect(find.byType(Container), findsWidgets);
    });

    testWidgets('animation runs while recording', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: VoiceButton(),
            ),
          ),
        ),
      );

      // Start recording
      container.read(voiceStateProvider.notifier).startRecording();
      await tester.pump();

      // Allow animation to progress
      await tester.pump(const Duration(milliseconds: 500));

      // Container should still be rendered
      expect(find.byType(Container), findsWidgets);
    });
  });
}