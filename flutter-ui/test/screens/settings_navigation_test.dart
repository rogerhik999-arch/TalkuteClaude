// Tests for settings screen navigation

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/screens/settings_screen.dart';

void main() {
  group('SettingsScreen Widget Tests', () {
    testWidgets('SettingsScreen has dictionary navigation option', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have dictionary option
      expect(find.text('Personal Dictionary'), findsOneWidget);
      expect(find.text('Manage custom terms and phrases'), findsOneWidget);
    });

    testWidgets('SettingsScreen dictionary option is tappable', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Find and tap dictionary list tile
      final dictionaryTile = find.text('Personal Dictionary');
      expect(dictionaryTile, findsOneWidget);

      await tester.tap(dictionaryTile);
      await tester.pumpAndSettle();

      // Should navigate to dictionary screen
      expect(find.text('No dictionary entries yet'), findsOneWidget);
    });

    testWidgets('SettingsScreen has app bar', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have app bar with title
      expect(find.text('Settings'), findsOneWidget);
    });

    testWidgets('SettingsScreen has other settings sections', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have other settings sections
      expect(find.text('General'), findsOneWidget);
    });

    testWidgets('SettingsScreen dictionary uses correct icon', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have book icon for dictionary
      expect(find.byIcon(Icons.book_outlined), findsOneWidget);
    });
  });
}