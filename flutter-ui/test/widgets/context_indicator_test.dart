// Tests for context indicator widget

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/models/context.dart';
import 'package:talkute/state/voice_state.dart';
import 'package:talkute/widgets/context_indicator.dart';

void main() {
  group('ContextIndicator Widget Tests', () {
    testWidgets('ContextIndicator renders without context', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: ContextIndicator(),
            ),
          ),
        ),
      );

      // Should show placeholder when no context
      expect(find.text('No context detected'), findsOneWidget);
    });

    testWidgets('ContextIndicator shows application name', (tester) async {
      final context = ApplicationContext(
        contextId: '1',
        applicationName: 'Slack',
        applicationCategory: 'chat',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            voiceStateProvider.overrideWith((ref) {
              return VoiceStateNotifier()..setDetectedContext(context);
            }),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: ContextIndicator(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show application name
      expect(find.text('Slack'), findsOneWidget);
    });

    testWidgets('ContextIndicator shows category icon for email', (tester) async {
      final context = ApplicationContext(
        contextId: '2',
        applicationName: 'Gmail',
        applicationCategory: 'email',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            voiceStateProvider.overrideWith((ref) {
              return VoiceStateNotifier()..setDetectedContext(context);
            }),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: ContextIndicator(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have email icon
      expect(find.byIcon(Icons.email), findsOneWidget);
    });

    testWidgets('ContextIndicator shows category icon for chat', (tester) async {
      final context = ApplicationContext(
        contextId: '3',
        applicationName: 'Discord',
        applicationCategory: 'chat',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            voiceStateProvider.overrideWith((ref) {
              return VoiceStateNotifier()..setDetectedContext(context);
            }),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: ContextIndicator(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have chat icon
      expect(find.byIcon(Icons.chat), findsOneWidget);
    });

    testWidgets('ContextIndicator shows category icon for code', (tester) async {
      final context = ApplicationContext(
        contextId: '4',
        applicationName: 'VSCode',
        applicationCategory: 'code',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            voiceStateProvider.overrideWith((ref) {
              return VoiceStateNotifier()..setDetectedContext(context);
            }),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: ContextIndicator(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have code icon
      expect(find.byIcon(Icons.code), findsOneWidget);
    });

    testWidgets('ContextIndicator shows category icon for document', (tester) async {
      final context = ApplicationContext(
        contextId: '5',
        applicationName: 'Word',
        applicationCategory: 'document',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            voiceStateProvider.overrideWith((ref) {
              return VoiceStateNotifier()..setDetectedContext(context);
            }),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: ContextIndicator(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have document icon
      expect(find.byIcon(Icons.description), findsOneWidget);
    });

    testWidgets('ContextIndicator shows category icon for browser', (tester) async {
      final context = ApplicationContext(
        contextId: '6',
        applicationName: 'Chrome',
        applicationCategory: 'browser',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            voiceStateProvider.overrideWith((ref) {
              return VoiceStateNotifier()..setDetectedContext(context);
            }),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: ContextIndicator(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have browser icon
      expect(find.byIcon(Icons.language), findsOneWidget);
    });

    testWidgets('ContextIndicator is tappable', (tester) async {
      final context = ApplicationContext(
        contextId: '7',
        applicationName: 'Slack',
        applicationCategory: 'chat',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            voiceStateProvider.overrideWith((ref) {
              return VoiceStateNotifier()..setDetectedContext(context);
            }),
          ],
          child: const MaterialApp(
            home: Scaffold(
              body: ContextIndicator(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should be tappable
      expect(find.byType(InkWell), findsOneWidget);
    });
  });
}