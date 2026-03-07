/// Clipboard Fallback Dialog
///
/// Shows when text injection fails, allowing user to copy text to clipboard.

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

/// Show clipboard fallback dialog when text injection fails
Future<bool> showClipboardFallbackDialog(
  BuildContext context, {
  required String text,
  String? errorMessage,
}) async {
  final result = await showDialog<bool>(
    context: context,
    barrierDismissible: false,
    builder: (context) => AlertDialog(
      title: const Row(
        children: [
          Icon(Icons.content_copy, color: Colors.orange),
          SizedBox(width: 8),
          Text('Copy to Clipboard'),
        ],
      ),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Text could not be inserted automatically.',
            style: Theme.of(context).textTheme.bodyMedium,
          ),
          if (errorMessage != null) ...[
            const SizedBox(height: 8),
            Text(
              'Error: $errorMessage',
              style: Theme.of(context).textTheme.bodySmall?.copyWith(
                    color: Colors.red,
                  ),
            ),
          ],
          const SizedBox(height: 16),
          Container(
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: Theme.of(context).colorScheme.surfaceContainerHighest,
              borderRadius: BorderRadius.circular(8),
            ),
            child: SelectableText(
              text,
              style: const TextStyle(fontFamily: 'monospace'),
            ),
          ),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context, false),
          child: const Text('Cancel'),
        ),
        FilledButton.icon(
          onPressed: () async {
            await Clipboard.setData(ClipboardData(text: text));
            if (context.mounted) {
              Navigator.pop(context, true);
            }
          },
          icon: const Icon(Icons.copy),
          label: const Text('Copy'),
        ),
      ],
    ),
  );

  return result ?? false;
}

/// Show quota warning notification
void showQuotaWarningSnackBar(BuildContext context, {required int used, required int total}) {
  final remaining = total - used;
  final percentage = (used / total * 100).toStringAsFixed(0);

  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      content: Row(
        children: [
          const Icon(Icons.warning_amber, color: Colors.orange),
          const SizedBox(width: 12),
          Expanded(
            child: Text(
              'Usage at $percentage%. $remaining words remaining this week.',
            ),
          ),
        ],
      ),
      action: SnackBarAction(
        label: 'Upgrade',
        onPressed: () {
          // TODO: Navigate to upgrade screen
        },
      ),
      duration: const Duration(seconds: 5),
      behavior: SnackBarBehavior.floating,
    ),
  );
}

/// Show quota exceeded dialog
Future<void> showQuotaExceededDialog(BuildContext context) async {
  await showDialog(
    context: context,
    barrierDismissible: false,
    builder: (context) => AlertDialog(
      title: const Row(
        children: [
          Icon(Icons.block, color: Colors.red),
          SizedBox(width: 8),
          Text('Weekly Quota Exceeded'),
        ],
      ),
      content: const Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'You have reached your weekly word limit.',
          ),
          SizedBox(height: 16),
          Text(
            'Upgrade to Premium for:',
            style: TextStyle(fontWeight: FontWeight.bold),
          ),
          SizedBox(height: 8),
          _FeatureRow('Unlimited words'),
          _FeatureRow('Priority processing'),
          _FeatureRow('Advanced polishing modes'),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Maybe Later'),
        ),
        FilledButton.icon(
          onPressed: () {
            Navigator.pop(context);
            // TODO: Navigate to upgrade screen
          },
          icon: const Icon(Icons.star),
          label: const Text('Upgrade to Premium'),
        ),
      ],
    ),
  );
}

class _FeatureRow extends StatelessWidget {
  final String text;

  const _FeatureRow(this.text);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        children: [
          const Icon(Icons.check_circle, color: Colors.green, size: 16),
          const SizedBox(width: 8),
          Text(text),
        ],
      ),
    );
  }
}