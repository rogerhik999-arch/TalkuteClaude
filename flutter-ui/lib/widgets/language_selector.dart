/// Language selector widget for multi-language support
///
/// Provides a UI for selecting input and output languages.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/language.dart';

/// Style of language selector
enum LanguageSelectorStyle {
  dropdown,
  list,
  chips,
}

/// Language selector widget
class LanguageSelector extends ConsumerWidget {
  /// Callback when a language is selected
  final void Function(SupportedLanguage)? onLanguageSelected;

  /// Currently selected language code
  final String? selectedLanguageCode;

  /// Style of the selector
  final LanguageSelectorStyle style;

  /// Optional filter to show only specific languages
  final List<String>? filter;

  const LanguageSelector({
    super.key,
    this.onLanguageSelected,
    this.selectedLanguageCode,
    this.style = LanguageSelectorStyle.list,
    this.filter,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final languages = filter != null
        ? Languages.all.where((l) => filter!.contains(l.code)).toList()
        : Languages.all;

    switch (style) {
      case LanguageSelectorStyle.dropdown:
        return _buildDropdown(context, languages);
      case LanguageSelectorStyle.chips:
        return _buildChips(context, languages);
      case LanguageSelectorStyle.list:
        return _buildList(context, languages);
    }
  }

  Widget _buildDropdown(BuildContext context, List<SupportedLanguage> languages) {
    final selected = selectedLanguageCode != null
        ? Languages.byCode(selectedLanguageCode!)
        : Languages.defaultLanguage;

    return DropdownButton<SupportedLanguage>(
      value: selected,
      isExpanded: true,
      items: languages.map((language) {
        return DropdownMenuItem(
          value: language,
          child: Row(
            children: [
              Text(language.flagEmoji),
              const SizedBox(width: 8),
              Text(language.name),
            ],
          ),
        );
      }).toList(),
      onChanged: (language) {
        if (language != null && onLanguageSelected != null) {
          onLanguageSelected!(language);
        }
      },
    );
  }

  Widget _buildList(BuildContext context, List<SupportedLanguage> languages) {
    return ListView.builder(
      shrinkWrap: true,
      itemCount: languages.length,
      itemBuilder: (context, index) {
        final language = languages[index];
        final isSelected = language.code == selectedLanguageCode;

        return ListTile(
          leading: Text(
            language.flagEmoji,
            style: const TextStyle(fontSize: 24),
          ),
          title: Text(language.name),
          subtitle: Text(language.nativeName),
          trailing: isSelected
              ? Icon(
                  Icons.check_circle,
                  color: Theme.of(context).colorScheme.primary,
                )
              : null,
          selected: isSelected,
          onTap: () {
            if (onLanguageSelected != null) {
              onLanguageSelected!(language);
            }
          },
        );
      },
    );
  }

  Widget _buildChips(BuildContext context, List<SupportedLanguage> languages) {
    return Wrap(
      spacing: 8,
      runSpacing: 8,
      children: languages.map((language) {
        final isSelected = language.code == selectedLanguageCode;

        return FilterChip(
          label: Text('${language.flagEmoji} ${language.name}'),
          selected: isSelected,
          onSelected: (_) {
            if (onLanguageSelected != null) {
              onLanguageSelected!(language);
            }
          },
        );
      }).toList(),
    );
  }
}

/// Compact language selector for inline use
class CompactLanguageSelector extends ConsumerWidget {
  /// Currently selected language
  final SupportedLanguage selectedLanguage;

  /// Callback when a language is selected
  final void Function(SupportedLanguage)? onLanguageSelected;

  /// Optional filter to show only specific languages
  final List<String>? filter;

  const CompactLanguageSelector({
    super.key,
    required this.selectedLanguage,
    this.onLanguageSelected,
    this.filter,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final languages = filter != null
        ? Languages.all.where((l) => filter!.contains(l.code)).toList()
        : Languages.all;

    return InkWell(
      onTap: () => _showLanguageSheet(context, languages),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            selectedLanguage.flagEmoji,
            style: const TextStyle(fontSize: 20),
          ),
          const SizedBox(width: 4),
          Text(
            selectedLanguage.name,
            style: Theme.of(context).textTheme.bodyMedium,
          ),
          const Icon(Icons.arrow_drop_down),
        ],
      ),
    );
  }

  void _showLanguageSheet(BuildContext context, List<SupportedLanguage> languages) {
    showModalBottomSheet(
      context: context,
      builder: (context) => SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Padding(
              padding: const EdgeInsets.all(16),
              child: Text(
                'Select Language',
                style: Theme.of(context).textTheme.titleLarge,
              ),
            ),
            Flexible(
              child: ListView.builder(
                shrinkWrap: true,
                itemCount: languages.length,
                itemBuilder: (context, index) {
                  final language = languages[index];
                  final isSelected = language.code == selectedLanguage.code;

                  return ListTile(
                    leading: Text(
                      language.flagEmoji,
                      style: const TextStyle(fontSize: 24),
                    ),
                    title: Text(language.name),
                    subtitle: Text(language.nativeName),
                    trailing: isSelected
                        ? Icon(
                            Icons.check_circle,
                            color: Theme.of(context).colorScheme.primary,
                          )
                        : null,
                    onTap: () {
                      Navigator.pop(context);
                      if (onLanguageSelected != null) {
                        onLanguageSelected!(language);
                      }
                    },
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}