/// Settings screen for the Talkute app
///
/// Provides access to app configuration and dictionary management.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dictionary_screen.dart';
import '../models/language.dart';

/// Provider for input language
final inputLanguageProvider = StateProvider<SupportedLanguage>((ref) => Languages.defaultLanguage);

/// Provider for translation language
final translationLanguageProvider = StateProvider<SupportedLanguage>((ref) => Languages.byCode('zh') ?? Languages.defaultLanguage);

/// Provider for auto-translate enabled
final autoTranslateProvider = StateProvider<bool>((ref) => false);

/// Settings screen widget
class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Settings'),
      ),
      body: ListView(
        children: [
          // General section
          _buildSectionHeader('General'),
          ListTile(
            leading: const Icon(Icons.book_outlined),
            title: const Text('Personal Dictionary'),
            subtitle: const Text('Manage custom terms and phrases'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => const DictionaryScreen(),
                ),
              );
            },
          ),
          const Divider(),
          // Language section
          _buildSectionHeader('Language'),
          ListTile(
            leading: const Icon(Icons.language),
            title: const Text('Input Language'),
            subtitle: Consumer(
              builder: (context, ref, _) {
                final language = ref.watch(inputLanguageProvider);
                return Text('${language.flagEmoji} ${language.name}');
              },
            ),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => _showLanguageSelector(context, ref, true),
          ),
          ListTile(
            leading: const Icon(Icons.translate),
            title: const Text('Translation Language'),
            subtitle: Consumer(
              builder: (context, ref, _) {
                final language = ref.watch(translationLanguageProvider);
                return Text('${language.flagEmoji} ${language.name}');
              },
            ),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => _showLanguageSelector(context, ref, false),
          ),
          Consumer(
            builder: (context, ref, _) {
              return SwitchListTile(
                secondary: const Icon(Icons.auto_fix_high),
                title: const Text('Auto-translate'),
                subtitle: const Text('Automatically translate after transcription'),
                value: ref.watch(autoTranslateProvider),
                onChanged: (value) {
                  ref.read(autoTranslateProvider.notifier).state = value;
                },
              );
            },
          ),
          const Divider(),
          // Voice section
          ListTile(
            leading: const Icon(Icons.mic_outlined),
            title: const Text('Voice Input'),
            subtitle: const Text('Microphone and speech settings'),
            trailing: const Icon(Icons.chevron_right),
            enabled: false,
            onTap: null,
          ),
          // About section
          _buildSectionHeader('About'),
          ListTile(
            leading: const Icon(Icons.info_outline),
            title: const Text('Version'),
            subtitle: const Text('1.0.0'),
          ),
          ListTile(
            leading: const Icon(Icons.privacy_tip_outlined),
            title: const Text('Privacy Policy'),
            trailing: const Icon(Icons.chevron_right),
            enabled: false,
            onTap: null,
          ),
        ],
      ),
    );
  }

  Widget _buildSectionHeader(String title) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
      child: Text(
        title,
        style: const TextStyle(
          fontSize: 14,
          fontWeight: FontWeight.bold,
          color: Colors.grey,
        ),
      ),
    );
  }

  void _showLanguageSelector(BuildContext context, WidgetRef ref, bool isInputLanguage) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => DraggableScrollableSheet(
        initialChildSize: 0.6,
        maxChildSize: 0.9,
        minChildSize: 0.3,
        expand: false,
        builder: (context, scrollController) => SafeArea(
          child: Column(
            children: [
              Padding(
                padding: const EdgeInsets.all(16),
                child: Text(
                  'Select Language',
                  style: Theme.of(context).textTheme.titleLarge,
                ),
              ),
              Expanded(
                child: ListView.builder(
                  controller: scrollController,
                  itemCount: Languages.all.length,
                  itemBuilder: (context, index) {
                    final language = Languages.all[index];
                    final currentLanguage = isInputLanguage
                        ? ref.read(inputLanguageProvider)
                        : ref.read(translationLanguageProvider);
                    final isSelected = language.code == currentLanguage.code;

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
                        if (isInputLanguage) {
                          ref.read(inputLanguageProvider.notifier).state = language;
                        } else {
                          ref.read(translationLanguageProvider.notifier).state = language;
                        }
                        Navigator.pop(context);
                      },
                    );
                  },
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}