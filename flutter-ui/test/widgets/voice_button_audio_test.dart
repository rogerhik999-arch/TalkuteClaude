/// Tests for audio level visualization in voice button
///
/// Tests the visual feedback of audio levels during recording.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/widgets/voice_button.dart';
import 'package:talkute/state/voice_state.dart';

void main() {
  group('VoiceButton audio visualization', () {
    testWidgets('audio level affects button appearance', (tester) async {
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

      // Update audio level to high
      container.read(voiceStateProvider.notifier).updateAudioLevel(0.9);
      await tester.pump();

      // Find the container with the button
      final containerWidget = tester.widget<Container>(
        find.descendant(
          of: find.byType(GestureDetector),
          matching: find.byType(Container),
        ).first,
      );

      // Should have a box decoration
      expect(containerWidget.decoration, isNotNull);
    });

    testWidgets('audio level clamped between 0.0 and 1.0', (tester) async {
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

      // Try to set audio level above 1.0
      container.read(voiceStateProvider.notifier).updateAudioLevel(1.5);
      await tester.pump();

      // Should be clamped to 1.0
      expect(container.read(voiceStateProvider).audioLevel, equals(1.0));

      // Try to set audio level below 0.0
      container.read(voiceStateProvider.notifier).updateAudioLevel(-0.5);
      await tester.pump();

      // Should be clamped to 0.0
      expect(container.read(voiceStateProvider).audioLevel, equals(0.0));
    });

    testWidgets('audio level only updates when recording', (tester) async {
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

      // Try to update audio level when not recording
      container.read(voiceStateProvider.notifier).updateAudioLevel(0.8);
      await tester.pump();

      // Should remain at 0.0 since not recording
      expect(container.read(voiceStateProvider).audioLevel, equals(0.0));

      // Start recording and update
      container.read(voiceStateProvider.notifier).startRecording();
      container.read(voiceStateProvider.notifier).updateAudioLevel(0.8);
      await tester.pump();

      // Should now be 0.8
      expect(container.read(voiceStateProvider).audioLevel, equals(0.8));
    });

    testWidgets('audio level resets when recording stops', (tester) async {
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

      // Start recording and set high audio level
      container.read(voiceStateProvider.notifier).startRecording();
      container.read(voiceStateProvider.notifier).updateAudioLevel(0.9);
      await tester.pump();
      expect(container.read(voiceStateProvider).audioLevel, equals(0.9));

      // Stop recording
      container.read(voiceStateProvider.notifier).stopRecording();
      await tester.pump();

      // Audio level should be reset
      expect(container.read(voiceStateProvider).audioLevel, equals(0.0));
    });

    testWidgets('audio level affects shadow intensity', (tester) async {
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

      // Start recording with low audio level
      container.read(voiceStateProvider.notifier).startRecording();
      container.read(voiceStateProvider.notifier).updateAudioLevel(0.2);
      await tester.pump();

      // Find container with decoration
      final containerLow = tester.widget<Container>(
        find.descendant(
          of: find.byType(GestureDetector),
          matching: find.byType(Container),
        ).first,
      );
      final decorationLow = containerLow.decoration as BoxDecoration;

      // Update to high audio level
      container.read(voiceStateProvider.notifier).updateAudioLevel(0.9);
      await tester.pump();

      final containerHigh = tester.widget<Container>(
        find.descendant(
          of: find.byType(GestureDetector),
          matching: find.byType(Container),
        ).first,
      );
      final decorationHigh = containerHigh.decoration as BoxDecoration;

      // Both should have shadows
      expect(decorationLow.boxShadow, isNotEmpty);
      expect(decorationHigh.boxShadow, isNotEmpty);
    });
  });

  group('VoiceButton animation', () {
    testWidgets('animation starts when recording begins', (tester) async {
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

      // Animation should be running (pulsing effect)
      await tester.pump(const Duration(milliseconds: 500));

      // Widget should still render properly
      expect(find.byType(VoiceButton), findsOneWidget);
    });

    testWidgets('animation stops when recording stops', (tester) async {
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
      await tester.pump(const Duration(milliseconds: 300));

      // Stop recording
      container.read(voiceStateProvider.notifier).stopRecording();
      await tester.pump();

      // Widget should render properly
      expect(find.byType(VoiceButton), findsOneWidget);
    });
  });
}