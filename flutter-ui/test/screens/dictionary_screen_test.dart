// Tests for dictionary screen

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/models/dictionary.dart';
import 'package:talkute/screens/dictionary_screen.dart';

void main() {
  group('DictionaryScreen Widget Tests', () {
    testWidgets('DictionaryScreen shows empty state when no entries', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: DictionaryScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show empty state message
      expect(find.text('No dictionary entries yet'), findsOneWidget);
      expect(find.text('Tap + to add your first custom term'), findsOneWidget);
    });

    testWidgets('DictionaryScreen shows entries list', (tester) async {
      final entries = [
        PersonalDictionaryEntry(
          entryId: '1',
          deviceId: 'test-device',
          phrase: 'API',
          replacement: 'Application Programming Interface',
          category: DictionaryCategory.technical,
          createdAt: DateTime.now(),
        ),
      ];

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            dictionaryEntriesProvider.overrideWith((ref) => entries),
          ],
          child: const MaterialApp(
            home: DictionaryScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show the entry
      expect(find.text('API'), findsOneWidget);
      expect(find.text('Application Programming Interface'), findsOneWidget);
    });

    testWidgets('DictionaryScreen has add button', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: DictionaryScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should have floating action button
      expect(find.byType(FloatingActionButton), findsOneWidget);
      expect(find.byIcon(Icons.add), findsOneWidget);
    });

    testWidgets('DictionaryScreen shows category badge', (tester) async {
      final entries = [
        PersonalDictionaryEntry(
          entryId: '1',
          deviceId: 'test-device',
          phrase: 'API',
          replacement: 'Application Programming Interface',
          category: DictionaryCategory.technical,
          createdAt: DateTime.now(),
        ),
      ];

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            dictionaryEntriesProvider.overrideWith((ref) => entries),
          ],
          child: const MaterialApp(
            home: DictionaryScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show category badge
      expect(find.text('Technical'), findsOneWidget);
    });

    testWidgets('DictionaryScreen entry is tappable', (tester) async {
      final entries = [
        PersonalDictionaryEntry(
          entryId: '1',
          deviceId: 'test-device',
          phrase: 'API',
          replacement: 'Application Programming Interface',
          category: DictionaryCategory.technical,
          createdAt: DateTime.now(),
        ),
      ];

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            dictionaryEntriesProvider.overrideWith((ref) => entries),
          ],
          child: const MaterialApp(
            home: DictionaryScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Find and tap the list tile
      expect(find.byType(ListTile), findsOneWidget);
    });

    testWidgets('DictionaryScreen shows multiple entries', (tester) async {
      final entries = [
        PersonalDictionaryEntry(
          entryId: '1',
          deviceId: 'test-device',
          phrase: 'API',
          replacement: 'Application Programming Interface',
          category: DictionaryCategory.technical,
          createdAt: DateTime.now(),
        ),
        PersonalDictionaryEntry(
          entryId: '2',
          deviceId: 'test-device',
          phrase: 'CI/CD',
          replacement: 'Continuous Integration/Continuous Deployment',
          category: DictionaryCategory.technical,
          createdAt: DateTime.now(),
        ),
      ];

      await tester.pumpWidget(
        ProviderScope(
          overrides: [
            dictionaryEntriesProvider.overrideWith((ref) => entries),
          ],
          child: const MaterialApp(
            home: DictionaryScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show both entries
      expect(find.text('API'), findsOneWidget);
      expect(find.text('CI/CD'), findsOneWidget);
    });
  });

  group('PersonalDictionaryEntry Model Tests', () {
    test('PersonalDictionaryEntry creates correctly', () {
      final entry = PersonalDictionaryEntry(
        entryId: 'test-id',
        deviceId: 'test-device',
        phrase: 'API',
        replacement: 'Application Programming Interface',
        category: DictionaryCategory.technical,
        createdAt: DateTime.now(),
      );

      expect(entry.entryId, 'test-id');
      expect(entry.phrase, 'API');
      expect(entry.replacement, 'Application Programming Interface');
      expect(entry.category, DictionaryCategory.technical);
      expect(entry.caseSensitive, false);
      expect(entry.wholeWordOnly, true);
    });

    test('PersonalDictionaryEntry copyWith works', () {
      final entry = PersonalDictionaryEntry(
        entryId: 'test-id',
        deviceId: 'test-device',
        phrase: 'API',
        replacement: 'Application Programming Interface',
        category: DictionaryCategory.technical,
        createdAt: DateTime.now(),
      );

      final updated = entry.copyWith(phrase: 'REST API');

      expect(updated.phrase, 'REST API');
      expect(updated.entryId, 'test-id');
      expect(updated.replacement, 'Application Programming Interface');
    });

    test('DictionaryCategory fromString works', () {
      expect(DictionaryCategory.fromString('technical'), DictionaryCategory.technical);
      expect(DictionaryCategory.fromString('business'), DictionaryCategory.business);
      expect(DictionaryCategory.fromString('medical'), DictionaryCategory.medical);
      expect(DictionaryCategory.fromString('general'), DictionaryCategory.general);
      expect(DictionaryCategory.fromString('unknown'), DictionaryCategory.general);
    });

    test('DictionaryCategory displayName works', () {
      expect(DictionaryCategory.technical.displayName, 'Technical');
      expect(DictionaryCategory.business.displayName, 'Business');
      expect(DictionaryCategory.medical.displayName, 'Medical');
      expect(DictionaryCategory.general.displayName, 'General');
    });
  });
}