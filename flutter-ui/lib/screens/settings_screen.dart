/// Settings screen for the Talkute app
///
/// Provides access to app configuration and dictionary management.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'dictionary_screen.dart';

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
          // Placeholder for future settings
          ListTile(
            leading: const Icon(Icons.mic_outlined),
            title: const Text('Voice Input'),
            subtitle: const Text('Microphone and speech settings'),
            trailing: const Icon(Icons.chevron_right),
            enabled: false,
            onTap: null,
          ),
          ListTile(
            leading: const Icon(Icons.translate_outlined),
            title: const Text('Language'),
            subtitle: const Text('Input and output language preferences'),
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
}