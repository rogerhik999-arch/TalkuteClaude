// Tests for enhanced settings screen with all preferences

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/screens/settings_screen.dart';

void main() {
  group('Settings Preferences Tests', () {
    testWidgets('SettingsScreen renders correctly', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Basic check - should have Settings title
      expect(find.text('Settings'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows all preference sections', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show General section (visible without scrolling)
      expect(find.text('General'), findsOneWidget);
      expect(find.text('Language'), findsOneWidget);
      expect(find.text('Processing'), findsOneWidget);

      // Scroll to find Voice section
      await tester.scrollUntilVisible(
        find.text('Voice'),
        200,
      );
      expect(find.text('Voice'), findsOneWidget);

      // Scroll to find Data section
      await tester.scrollUntilVisible(
        find.text('Data'),
        200,
      );
      expect(find.text('Data'), findsOneWidget);

      // Scroll to find Privacy section
      await tester.scrollUntilVisible(
        find.text('Privacy'),
        200,
      );
      expect(find.text('Privacy'), findsOneWidget);

      // Scroll to find About section
      await tester.scrollUntilVisible(
        find.text('About'),
        200,
      );
      expect(find.text('About'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows auto-process toggle', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show auto-process option
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

      await tester.pumpAndSettle();

      // Scroll to find filler removal option
      await tester.scrollUntilVisible(
        find.text('Remove fillers'),
        100,
      );

      // Should show filler removal option (with plural 'fillers')
      expect(find.text('Remove fillers'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows context-aware mode toggle', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find context-aware option
      await tester.scrollUntilVisible(
        find.text('Context-aware mode'),
        100,
      );

      // Should show context-aware option
      expect(find.text('Context-aware mode'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows push-to-talk option', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find push-to-talk option in Voice section
      await tester.scrollUntilVisible(
        find.text('Push-to-talk mode'),
        200,
      );

      // Should show push-to-talk option in Voice section
      expect(find.text('Push-to-talk mode'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows keyboard shortcuts option', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find keyboard shortcuts option
      await tester.scrollUntilVisible(
        find.text('Keyboard shortcuts'),
        200,
      );

      // Should show keyboard shortcuts option
      expect(find.text('Keyboard shortcuts'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows crash reporting toggle', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find crash reporting in Privacy section
      await tester.scrollUntilVisible(
        find.text('Crash reporting'),
        300,
      );

      // Should show crash reporting option
      expect(find.text('Crash reporting'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows usage analytics toggle', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find usage analytics in Privacy section
      await tester.scrollUntilVisible(
        find.text('Usage analytics'),
        300,
      );

      // Should show analytics option
      expect(find.text('Usage analytics'), findsOneWidget);
    });

    testWidgets('SettingsScreen shows data management section', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find Data section
      await tester.scrollUntilVisible(
        find.text('Data'),
        300,
      );

      // Should show data section
      expect(find.text('Data'), findsOneWidget);

      // Should show export option
      expect(find.text('Export data'), findsOneWidget);

      // Should show clear data option
      expect(find.textContaining('Clear'), findsWidgets);
    });

    testWidgets('SettingsScreen shows transcription history option', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Should show history option
      expect(find.textContaining('history'), findsWidgets);
    });

    testWidgets('Auto-process toggle can be toggled', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find auto-process option
      await tester.scrollUntilVisible(
        find.text('Auto-process'),
        100,
      );

      // Find the auto-process text
      final autoProcessText = find.text('Auto-process');
      expect(autoProcessText, findsOneWidget);

      // Tap to toggle
      await tester.tap(autoProcessText);
      await tester.pumpAndSettle();
    });

    testWidgets('Filler removal toggle can be toggled', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find filler removal option
      await tester.scrollUntilVisible(
        find.text('Remove fillers'),
        100,
      );

      // Find the filler removal text
      final fillerText = find.text('Remove fillers');
      expect(fillerText, findsOneWidget);

      // Tap to toggle
      await tester.tap(fillerText);
      await tester.pumpAndSettle();
    });

    testWidgets('Context-aware mode toggle can be toggled', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find context-aware option
      await tester.scrollUntilVisible(
        find.text('Context-aware mode'),
        100,
      );

      // Find the context-aware text
      final contextText = find.text('Context-aware mode');
      expect(contextText, findsOneWidget);

      // Tap to toggle
      await tester.tap(contextText);
      await tester.pumpAndSettle();
    });

    testWidgets('Push-to-talk toggle can be toggled', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find push-to-talk option
      await tester.scrollUntilVisible(
        find.text('Push-to-talk mode'),
        200,
      );

      // Find the push-to-talk text
      final pttText = find.text('Push-to-talk mode');
      expect(pttText, findsOneWidget);

      // Tap to toggle
      await tester.tap(pttText);
      await tester.pumpAndSettle();
    });

    testWidgets('Crash reporting toggle can be toggled', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find crash reporting in Privacy section
      await tester.scrollUntilVisible(
        find.text('Crash reporting'),
        300,
      );

      // Find the crash reporting text
      final crashText = find.text('Crash reporting');
      expect(crashText, findsOneWidget);

      // Tap to toggle
      await tester.tap(crashText);
      await tester.pumpAndSettle();
    });

    testWidgets('Settings shows microphone selection', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find microphone option in Voice section
      await tester.scrollUntilVisible(
        find.text('Microphone'),
        200,
      );

      // Should show microphone option
      expect(find.text('Microphone'), findsOneWidget);
    });

    testWidgets('Settings shows noise cancellation option', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find noise cancellation in Voice section
      await tester.scrollUntilVisible(
        find.text('Noise cancellation'),
        200,
      );

      // Should show noise cancellation option
      expect(find.text('Noise cancellation'), findsOneWidget);
    });

    testWidgets('Settings shows privacy policy', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find Privacy Policy at the bottom
      await tester.scrollUntilVisible(
        find.text('Privacy Policy'),
        300,
      );

      // Should show privacy policy
      expect(find.text('Privacy Policy'), findsOneWidget);
    });

    testWidgets('Settings shows terms of service', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find Terms of Service at the bottom
      await tester.scrollUntilVisible(
        find.text('Terms of Service'),
        300,
      );

      // Should show terms of service
      expect(find.text('Terms of Service'), findsOneWidget);
    });

    testWidgets('Settings shows version info', (tester) async {
      await tester.pumpWidget(
        const ProviderScope(
          child: MaterialApp(
            home: SettingsScreen(),
          ),
        ),
      );

      await tester.pumpAndSettle();

      // Scroll to find Version at the bottom
      await tester.scrollUntilVisible(
        find.text('Version'),
        300,
      );

      // Should show version
      expect(find.text('Version'), findsOneWidget);
      expect(find.text('1.0.0'), findsOneWidget);
    });
  });
}
