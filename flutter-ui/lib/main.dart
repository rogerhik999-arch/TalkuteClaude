import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'screens/home_screen.dart';
import 'screens/settings_screen.dart';
import 'screens/history_screen.dart';
import 'services/tray_service.dart';
import 'state/session_state.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const ProviderScope(child: TalkuteApp()));
}

class TalkuteApp extends ConsumerStatefulWidget {
  const TalkuteApp({super.key});

  @override
  ConsumerState<TalkuteApp> createState() => _TalkuteAppState();
}

class _TalkuteAppState extends ConsumerState<TalkuteApp> {
  final GlobalKey<NavigatorState> _navigatorKey = GlobalKey<NavigatorState>();

  @override
  void initState() {
    super.initState();
    _initializeServices();
  }

  void _initializeServices() {
    // Initialize tray service with callbacks
    Future.microtask(() {
      final trayService = ref.read(trayServiceProvider);
      trayService.initialize();

      // Wire up tray menu callbacks
      trayService.onSettingsClicked = (_) => _navigateToSettings();
      trayService.onHistoryClicked = (_) => _navigateToHistory();
      trayService.onQuitClicked = (_) => _quitApp();
    });
  }

  void _navigateToSettings() {
    _navigatorKey.currentState?.push(
      MaterialPageRoute(builder: (context) => const SettingsScreen()),
    );
  }

  void _navigateToHistory() {
    _navigatorKey.currentState?.push(
      MaterialPageRoute(builder: (context) => const HistoryScreen()),
    );
  }

  void _quitApp() {
    // TODO: Call Rust FFI to quit the app
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorKey: _navigatorKey,
      title: 'Talkute',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const HomeScreen(),
    );
  }
}

/// App initializer widget that sets up services on startup
class AppInitializer extends ConsumerStatefulWidget {
  final Widget child;

  const AppInitializer({super.key, required this.child});

  @override
  ConsumerState<AppInitializer> createState() => _AppInitializerState();
}

class _AppInitializerState extends ConsumerState<AppInitializer> {
  @override
  void initState() {
    super.initState();
    _initializeServices();
  }

  Future<void> _initializeServices() async {
    // Initialize tray service
    final trayService = ref.read(trayServiceProvider);
    trayService.initialize();

    // TODO: Initialize other services when bridge is ready:
    // - Register global hotkey
    // - Load preferences from Rust FFI
    // - Set up session state listener
  }

  @override
  Widget build(BuildContext context) => widget.child;
}