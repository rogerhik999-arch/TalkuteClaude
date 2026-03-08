/// Tray Service
///
/// Manages the system tray icon and menu.

import 'dart:async';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/tray_menu_item.dart';
import '../state/session_state.dart';

/// Callback type for tray menu actions
typedef TrayMenuCallback = void Function(String menuItemId);

/// Tray service provider
final trayServiceProvider = Provider<TrayService>((ref) {
  return TrayService(ref);
});

/// Service for managing the system tray
class TrayService {
  final Ref _ref;
  StreamSubscription<SessionData>? _sessionSubscription;

  /// Callback when settings menu item is clicked
  TrayMenuCallback? onSettingsClicked;

  /// Callback when history menu item is clicked
  TrayMenuCallback? onHistoryClicked;

  /// Callback when quit menu item is clicked
  TrayMenuCallback? onQuitClicked;

  /// Callback when idle timeout is changed
  void Function(double seconds)? onIdleTimeoutChanged;

  TrayService(this._ref);

  /// Initialize tray service and subscribe to session state
  void initialize() {
    _sessionSubscription = _ref
        .listen<SessionData>(sessionStateNotifierProvider, (previous, next) {
      _updateIconFromState(next.state);
    }).close;

    // Set initial icon state
    _updateIconFromState(SessionState.idle);
  }

  /// Dispose of resources
  void dispose() {
    _sessionSubscription?.cancel();
  }

  /// Update tray icon based on session state
  void _updateIconFromState(SessionState state) {
    final iconName = switch (state) {
      SessionState.idle => 'idle',
      SessionState.recording => 'recording',
      SessionState.processing => 'processing',
      SessionState.error => 'error',
    };
    setTrayIcon(iconName);
  }

  /// Build the default tray menu items
  List<TrayMenuItem> buildMenuItems({
    required int quotaUsed,
    required int quotaTotal,
    required String hotkey,
    required double idleTimeoutSeconds,
  }) {
    return [
      TrayMenuItem.quota(quotaUsed, quotaTotal),
      TrayMenuItem.separator(),
      TrayMenuItem(
        id: 'hotkey',
        label: 'Hotkey: $hotkey',
        enabled: false,
      ),
      TrayMenuItem(
        id: 'idle_timeout',
        label: 'Idle timeout: ${idleTimeoutSeconds.toStringAsFixed(1)}s',
      ),
      TrayMenuItem.separator(),
      TrayMenuItem(
        id: 'settings',
        label: 'Settings',
      ),
      TrayMenuItem(
        id: 'history',
        label: 'History',
      ),
      TrayMenuItem.separator(),
      TrayMenuItem(
        id: 'quit',
        label: 'Quit',
      ),
    ];
  }

  /// Handle menu item click
  void handleMenuClick(String menuItemId) {
    switch (menuItemId) {
      case 'settings':
        onSettingsClicked?.call(menuItemId);
      case 'history':
        onHistoryClicked?.call(menuItemId);
      case 'quit':
        onQuitClicked?.call(menuItemId);
      case 'idle_timeout':
        // Idle timeout change would be handled by a slider UI
        break;
    }
  }

  /// Update the tray icon based on state string
  Future<void> setTrayIcon(String state) async {
    // TODO: Call Rust FFI set_tray_icon via bridge
    // For now, this is a placeholder that will be wired up
    // when flutter_rust_bridge generates the bindings
  }

  /// Update a menu item's label
  Future<void> updateMenuItem(String id, String newLabel) async {
    // TODO: Call Rust FFI update_menu_item via bridge
  }

  /// Show a notification
  Future<void> showNotification(String title, String message) async {
    // TODO: Call Rust FFI show_tray_notification via bridge
  }

  /// Update quota display in tray menu
  void updateQuota(int used, int total) {
    updateMenuItem('quota', 'Today: $used/$total');
  }

  /// Update hotkey display in tray menu
  void updateHotkey(String hotkey) {
    updateMenuItem('hotkey', 'Hotkey: $hotkey');
  }

  /// Update idle timeout display in tray menu
  void updateIdleTimeout(double seconds) {
    updateMenuItem('idle_timeout', 'Idle timeout: ${seconds.toStringAsFixed(1)}s');
  }
}