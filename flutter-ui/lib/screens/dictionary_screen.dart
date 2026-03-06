/// Dictionary screen for managing personal dictionary entries
///
/// Allows users to add, edit, and remove custom terms.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/dictionary.dart';

/// Provider for dictionary entries
final dictionaryEntriesProvider = StateProvider<List<PersonalDictionaryEntry>>((ref) {
  // TODO: Load from Rust FFI
  return [];
});

/// Dictionary screen widget
class DictionaryScreen extends ConsumerWidget {
  const DictionaryScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final entries = ref.watch(dictionaryEntriesProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Personal Dictionary'),
        actions: [
          IconButton(
            icon: const Icon(Icons.import_export),
            onPressed: () => _showImportExportMenu(context),
            tooltip: 'Import/Export',
          ),
        ],
      ),
      body: entries.isEmpty
          ? _buildEmptyState(context)
          : _buildEntriesList(context, ref, entries),
      floatingActionButton: FloatingActionButton(
        onPressed: () => _showAddEntryDialog(context, ref),
        child: const Icon(Icons.add),
        tooltip: 'Add entry',
      ),
    );
  }

  Widget _buildEmptyState(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.book_outlined,
            size: 64,
            color: Theme.of(context).colorScheme.primary.withValues(alpha: 0.5),
          ),
          const SizedBox(height: 16),
          Text(
            'No dictionary entries yet',
            style: Theme.of(context).textTheme.titleLarge,
          ),
          const SizedBox(height: 8),
          Text(
            'Tap + to add your first custom term',
            style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  color: Colors.grey,
                ),
          ),
        ],
      ),
    );
  }

  Widget _buildEntriesList(
    BuildContext context,
    WidgetRef ref,
    List<PersonalDictionaryEntry> entries,
  ) {
    return ListView.builder(
      padding: const EdgeInsets.all(8),
      itemCount: entries.length,
      itemBuilder: (context, index) {
        final entry = entries[index];
        return _DictionaryEntryCard(
          entry: entry,
          onEdit: () => _showEditEntryDialog(context, ref, entry),
          onDelete: () => _confirmDelete(context, ref, entry),
        );
      },
    );
  }

  void _showAddEntryDialog(BuildContext context, WidgetRef ref) {
    showDialog(
      context: context,
      builder: (context) => _AddEntryDialog(
        onAdd: (entry) {
          final entries = ref.read(dictionaryEntriesProvider);
          ref.read(dictionaryEntriesProvider.notifier).state = [...entries, entry];
        },
      ),
    );
  }

  void _showEditEntryDialog(
    BuildContext context,
    WidgetRef ref,
    PersonalDictionaryEntry entry,
  ) {
    showDialog(
      context: context,
      builder: (context) => _EditEntryDialog(
        entry: entry,
        onUpdate: (updatedEntry) {
          final entries = ref.read(dictionaryEntriesProvider);
          ref.read(dictionaryEntriesProvider.notifier).state = entries
              .map((e) => e.entryId == updatedEntry.entryId ? updatedEntry : e)
              .toList();
        },
      ),
    );
  }

  void _confirmDelete(
    BuildContext context,
    WidgetRef ref,
    PersonalDictionaryEntry entry,
  ) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Delete Entry'),
        content: Text('Delete "${entry.phrase}" from your dictionary?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              final entries = ref.read(dictionaryEntriesProvider);
              ref.read(dictionaryEntriesProvider.notifier).state =
                  entries.where((e) => e.entryId != entry.entryId).toList();
              Navigator.pop(context);
            },
            child: Text(
              'Delete',
              style: TextStyle(color: Theme.of(context).colorScheme.error),
            ),
          ),
        ],
      ),
    );
  }

  void _showImportExportMenu(BuildContext context) {
    showModalBottomSheet(
      context: context,
      builder: (context) => SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              leading: const Icon(Icons.file_upload),
              title: const Text('Import Dictionary'),
              onTap: () {
                Navigator.pop(context);
                // TODO: Implement import
              },
            ),
            ListTile(
              leading: const Icon(Icons.file_download),
              title: const Text('Export Dictionary'),
              onTap: () {
                Navigator.pop(context);
                // TODO: Implement export
              },
            ),
          ],
        ),
      ),
    );
  }
}

/// Dictionary entry card widget
class _DictionaryEntryCard extends StatelessWidget {
  final PersonalDictionaryEntry entry;
  final VoidCallback onEdit;
  final VoidCallback onDelete;

  const _DictionaryEntryCard({
    required this.entry,
    required this.onEdit,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      child: ListTile(
        title: Text(
          entry.phrase,
          style: const TextStyle(fontWeight: FontWeight.bold),
        ),
        subtitle: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(entry.replacement),
            const SizedBox(height: 4),
            _CategoryBadge(category: entry.category),
          ],
        ),
        isThreeLine: true,
        trailing: PopupMenuButton<String>(
          onSelected: (value) {
            switch (value) {
              case 'edit':
                onEdit();
                break;
              case 'delete':
                onDelete();
                break;
            }
          },
          itemBuilder: (context) => [
            const PopupMenuItem(
              value: 'edit',
              child: ListTile(
                leading: Icon(Icons.edit),
                title: Text('Edit'),
              ),
            ),
            const PopupMenuItem(
              value: 'delete',
              child: ListTile(
                leading: Icon(Icons.delete),
                title: Text('Delete'),
              ),
            ),
          ],
        ),
        onTap: onEdit,
      ),
    );
  }
}

/// Category badge widget
class _CategoryBadge extends StatelessWidget {
  final DictionaryCategory category;

  const _CategoryBadge({required this.category});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
      decoration: BoxDecoration(
        color: _getCategoryColor().withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: _getCategoryColor().withValues(alpha: 0.3),
        ),
      ),
      child: Text(
        category.displayName,
        style: TextStyle(
          color: _getCategoryColor(),
          fontSize: 12,
          fontWeight: FontWeight.w500,
        ),
      ),
    );
  }

  Color _getCategoryColor() {
    switch (category) {
      case DictionaryCategory.technical:
        return Colors.blue;
      case DictionaryCategory.business:
        return Colors.green;
      case DictionaryCategory.medical:
        return Colors.red;
      case DictionaryCategory.general:
        return Colors.grey;
    }
  }
}

/// Add entry dialog
class _AddEntryDialog extends StatefulWidget {
  final void Function(PersonalDictionaryEntry) onAdd;

  const _AddEntryDialog({required this.onAdd});

  @override
  State<_AddEntryDialog> createState() => _AddEntryDialogState();
}

class _AddEntryDialogState extends State<_AddEntryDialog> {
  final _phraseController = TextEditingController();
  final _replacementController = TextEditingController();
  DictionaryCategory _category = DictionaryCategory.general;
  bool _caseSensitive = false;
  bool _wholeWordOnly = true;

  @override
  void dispose() {
    _phraseController.dispose();
    _replacementController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Add Dictionary Entry'),
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: _phraseController,
              decoration: const InputDecoration(
                labelText: 'Phrase',
                hintText: 'e.g., API',
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _replacementController,
              decoration: const InputDecoration(
                labelText: 'Replacement',
                hintText: 'e.g., Application Programming Interface',
              ),
            ),
            const SizedBox(height: 16),
            DropdownButtonFormField<DictionaryCategory>(
              value: _category,
              decoration: const InputDecoration(
                labelText: 'Category',
              ),
              items: DictionaryCategory.values.map((cat) {
                return DropdownMenuItem(
                  value: cat,
                  child: Text(cat.displayName),
                );
              }).toList(),
              onChanged: (value) {
                if (value != null) {
                  setState(() => _category = value);
                }
              },
            ),
            const SizedBox(height: 16),
            SwitchListTile(
              title: const Text('Case sensitive'),
              value: _caseSensitive,
              onChanged: (value) => setState(() => _caseSensitive = value),
            ),
            SwitchListTile(
              title: const Text('Whole word only'),
              value: _wholeWordOnly,
              onChanged: (value) => setState(() => _wholeWordOnly = value),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Cancel'),
        ),
        FilledButton(
          onPressed: () {
            if (_phraseController.text.isNotEmpty &&
                _replacementController.text.isNotEmpty) {
              widget.onAdd(PersonalDictionaryEntry(
                entryId: DateTime.now().millisecondsSinceEpoch.toString(),
                deviceId: 'local',
                phrase: _phraseController.text,
                replacement: _replacementController.text,
                category: _category,
                caseSensitive: _caseSensitive,
                wholeWordOnly: _wholeWordOnly,
                createdAt: DateTime.now(),
              ));
              Navigator.pop(context);
            }
          },
          child: const Text('Add'),
        ),
      ],
    );
  }
}

/// Edit entry dialog
class _EditEntryDialog extends StatefulWidget {
  final PersonalDictionaryEntry entry;
  final void Function(PersonalDictionaryEntry) onUpdate;

  const _EditEntryDialog({
    required this.entry,
    required this.onUpdate,
  });

  @override
  State<_EditEntryDialog> createState() => _EditEntryDialogState();
}

class _EditEntryDialogState extends State<_EditEntryDialog> {
  late final TextEditingController _phraseController;
  late final TextEditingController _replacementController;
  late DictionaryCategory _category;
  late bool _caseSensitive;
  late bool _wholeWordOnly;

  @override
  void initState() {
    super.initState();
    _phraseController = TextEditingController(text: widget.entry.phrase);
    _replacementController = TextEditingController(text: widget.entry.replacement);
    _category = widget.entry.category;
    _caseSensitive = widget.entry.caseSensitive;
    _wholeWordOnly = widget.entry.wholeWordOnly;
  }

  @override
  void dispose() {
    _phraseController.dispose();
    _replacementController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Edit Dictionary Entry'),
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: _phraseController,
              decoration: const InputDecoration(
                labelText: 'Phrase',
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: _replacementController,
              decoration: const InputDecoration(
                labelText: 'Replacement',
              ),
            ),
            const SizedBox(height: 16),
            DropdownButtonFormField<DictionaryCategory>(
              value: _category,
              decoration: const InputDecoration(
                labelText: 'Category',
              ),
              items: DictionaryCategory.values.map((cat) {
                return DropdownMenuItem(
                  value: cat,
                  child: Text(cat.displayName),
                );
              }).toList(),
              onChanged: (value) {
                if (value != null) {
                  setState(() => _category = value);
                }
              },
            ),
            const SizedBox(height: 16),
            SwitchListTile(
              title: const Text('Case sensitive'),
              value: _caseSensitive,
              onChanged: (value) => setState(() => _caseSensitive = value),
            ),
            SwitchListTile(
              title: const Text('Whole word only'),
              value: _wholeWordOnly,
              onChanged: (value) => setState(() => _wholeWordOnly = value),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Cancel'),
        ),
        FilledButton(
          onPressed: () {
            if (_phraseController.text.isNotEmpty &&
                _replacementController.text.isNotEmpty) {
              widget.onUpdate(widget.entry.copyWith(
                phrase: _phraseController.text,
                replacement: _replacementController.text,
                category: _category,
                caseSensitive: _caseSensitive,
                wholeWordOnly: _wholeWordOnly,
              ));
              Navigator.pop(context);
            }
          },
          child: const Text('Save'),
        ),
      ],
    );
  }
}