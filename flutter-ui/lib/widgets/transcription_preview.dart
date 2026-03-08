/// Transcription preview widget for Talkute
///
/// Displays the raw and polished text with copy functionality.

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../state/voice_state.dart';

/// Transcription preview widget
class TranscriptionPreview extends ConsumerWidget {
  final bool showRaw;
  final bool showPolished;
  final bool showCopyButton;

  const TranscriptionPreview({
    super.key,
    this.showRaw = true,
    this.showPolished = true,
    this.showCopyButton = true,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(voiceStateProvider);

    if (!state.hasTranscription && !state.hasPolishedText) {
      return const SizedBox.shrink();
    }

    return Card(
      margin: const EdgeInsets.all(16),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            if (showPolished && state.hasPolishedText) ...[
              _buildSection(
                context,
                title: 'Polished Text',
                text: state.polishedText,
                icon: Icons.auto_fix_high,
                color: Colors.green,
              ),
              if (showRaw && state.hasTranscription) const Divider(height: 24),
            ],
            if (showRaw && state.hasTranscription) ...[
              _buildSection(
                context,
                title: 'Raw Transcription',
                text: state.rawTranscription,
                icon: Icons.text_fields,
                color: Colors.blue,
              ),
            ],
            if (state.wordCount > 0) ...[
              const SizedBox(height: 8),
              _buildWordCount(context, state.wordCount),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildSection(
    BuildContext context, {
    required String title,
    required String text,
    required IconData icon,
    required Color color,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Icon(icon, size: 18, color: color),
            const SizedBox(width: 8),
            Text(
              title,
              style: Theme.of(context).textTheme.titleSmall?.copyWith(
                    color: color,
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const Spacer(),
            if (showCopyButton)
              IconButton(
                icon: const Icon(Icons.copy, size: 18),
                onPressed: () => _copyToClipboard(context, text),
                tooltip: 'Copy to clipboard',
              ),
          ],
        ),
        const SizedBox(height: 8),
        SelectableText(
          text,
          style: Theme.of(context).textTheme.bodyMedium,
        ),
      ],
    );
  }

  Widget _buildWordCount(BuildContext context, int count) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        Icon(
          Icons.format_size,
          size: 14,
          color: Theme.of(context).textTheme.bodySmall?.color,
        ),
        const SizedBox(width: 4),
        Text(
          '$count words',
          style: Theme.of(context).textTheme.bodySmall,
        ),
      ],
    );
  }

  void _copyToClipboard(BuildContext context, String text) {
    Clipboard.setData(ClipboardData(text: text));
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Copied to clipboard'),
        duration: Duration(seconds: 2),
      ),
    );
  }
}

/// Compact transcription preview for inline use
class CompactTranscriptionPreview extends ConsumerWidget {
  const CompactTranscriptionPreview({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(voiceStateProvider);

    if (!state.hasPolishedText) {
      return const SizedBox.shrink();
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        children: [
          Expanded(
            child: Text(
              state.polishedText,
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          ),
          IconButton(
            icon: const Icon(Icons.copy, size: 18),
            onPressed: () => _copyToClipboard(context, state.polishedText),
            tooltip: 'Copy to clipboard',
          ),
        ],
      ),
    );
  }

  void _copyToClipboard(BuildContext context, String text) {
    Clipboard.setData(ClipboardData(text: text));
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(
        content: Text('Copied to clipboard'),
        duration: Duration(seconds: 2),
      ),
    );
  }
}
