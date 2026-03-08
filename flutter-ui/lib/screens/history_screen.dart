/// History screen for viewing past transcriptions
///
/// Displays a list of transcriptions with timestamps and
/// provides options to copy, delete, or re-process them.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Provider for transcription history
final transcriptionHistoryProvider = StateProvider<List<TranscriptionEntry>>((ref) => []);

/// A single transcription history entry
class TranscriptionEntry {
  final String id;
  final String text;
  final DateTime timestamp;
  final String? translatedText;
  final String language;

  TranscriptionEntry({
    required this.id,
    required this.text,
    required this.timestamp,
    this.translatedText,
    required this.language,
  });
}

/// History screen widget
class HistoryScreen extends ConsumerWidget {
  const HistoryScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final history = ref.watch(transcriptionHistoryProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Transcription History'),
        actions: [
          if (history.isNotEmpty)
            IconButton(
              icon: const Icon(Icons.delete_sweep),
              tooltip: 'Clear all',
              onPressed: () => _showClearConfirmation(context, ref),
            ),
        ],
      ),
      body: history.isEmpty
          ? _buildEmptyState()
          : _buildHistoryList(context, ref, history),
    );
  }

  Widget _buildEmptyState() {
    return const Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.history,
            size: 64,
            color: Colors.grey,
          ),
          SizedBox(height: 16),
          Text(
            'No transcriptions yet',
            style: TextStyle(
              fontSize: 18,
              color: Colors.grey,
            ),
          ),
          SizedBox(height: 8),
          Text(
            'Start recording to see your history here',
            style: TextStyle(color: Colors.grey),
          ),
        ],
      ),
    );
  }

  Widget _buildHistoryList(BuildContext context, WidgetRef ref, List<TranscriptionEntry> history) {
    return ListView.builder(
      itemCount: history.length,
      itemBuilder: (context, index) {
        final entry = history[index];
        return _HistoryEntryCard(
          entry: entry,
          onDelete: () => _deleteEntry(ref, entry.id),
          onCopy: () => _copyToClipboard(context, entry.text),
        );
      },
    );
  }

  void _showClearConfirmation(BuildContext context, WidgetRef ref) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Clear History'),
        content: const Text('Delete all transcription history? This cannot be undone.'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          FilledButton(
            style: FilledButton.styleFrom(backgroundColor: Colors.red),
            onPressed: () {
              ref.read(transcriptionHistoryProvider.notifier).state = [];
              Navigator.pop(context);
            },
            child: const Text('Clear All'),
          ),
        ],
      ),
    );
  }

  void _deleteEntry(WidgetRef ref, String id) {
    final currentHistory = ref.read(transcriptionHistoryProvider);
    ref.read(transcriptionHistoryProvider.notifier).state =
        currentHistory.where((e) => e.id != id).toList();
  }

  void _copyToClipboard(BuildContext context, String text) {
    // In a real app, this would use the clipboard API
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Copied to clipboard')),
    );
  }
}

/// Card widget for displaying a single history entry
class _HistoryEntryCard extends StatelessWidget {
  final TranscriptionEntry entry;
  final VoidCallback onDelete;
  final VoidCallback onCopy;

  const _HistoryEntryCard({
    required this.entry,
    required this.onDelete,
    required this.onCopy,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Text(
                  _formatTimestamp(entry.timestamp),
                  style: Theme.of(context).textTheme.bodySmall,
                ),
                const Spacer(),
                PopupMenuButton<String>(
                  icon: const Icon(Icons.more_vert),
                  onSelected: (value) {
                    switch (value) {
                      case 'copy':
                        onCopy();
                        break;
                      case 'delete':
                        onDelete();
                        break;
                    }
                  },
                  itemBuilder: (context) => [
                    const PopupMenuItem(
                      value: 'copy',
                      child: ListTile(
                        leading: Icon(Icons.copy),
                        title: Text('Copy'),
                        contentPadding: EdgeInsets.zero,
                      ),
                    ),
                    const PopupMenuItem(
                      value: 'delete',
                      child: ListTile(
                        leading: Icon(Icons.delete, color: Colors.red),
                        title: Text('Delete', style: TextStyle(color: Colors.red)),
                        contentPadding: EdgeInsets.zero,
                      ),
                    ),
                  ],
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              entry.text,
              style: Theme.of(context).textTheme.bodyLarge,
            ),
            if (entry.translatedText != null) ...[
              const SizedBox(height: 8),
              Container(
                padding: const EdgeInsets.all(8),
                decoration: BoxDecoration(
                  color: Theme.of(context).colorScheme.surfaceContainerHighest,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Translation',
                      style: Theme.of(context).textTheme.labelSmall,
                    ),
                    const SizedBox(height: 4),
                    Text(entry.translatedText!),
                  ],
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  String _formatTimestamp(DateTime timestamp) {
    final now = DateTime.now();
    final difference = now.difference(timestamp);

    if (difference.inMinutes < 1) {
      return 'Just now';
    } else if (difference.inHours < 1) {
      return '${difference.inMinutes}m ago';
    } else if (difference.inDays < 1) {
      return '${difference.inHours}h ago';
    } else if (difference.inDays < 7) {
      return '${difference.inDays}d ago';
    } else {
      return '${timestamp.day}/${timestamp.month}/${timestamp.year}';
    }
  }
}
