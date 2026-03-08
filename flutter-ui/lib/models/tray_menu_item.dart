/// Tray menu item model
///
/// Represents an item in the system tray context menu.

/// Tray menu item for the system tray context menu
class TrayMenuItem {
  final String id;
  final String label;
  final bool enabled;
  final bool isSeparator;

  const TrayMenuItem({
    required this.id,
    required this.label,
    this.enabled = true,
    this.isSeparator = false,
  });

  /// Create a separator item
  factory TrayMenuItem.separator() {
    return const TrayMenuItem(
      id: 'separator',
      label: '',
      isSeparator: true,
    );
  }

  /// Create a menu item with quota display
  factory TrayMenuItem.quota(int used, int total) {
    final percentage = total > 0 ? (used / total * 100).round() : 0;
    return TrayMenuItem(
      id: 'quota',
      label: 'Today: $used/$total ($percentage%)',
    );
  }
}