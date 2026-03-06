/// Translation toggle widget for enabling/disabling translation
///
/// Allows users to enable real-time translation of transcribed text.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Translation toggle widget
class TranslationToggle extends ConsumerWidget {
  /// Callback when toggle state changes
  final void Function(bool)? onChanged;

  /// Initial toggle value
  final bool initialValue;

  /// Target language name to display
  final String? targetLanguage;

  /// Whether the toggle is enabled
  final bool enabled;

  /// Whether to show compact mode (icon only)
  final bool compact;

  const TranslationToggle({
    super.key,
    this.onChanged,
    this.initialValue = false,
    this.targetLanguage,
    this.enabled = true,
    this.compact = false,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    if (compact) {
      return _buildCompact(context);
    }
    return _buildFull(context);
  }

  Widget _buildFull(BuildContext context) {
    return ListTile(
      leading: Icon(
        Icons.translate,
        color: enabled
            ? Theme.of(context).colorScheme.primary
            : Theme.of(context).disabledColor,
      ),
      title: const Text('Translate'),
      subtitle: initialValue && targetLanguage != null
          ? Text('To $targetLanguage')
          : null,
      trailing: Switch(
        value: initialValue,
        onChanged: enabled ? onChanged : null,
      ),
      onTap: enabled && onChanged != null
          ? () => onChanged!(!initialValue)
          : null,
    );
  }

  Widget _buildCompact(BuildContext context) {
    return IconButton(
      icon: Icon(
        initialValue ? Icons.translate : Icons.translate_outlined,
        color: initialValue
            ? Theme.of(context).colorScheme.primary
            : Theme.of(context).disabledColor,
      ),
      onPressed: enabled && onChanged != null
          ? () => onChanged!(!initialValue)
          : null,
      tooltip: initialValue && targetLanguage != null
          ? 'Translating to $targetLanguage'
          : 'Enable translation',
    );
  }
}

/// Provider for translation enabled state
final translationEnabledProvider = StateProvider<bool>((ref) => false);

/// Provider for target language
final targetLanguageProvider = StateProvider<String?>((ref) => null);