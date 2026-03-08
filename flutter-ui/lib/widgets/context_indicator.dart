/// Context indicator widget
///
/// Displays the detected application context with an appropriate icon.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../state/voice_state.dart';

/// Context indicator widget showing the detected application context
class ContextIndicator extends ConsumerWidget {
  const ContextIndicator({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final voiceState = ref.watch(voiceStateProvider);
    final detectedContext = voiceState.detectedContext;

    if (detectedContext == null) {
      return const Text(
        'No context detected',
        style: TextStyle(
          color: Colors.grey,
          fontSize: 12,
        ),
      );
    }

    final categoryIcon = _getIconForCategory(detectedContext.applicationCategory);
    final categoryColor = _getColorForCategory(detectedContext.applicationCategory);

    return InkWell(
      onTap: () {
        // TODO: Show context selection dialog
      },
      borderRadius: BorderRadius.circular(8),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
        decoration: BoxDecoration(
          color: categoryColor.withValues(alpha: 0.1),
          borderRadius: BorderRadius.circular(16),
          border: Border.all(
            color: categoryColor.withValues(alpha: 0.3),
          ),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              categoryIcon,
              size: 16,
              color: categoryColor,
            ),
            const SizedBox(width: 6),
            Text(
              detectedContext.applicationName,
              style: TextStyle(
                color: categoryColor,
                fontSize: 12,
                fontWeight: FontWeight.w500,
              ),
            ),
          ],
        ),
      ),
    );
  }

  IconData _getIconForCategory(String category) {
    switch (category.toLowerCase()) {
      case 'email':
        return Icons.email;
      case 'chat':
        return Icons.chat;
      case 'code':
        return Icons.code;
      case 'document':
        return Icons.description;
      case 'browser':
        return Icons.language;
      default:
        return Icons.apps;
    }
  }

  Color _getColorForCategory(String category) {
    switch (category.toLowerCase()) {
      case 'email':
        return Colors.blue;
      case 'chat':
        return Colors.green;
      case 'code':
        return Colors.purple;
      case 'document':
        return Colors.orange;
      case 'browser':
        return Colors.teal;
      default:
        return Colors.grey;
    }
  }
}