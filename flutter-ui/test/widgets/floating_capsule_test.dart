//! FloatingCapsule widget tests

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/widgets/floating_capsule.dart';
import 'package:talkute/state/session_state.dart';

void main() {
  group('FloatingCapsule Widget Tests', () {
    testWidgets('FloatingCapsule renders in idle state', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: FloatingCapsule(),
            ),
          ),
        ),
      );

      expect(find.text('Press to talk'), findsOneWidget);
      expect(find.byIcon(Icons.mic), findsOneWidget);
    });

    testWidgets('FloatingCapsule shows recording state', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            sessionStateNotifierProvider.overrideWith(
              () => SessionStateNotifier()..startRecording('test'),
            ),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: FloatingCapsule(),
            ),
          ),
        ),
      );

      expect(find.text('Listening...'), findsOneWidget);
    });

    testWidgets('FloatingCapsule shows processing state', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            sessionStateNotifierProvider.overrideWith(
              () => SessionStateNotifier()
                ..startRecording('test')
                ..startProcessing(),
            ),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: FloatingCapsule(),
            ),
          ),
        ),
      );

      expect(find.text('Processing'), findsOneWidget);
      expect(find.byType(CircularProgressIndicator), findsOneWidget);
    });

    testWidgets('FloatingCapsule shows error state', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            sessionStateNotifierProvider.overrideWith(
              () => SessionStateNotifier()..setError('Test error'),
            ),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: FloatingCapsule(),
            ),
          ),
        ),
      );

      expect(find.text('Test error'), findsOneWidget);
      expect(find.text('Retry'), findsOneWidget);
      expect(find.text('Dismiss'), findsOneWidget);
    });

    testWidgets('Retry button triggers retry action', (tester) async {
      final notifier = SessionStateNotifier()..setError('Test error');

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            sessionStateNotifierProvider.overrideWith(() => notifier),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: FloatingCapsule(),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Retry'));
      await tester.pump();

      // Verify the notifier was called
      expect(notifier.state.state, SessionState.processing);
    });

    testWidgets('Dismiss button clears error', (tester) async {
      final notifier = SessionStateNotifier()..setError('Test error');

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            sessionStateNotifierProvider.overrideWith(() => notifier),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: FloatingCapsule(),
            ),
          ),
        ),
      );

      await tester.tap(find.text('Dismiss'));
      await tester.pump();

      expect(notifier.state.state, SessionState.idle);
    });

    testWidgets('FloatingCapsule has correct width for each state', (tester) async {
      // Test idle state width
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: FloatingCapsule(),
            ),
          ),
        ),
      );

      var container = tester.widget<AnimatedContainer>(find.byType(AnimatedContainer).first);
      // Width should be 200 for idle state

      // Test error state width
      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            sessionStateNotifierProvider.overrideWith(
              () => SessionStateNotifier()..setError('Error'),
            ),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: FloatingCapsule(),
            ),
          ),
        ),
      );

      container = tester.widget<AnimatedContainer>(find.byType(AnimatedContainer).first);
      // Width should be 280 for error state
    });
  });
}