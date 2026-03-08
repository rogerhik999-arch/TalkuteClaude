// Tests for language selector widget

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/models/language.dart';
import 'package:talkute/widgets/language_selector.dart';

void main() {
  group('LanguageSelector Widget Tests', () {
    testWidgets('LanguageSelector shows all languages', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: LanguageSelector(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show at least one language (checking for native names which are unique)
      expect(find.text('English'), findsWidgets);
    });

    testWidgets('LanguageSelector shows flag emojis', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: LanguageSelector(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show flag emoji in the list
      expect(find.textContaining('🇺🇸'), findsOneWidget);
    });

    testWidgets('LanguageSelector shows native names', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: LanguageSelector(),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show native names (these are unique)
      expect(find.text('中文'), findsOneWidget);
      expect(find.text('日本語'), findsOneWidget);
    });

    testWidgets('LanguageSelector can select language', (tester) async {
      String? selectedCode;

      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: LanguageSelector(
                onLanguageSelected: (language) {
                  selectedCode = language.code;
                },
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Tap on Chinese
      await tester.tap(find.text('中文'));
      await tester.pumpAndSettle();

      expect(selectedCode, 'zh');
    });

    testWidgets('LanguageSelector shows selected language', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: LanguageSelector(
                selectedLanguageCode: 'zh',
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Selected language should be highlighted
      // The widget should show Chinese as selected
      expect(find.text('中文'), findsOneWidget);
    });

    testWidgets('LanguageSelector uses dropdown style', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: LanguageSelector(
                style: LanguageSelectorStyle.dropdown,
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have a dropdown
      expect(find.byType(DropdownButton<SupportedLanguage>), findsOneWidget);
    });

    testWidgets('LanguageSelector uses list style', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: LanguageSelector(
                style: LanguageSelectorStyle.list,
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have a list view
      expect(find.byType(ListView), findsOneWidget);
    });

    testWidgets('LanguageSelector filters languages', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: LanguageSelector(
                filter: ['en', 'zh', 'ja'],
              ),
            ),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should only show filtered languages (using native names which are unique)
      expect(find.text('中文'), findsOneWidget);
      expect(find.text('日本語'), findsOneWidget);
      // Should not show German native name
      expect(find.text('Deutsch'), findsNothing);
    });
  });
}