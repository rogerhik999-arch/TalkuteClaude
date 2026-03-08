// Tests for translation toggle widget

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/widgets/translation_toggle.dart';

void main() {
  group('TranslationToggle Widget Tests', () {
    testWidgets('TranslationToggle shows toggle switch', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranslationToggle(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have a switch widget
      expect(find.byType(Switch), findsOneWidget);
    });

    testWidgets('TranslationToggle shows label', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranslationToggle(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show translation label
      expect(find.text('Translate'), findsOneWidget);
    });

    testWidgets('TranslationToggle can be toggled on', (tester) async {
      bool isToggled = false;

      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranslationToggle(
                onChanged: (value) {
                  isToggled = value;
                },
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Tap the switch
      await tester.tap(find.byType(Switch));
      await tester.pumpAndSettle();

      expect(isToggled, true);
    });

    testWidgets('TranslationToggle starts with initial value', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranslationToggle(
                initialValue: true,
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Switch should be on
      final switchWidget = tester.widget<Switch>(find.byType(Switch));
      expect(switchWidget.value, true);
    });

    testWidgets('TranslationToggle shows target language when enabled', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranslationToggle(
                initialValue: true,
                targetLanguage: 'Chinese',
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show target language
      expect(find.textContaining('Chinese'), findsOneWidget);
    });

    testWidgets('TranslationToggle hides target language when disabled', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranslationToggle(
                initialValue: false,
                targetLanguage: 'Chinese',
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should not show target language when disabled
      expect(find.textContaining('Chinese'), findsNothing);
    });

    testWidgets('TranslationToggle is disabled when no target language', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranslationToggle(
                enabled: false,
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Switch should be disabled
      final switchWidget = tester.widget<Switch>(find.byType(Switch));
      expect(switchWidget.onChanged, isNull);
    });

    testWidgets('TranslationToggle shows compact mode', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranslationToggle(
                compact: true,
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have an icon button in compact mode
      expect(find.byType(IconButton), findsOneWidget);
    });
  });
}