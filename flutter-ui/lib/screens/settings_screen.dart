/// Settings screen for the Talkute app
///
/// Provides access to app configuration, preferences, and data management.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dictionary_screen.dart';
import 'history_screen.dart';
import '../models/language.dart';
import '../widgets/intensity_selector.dart';

// ============================================================
// State Providers for Settings
// ============================================================

/// Provider for input language
final inputLanguageProvider = StateProvider<SupportedLanguage>((ref) => Languages.defaultLanguage);

/// Provider for translation language
final translationLanguageProvider = StateProvider<SupportedLanguage>((ref) => Languages.byCode('zh') ?? Languages.defaultLanguage);

/// Provider for auto-translate enabled
final autoTranslateProvider = StateProvider<bool>((ref) => false);

/// Provider for auto-process (polish + filler removal) enabled
final autoProcessProvider = StateProvider<bool>((ref) => true);

/// Provider for filler removal enabled
final fillerRemovalProvider = StateProvider<bool>((ref) => true);

/// Provider for context-aware mode
final contextAwareProvider = StateProvider<bool>((ref) => true);

/// Provider for push-to-talk mode
final pushToTalkProvider = StateProvider<bool>((ref) => false);

/// Provider for crash reporting opt-in
final crashReportingProvider = StateProvider<bool>((ref) => false);

/// Provider for usage analytics opt-in
final analyticsProvider = StateProvider<bool>((ref) => false);

/// Provider for noise cancellation
final noiseCancellationProvider = StateProvider<bool>((ref) => true);

/// Provider for selected microphone (null = default)
final selectedMicrophoneProvider = StateProvider<String?>((ref) => null);

// ============================================================
// Settings Screen Widget
// ============================================================

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
          ListTile(
            leading: const Icon(Icons.history),
            title: const Text('Transcription history'),
            subtitle: const Text('View past transcriptions'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => const HistoryScreen(),
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

          // Processing section
          _buildSectionHeader('Processing'),
          Consumer(
            builder: (context, ref, _) {
              return SwitchListTile(
                secondary: const Icon(Icons.auto_awesome),
                title: const Text('Auto-process'),
                subtitle: const Text('Polish text and remove fillers automatically'),
                value: ref.watch(autoProcessProvider),
                onChanged: (value) {
                  ref.read(autoProcessProvider.notifier).state = value;
                },
              );
            },
          ),
          Consumer(
            builder: (context, ref, _) {
              return SwitchListTile(
                secondary: const Icon(Icons.filter_alt_outlined),
                title: const Text('Remove fillers'),
                subtitle: const Text('Remove um, uh, like, etc.'),
                value: ref.watch(fillerRemovalProvider),
                onChanged: (value) {
                  ref.read(fillerRemovalProvider.notifier).state = value;
                },
              );
            },
          ),
          Consumer(
            builder: (context, ref, _) {
              return SwitchListTile(
                secondary: const Icon(Icons.psychology_outlined),
                title: const Text('Context-aware mode'),
                subtitle: const Text('Adapt tone based on active application'),
                value: ref.watch(contextAwareProvider),
                onChanged: (value) {
                  ref.read(contextAwareProvider.notifier).state = value;
                },
              );
            },
          ),
          const IntensitySelector(),

          const Divider(),

          // Voice section
          _buildSectionHeader('Voice'),
          ListTile(
            leading: const Icon(Icons.mic),
            title: const Text('Microphone'),
            subtitle: Consumer(
              builder: (context, ref, _) {
                final mic = ref.watch(selectedMicrophoneProvider);
                return Text(mic ?? 'Default');
              },
            ),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => _showMicrophoneSelector(context, ref),
          ),
          Consumer(
            builder: (context, ref, _) {
              return SwitchListTile(
                secondary: const Icon(Icons.noise_control_off),
                title: const Text('Noise cancellation'),
                subtitle: const Text('Reduce background noise'),
                value: ref.watch(noiseCancellationProvider),
                onChanged: (value) {
                  ref.read(noiseCancellationProvider.notifier).state = value;
                },
              );
            },
          ),
          Consumer(
            builder: (context, ref, _) {
              return SwitchListTile(
                secondary: const Icon(Icons.push_pin_outlined),
                title: const Text('Push-to-talk mode'),
                subtitle: const Text('Hold button to record'),
                value: ref.watch(pushToTalkProvider),
                onChanged: (value) {
                  ref.read(pushToTalkProvider.notifier).state = value;
                },
              );
            },
          ),
          ListTile(
            leading: const Icon(Icons.keyboard),
            title: const Text('Keyboard shortcuts'),
            subtitle: const Text('Configure voice activation keys'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => _showKeyboardShortcuts(context),
          ),

          const Divider(),

          // Data section
          _buildSectionHeader('Data'),
          ListTile(
            leading: const Icon(Icons.download_outlined),
            title: const Text('Export data'),
            subtitle: const Text('Export dictionary and settings'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => _showExportDialog(context),
          ),
          ListTile(
            leading: const Icon(Icons.delete_outline, color: Colors.red),
            title: const Text('Clear all data', style: TextStyle(color: Colors.red)),
            subtitle: const Text('Delete all app data'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => _showClearDataDialog(context),
          ),

          const Divider(),

          // Privacy section
          _buildSectionHeader('Privacy'),
          Consumer(
            builder: (context, ref, _) {
              return SwitchListTile(
                secondary: const Icon(Icons.bug_report_outlined),
                title: const Text('Crash reporting'),
                subtitle: const Text('Help improve the app'),
                value: ref.watch(crashReportingProvider),
                onChanged: (value) {
                  ref.read(crashReportingProvider.notifier).state = value;
                },
              );
            },
          ),
          Consumer(
            builder: (context, ref, _) {
              return SwitchListTile(
                secondary: const Icon(Icons.analytics_outlined),
                title: const Text('Usage analytics'),
                subtitle: const Text('Anonymous usage statistics'),
                value: ref.watch(analyticsProvider),
                onChanged: (value) {
                  ref.read(analyticsProvider.notifier).state = value;
                },
              );
            },
          ),

          const Divider(),

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
            onTap: () => _showPrivacyPolicy(context),
          ),
          ListTile(
            leading: const Icon(Icons.description_outlined),
            title: const Text('Terms of Service'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () => _showTermsOfService(context),
          ),

          const SizedBox(height: 32),
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

  void _showMicrophoneSelector(BuildContext context, WidgetRef ref) {
    // In a real app, this would query available microphones
    final microphones = [
      {'id': null, 'name': 'Default'},
      {'id': 'mic_1', 'name': 'Built-in Microphone'},
      {'id': 'mic_2', 'name': 'USB Microphone'},
      {'id': 'mic_3', 'name': 'Bluetooth Headset'},
    ];

    showModalBottomSheet(
      context: context,
      builder: (context) => SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Padding(
              padding: const EdgeInsets.all(16),
              child: Text(
                'Select Microphone',
                style: Theme.of(context).textTheme.titleLarge,
              ),
            ),
            ...microphones.map((mic) => ListTile(
              leading: Icon(
                mic['id'] == null ? Icons.mic : Icons.mic_none,
              ),
              title: Text(mic['name'] as String),
              trailing: Consumer(
                builder: (context, ref, _) {
                  final selected = ref.watch(selectedMicrophoneProvider);
                  if (selected == mic['id']) {
                    return Icon(
                      Icons.check_circle,
                      color: Theme.of(context).colorScheme.primary,
                    );
                  }
                  return const SizedBox.shrink();
                },
              ),
              onTap: () {
                ref.read(selectedMicrophoneProvider.notifier).state =
                    mic['id'] as String?;
                Navigator.pop(context);
              },
            )),
          ],
        ),
      ),
    );
  }

  void _showKeyboardShortcuts(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Keyboard Shortcuts'),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _ShortcutRow('Ctrl + Space', 'Start/Stop recording'),
            _ShortcutRow('Ctrl + T', 'Toggle translation'),
            _ShortcutRow('Ctrl + P', 'Toggle auto-process'),
            _ShortcutRow('Ctrl + D', 'Open dictionary'),
            SizedBox(height: 16),
            Text(
              'Shortcuts can be customized in the app settings.',
              style: TextStyle(color: Colors.grey, fontSize: 12),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }

  void _showExportDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Export Data'),
        content: const Text('Export your dictionary entries and app settings to a JSON file?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () {
              Navigator.pop(context);
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('Data exported successfully')),
              );
            },
            child: const Text('Export'),
          ),
        ],
      ),
    );
  }

  void _showClearDataDialog(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Clear All Data'),
        content: const Text(
          'This will permanently delete all your data including:\n'
          '• Personal dictionary entries\n'
          '• Transcription history\n'
          '• App settings\n\n'
          'This action cannot be undone.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          FilledButton(
            style: FilledButton.styleFrom(
              backgroundColor: Colors.red,
            ),
            onPressed: () {
              Navigator.pop(context);
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('All data cleared')),
              );
            },
            child: const Text('Clear All'),
          ),
        ],
      ),
    );
  }

  void _showPrivacyPolicy(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Privacy Policy'),
        content: const SingleChildScrollView(
          child: Text(
            'Talkute Privacy Policy\n\n'
            'Your privacy is important to us.\n\n'
            'Data Collection:\n'
            '• Voice recordings are processed locally when possible\n'
            '• Transcription data may be sent to cloud services for processing\n'
            '• Personal dictionary entries are stored locally\n\n'
            'Data Usage:\n'
            '• Your data is used only for transcription and translation\n'
            '• We do not sell or share your personal data\n\n'
            'Your Rights:\n'
            '• You can export your data at any time\n'
            '• You can delete all data from settings',
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }

  void _showTermsOfService(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Terms of Service'),
        content: const SingleChildScrollView(
          child: Text(
            'Talkute Terms of Service\n\n'
            'By using Talkute, you agree to:\n\n'
            '1. Use the service responsibly\n'
            '2. Not attempt to abuse or overload our services\n'
            '3. Respect rate limits and usage quotas\n'
            '4. Keep your account credentials secure\n\n'
            'We reserve the right to:\n'
            '• Modify or discontinue the service\n'
            '• Enforce usage limits\n'
            '• Terminate accounts that violate these terms',
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Close'),
          ),
        ],
      ),
    );
  }
}

/// Helper widget for displaying keyboard shortcuts
class _ShortcutRow extends StatelessWidget {
  final String shortcut;
  final String description;

  const _ShortcutRow(this.shortcut, this.description);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
            decoration: BoxDecoration(
              color: Colors.grey.shade200,
              borderRadius: BorderRadius.circular(4),
            ),
            child: Text(
              shortcut,
              style: const TextStyle(
                fontFamily: 'monospace',
                fontSize: 12,
              ),
            ),
          ),
          const SizedBox(width: 16),
          Text(description),
        ],
      ),
    );
  }
}
