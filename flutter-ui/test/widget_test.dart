// Smoke test for Talkute app
//
// Basic test to verify the app starts correctly.

import 'package:flutter_test/flutter_test.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:talkute/main.dart';

void main() {
  testWidgets('Talkute app starts correctly', (WidgetTester tester) async {
    // Build our app with ProviderScope and trigger a frame.
    await tester.pumpWidget(
      const ProviderScope(child: TalkuteApp()),
    );

    // Verify that the app title is displayed
    expect(find.text('Talkute'), findsOneWidget);
  });

  testWidgets('App has correct theme', (WidgetTester tester) async {
    // Build our app with ProviderScope
    await tester.pumpWidget(
      const ProviderScope(child: TalkuteApp()),
    );

    // Verify MaterialApp exists
    expect(find.byType(MaterialApp), findsOneWidget);

    // Verify theme has color scheme
    final materialApp = tester.widget<MaterialApp>(find.byType(MaterialApp));
    expect(materialApp.theme, isNotNull);
    expect(materialApp.theme?.colorScheme, isNotNull);
  });
}