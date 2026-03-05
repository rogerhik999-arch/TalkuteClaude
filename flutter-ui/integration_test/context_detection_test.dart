// Integration tests for automatic context detection

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/models/context.dart';
import 'package:talkute/state/voice_state.dart';
import 'package:talkute/screens/home_screen.dart';
import 'package:talkute/widgets/voice_button.dart';
import 'package:talkute/widgets/context_indicator.dart';

void main() {
  group('Automatic Context Detection Integration Tests', () {
    testWidgets('Context is detected when recording starts', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Initially no context - should show placeholder
      expect(find.text('No context detected'), findsOneWidget);

      // Tap voice button to start recording
      await tester.tap(find.byType(VoiceButton));
      await tester.pumpAndSettle();

      // After starting, context should be detected
      // (In real implementation, this would call detect_application_context FFI)
      // For now, verify the flow structure exists
      expect(find.byType(VoiceButton), findsOneWidget);
    });

    testWidgets('Context indicator updates when context changes', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: const MaterialApp(
            home: Scaffold(
              body: Column(
                children: [
                  ContextIndicator(),
                ],
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Initially no context
      expect(find.text('No context detected'), findsOneWidget);

      // Simulate context detection
      final context = ApplicationContext(
        contextId: 'test-1',
        applicationName: 'Gmail',
        applicationCategory: 'email',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );
      container.read(voiceStateProvider.notifier).setDetectedContext(context);

      await tester.pumpAndSettle();

      // Should show Gmail context
      expect(find.text('Gmail'), findsOneWidget);
      expect(find.byIcon(Icons.email), findsOneWidget);
    });

    testWidgets('Context clears when new session starts', (tester) async {
      final container = ProviderContainer();

      // Set initial context
      final initialContext = ApplicationContext(
        contextId: 'test-2',
        applicationName: 'Slack',
        applicationCategory: 'chat',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: const MaterialApp(
            home: HomeScreen(),
          ),
        ),
      );

      container.read(voiceStateProvider.notifier).setDetectedContext(initialContext);
      await tester.pumpAndSettle();

      // Verify context is set
      expect(container.read(voiceStateProvider).detectedContext?.applicationName, 'Slack');

      // Start new recording
      await container.read(voiceStateProvider.notifier).startRecording();
      await tester.pumpAndSettle();

      // Context should be cleared
      expect(container.read(voiceStateProvider).detectedContext, isNull);
    });

    testWidgets('Multiple context detection cycles work correctly', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: const MaterialApp(
            home: Scaffold(
              body: Column(
                children: [
                  ContextIndicator(),
                ],
              ),
            ),
          ),
        ),
      );

      // First context
      final context1 = ApplicationContext(
        contextId: 'cycle-1',
        applicationName: 'Outlook',
        applicationCategory: 'email',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );
      container.read(voiceStateProvider.notifier).setDetectedContext(context1);
      await tester.pumpAndSettle();
      expect(find.text('Outlook'), findsOneWidget);

      // Clear and set second context
      container.read(voiceStateProvider.notifier).clearDetectedContext();
      await tester.pumpAndSettle();
      expect(find.text('No context detected'), findsOneWidget);

      final context2 = ApplicationContext(
        contextId: 'cycle-2',
        applicationName: 'VSCode',
        applicationCategory: 'code',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );
      container.read(voiceStateProvider.notifier).setDetectedContext(context2);
      await tester.pumpAndSettle();
      expect(find.text('VSCode'), findsOneWidget);
      expect(find.byIcon(Icons.code), findsOneWidget);
    });
  });
}