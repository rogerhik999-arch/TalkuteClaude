//! SettingsScreen widget tests

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/screens/settings_screen.dart';

void main() {
  group('SettingsScreen Widget Tests', () {
    testWidgets('SettingsScreen renders all sections', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      expect(find.text('General'), findsOneWidget);
      expect(find.text('Voice'), findsOneWidget);
      expect(find.text('Language'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows auto-process toggle', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      expect(find.text('Auto-process'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows filler removal toggle', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      expect(find.text('Remove fillers'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows microphone selection', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      expect(find.text('Microphone'), findsOneWidget);
    });
  });
}