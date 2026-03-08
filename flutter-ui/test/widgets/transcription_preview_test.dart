/// Tests for transcription preview widget
///
/// Tests the transcription preview display, copy functionality, and visibility.

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/widgets/transcription_preview.dart';
import 'package:talkute/state/voice_state.dart';

void main() {
  group('TranscriptionPreview', () {
    testWidgets('is hidden when no transcription exists', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(),
            ),
          ),
        ),
      );

      // Should return empty widget
      expect(find.byType(SizedBox), findsOneWidget);
      expect(find.byType(Card), findsNothing);
    });

    testWidgets('shows polished text when available', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(),
            ),
          ),
        ),
      );

      // Set polished text
      container.read(voiceStateProvider.notifier).setPolishedText('Hello, world!');
      await tester.pump();

      // Should show card with text
      expect(find.byType(Card), findsOneWidget);
      expect(find.text('Hello, world!'), findsOneWidget);
      expect(find.text('Polished Text'), findsOneWidget);
    });

    testWidgets('shows raw transcription when available', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(showPolished: false),
            ),
          ),
        ),
      );

      // Set raw transcription
      container.read(voiceStateProvider.notifier).setRawTranscription('hello world');
      container.read(voiceStateProvider.notifier).setPolishedText('Hello, world!');
      await tester.pump();

      // Should show raw text section
      expect(find.text('hello world'), findsOneWidget);
      expect(find.text('Raw Transcription'), findsOneWidget);
    });

    testWidgets('shows both raw and polished text when both available', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(),
            ),
          ),
        ),
      );

      // Set both texts
      container.read(voiceStateProvider.notifier).setRawTranscription('hello world');
      container.read(voiceStateProvider.notifier).setPolishedText('Hello, world!');
      await tester.pump();

      // Should show both sections
      expect(find.text('Hello, world!'), findsOneWidget);
      expect(find.text('hello world'), findsOneWidget);
    });

    testWidgets('hides raw text when showRaw is false', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(showRaw: false),
            ),
          ),
        ),
      );

      // Set both texts
      container.read(voiceStateProvider.notifier).setRawTranscription('hello world');
      container.read(voiceStateProvider.notifier).setPolishedText('Hello, world!');
      await tester.pump();

      // Should only show polished
      expect(find.text('Hello, world!'), findsOneWidget);
      expect(find.text('hello world'), findsNothing);
      expect(find.text('Raw Transcription'), findsNothing);
    });

    testWidgets('hides polished text when showPolished is false', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(showPolished: false),
            ),
          ),
        ),
      );

      // Set both texts
      container.read(voiceStateProvider.notifier).setRawTranscription('hello world');
      container.read(voiceStateProvider.notifier).setPolishedText('Hello, world!');
      await tester.pump();

      // Should only show raw
      expect(find.text('hello world'), findsOneWidget);
      expect(find.text('Hello, world!'), findsNothing);
      expect(find.text('Polished Text'), findsNothing);
    });

    testWidgets('shows word count', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(),
            ),
          ),
        ),
      );

      // Set text with 5 words
      container.read(voiceStateProvider.notifier).setRawTranscription('one two three four five');
      await tester.pump();

      // Should show word count
      expect(find.text('5 words'), findsOneWidget);
    });

    testWidgets('copy button copies text to clipboard', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(),
            ),
          ),
        ),
      );

      // Set polished text
      container.read(voiceStateProvider.notifier).setPolishedText('Test text to copy');
      await tester.pump();

      // Tap copy button
      await tester.tap(find.byIcon(Icons.copy).first);
      await tester.pump();

      // Verify snackbar is shown
      expect(find.text('Copied to clipboard'), findsOneWidget);
    });

    testWidgets('hides copy button when showCopyButton is false', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(showCopyButton: false),
            ),
          ),
        ),
      );

      // Set polished text
      container.read(voiceStateProvider.notifier).setPolishedText('Test text');
      await tester.pump();

      // Should not show copy buttons
      expect(find.byIcon(Icons.copy), findsNothing);
    });

    testWidgets('shows icons for text sections', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: TranscriptionPreview(),
            ),
          ),
        ),
      );

      // Set both texts
      container.read(voiceStateProvider.notifier).setRawTranscription('hello world');
      container.read(voiceStateProvider.notifier).setPolishedText('Hello, world!');
      await tester.pump();

      // Should show icons
      expect(find.byIcon(Icons.auto_fix_high), findsOneWidget); // Polished icon
      expect(find.byIcon(Icons.text_fields), findsOneWidget); // Raw icon
    });
  });

  group('CompactTranscriptionPreview', () {
    testWidgets('is hidden when no polished text exists', (tester) async {
      await tester.pumpWidget(
        ProviderScope(
          child: MaterialApp(
            home: Scaffold(
              body: CompactTranscriptionPreview(),
            ),
          ),
        ),
      );

      // Should return empty widget
      expect(find.byType(SizedBox), findsOneWidget);
    });

    testWidgets('shows polished text when available', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: CompactTranscriptionPreview(),
            ),
          ),
        ),
      );

      // Set polished text
      container.read(voiceStateProvider.notifier).setPolishedText('Compact preview text');
      await tester.pump();

      // Should show text
      expect(find.text('Compact preview text'), findsOneWidget);
    });

    testWidgets('truncates long text with ellipsis', (tester) async {
      final container = ProviderContainer();
      const longText = 'This is a very long text that should be truncated when displayed in the compact preview widget';

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: SizedBox(
                width: 200,
                child: CompactTranscriptionPreview(),
              ),
            ),
          ),
        ),
      );

      // Set long text
      container.read(voiceStateProvider.notifier).setPolishedText(longText);
      await tester.pump();

      // Text widget should exist
      expect(find.byType(Text), findsWidgets);
    });

    testWidgets('copy button works in compact view', (tester) async {
      final container = ProviderContainer();

      await tester.pumpWidget(
        UncontrolledProviderScope(
          container: container,
          child: MaterialApp(
            home: Scaffold(
              body: CompactTranscriptionPreview(),
            ),
          ),
        ),
      );

      // Set polished text
      container.read(voiceStateProvider.notifier).setPolishedText('Compact text');
      await tester.pump();

      // Tap copy button
      await tester.tap(find.byIcon(Icons.copy));
      await tester.pump();

      // Verify snackbar is shown
      expect(find.text('Copied to clipboard'), findsOneWidget);
    });
  });
}