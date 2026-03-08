// Tests for settings language screen

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/screens/settings_screen.dart';
import 'package:talkute/models/language.dart';

void main() {
  group('Settings Language Tests', () {
    testWidgets('SettingsScreen shows language section', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show language section header
      expect(find.text('Language'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows input language option', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show input language option
      expect(find.text('Input Language'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows translation language option', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show translation language option
      expect(find.text('Translation Language'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows current input language', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show English as default
      expect(find.textContaining('English'), findsWidgets);
    });

    testWidgets('SettingsScreen language tap opens selector', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Tap input language
      await tester.tap(find.text('Input Language'));
      await tester.pumpAndSettle();

      // Should show language selector
      expect(find.text('Select Language'), findsOneWidget);
    });

    testWidgets('SettingsScreen can select different language', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Tap input language
      await tester.tap(find.text('Input Language'));
      await tester.pumpAndSettle();

      // Select Chinese (tap on the list tile containing 中文)
      await tester.tap(find.textContaining('中文'));
      await tester.pumpAndSettle();

      // Should close the modal and return to settings
      // The subtitle should now show Chinese
      expect(find.textContaining('Chinese'), findsWidgets);
    });

    testWidgets('SettingsScreen shows auto-translate toggle', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show auto-translate option
      expect(find.text('Auto-translate'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows language icons', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have language icon
      expect(find.byIcon(Icons.language), findsOneWidget);
    });
  });
}