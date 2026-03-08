/// Home screen for Talkute
///
/// Main screen with voice input functionality.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../state/voice_state.dart';
import '../widgets/voice_button.dart';
import '../widgets/transcription_preview.dart';
import '../widgets/context_indicator.dart';
import '../models/context.dart';
import 'settings_screen.dart';
import 'history_screen.dart';

/// Home screen widget
class HomeScreen extends ConsumerStatefulWidget {
  const HomeScreen({super.key});

  @override
  ConsumerState<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends ConsumerState<HomeScreen> {
  @override
  Widget build(BuildContext context) {
    final state = ref.watch(voiceStateProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Talkute'),
        actions: [
          IconButton(
            icon: const Icon(Icons.settings),
            onPressed: () => _showSettings(context),
          ),
          IconButton(
            icon: const Icon(Icons.history),
            onPressed: () => _showHistory(context),
          ),
        ],
      ),
      body: SafeArea(
        child: Column(
          children: [
            // Context indicator
            _buildContextIndicator(context),

            // Main content area
            Expanded(
              child: _buildMainContent(context, state),
            ),

            // Voice button and controls
            _buildBottomControls(context, state),
          ],
        ),
      ),
    );
  }

  Widget _buildContextIndicator(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: const ContextIndicator(),
    );
  }

  Widget _buildMainContent(BuildContext context, VoiceState state) {
    if (state.hasError) {
      return _buildErrorView(context, state);
    }

    if (state.hasPolishedText || state.hasTranscription) {
      return SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: TranscriptionPreview(
          showRaw: true,
          showPolished: true,
          showCopyButton: true,
        ),
      );
    }

    return _buildIdleView(context);
  }

  Widget _buildIdleView(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(
            Icons.mic_none,
            size: 64,
            color: Theme.of(context).colorScheme.primary.withOpacity(0.5),
          ),
          const SizedBox(height: 16),
          Text(
            'Tap the button below to start',
            style: Theme.of(context).textTheme.bodyLarge,
          ),
          const SizedBox(height: 8),
          Text(
            'Speak naturally and we\'ll polish your text',
            style: Theme.of(context).textTheme.bodyMedium,
          ),
        ],
      ),
    );
  }

  Widget _buildErrorView(BuildContext context, VoiceState state) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.error_outline,
              size: 64,
              color: Theme.of(context).colorScheme.error,
            ),
            const SizedBox(height: 16),
            Text(
              'Something went wrong',
              style: Theme.of(context).textTheme.titleLarge,
            ),
            const SizedBox(height: 8),
            Text(
              state.errorMessage ?? 'An error occurred',
              style: Theme.of(context).textTheme.bodyMedium,
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildBottomControls(BuildContext context, VoiceState state) {
    return Container(
      padding: const EdgeInsets.all(24),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          // Quota indicator
          _buildQuotaIndicator(context),

          const SizedBox(height: 24),

          // Voice button
          VoiceButton(
            size: 80,
            onStart: _startRecording,
            onStop: _stopRecording,
            onCancel: _cancelSession,
          ),

          const SizedBox(height: 16),

          // Status text
          Text(
            _getStatusText(state),
            style: Theme.of(context).textTheme.bodySmall,
          ),
        ],
      ),
    );
  }

  Widget _buildQuotaIndicator(BuildContext context) {
    // TODO: Get actual quota from state
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Icon(
          Icons.data_usage,
          size: 14,
          color: Theme.of(context).textTheme.bodySmall?.color,
        ),
        const SizedBox(width: 4),
        Text(
          '0 / 4000 words this week',
          style: Theme.of(context).textTheme.bodySmall,
        ),
      ],
    );
  }

  String _getStatusText(VoiceState state) {
    switch (state.status) {
      case VoiceRecordingStatus.idle:
        return 'Ready to record';
      case VoiceRecordingStatus.recording:
        return 'Recording... Long press to cancel';
      case VoiceRecordingStatus.transcribing:
        return 'Converting speech to text...';
      case VoiceRecordingStatus.polishing:
        return 'Polishing your text...';
      case VoiceRecordingStatus.completed:
        return 'Done! Tap to start new recording';
      case VoiceRecordingStatus.error:
        return 'Tap to try again';
    }
  }

  Future<void> _startRecording() async {
    final notifier = ref.read(voiceStateProvider.notifier);
    await notifier.startRecording();

    // Detect application context automatically
    await _detectContext();

    // TODO: Call Rust FFI to start actual recording
    // For now, simulate recording with a timer
  }

  /// Detect the current application context
  Future<void> _detectContext() async {
    final notifier = ref.read(voiceStateProvider.notifier);

    try {
      // TODO: Call Rust FFI detect_application_context()
      // For now, simulate context detection with a placeholder
      // Using a microtask delay to avoid blocking the UI
      await Future.microtask(() {});

      // Create a simulated context (in real implementation, this comes from FFI)
      final context = ApplicationContext(
        contextId: DateTime.now().millisecondsSinceEpoch.toString(),
        applicationName: 'Unknown',
        applicationCategory: 'other',
        detectedAt: DateTime.now(),
        lastUsedAt: DateTime.now(),
      );

      notifier.setDetectedContext(context);

      // Check quota and show warning if needed
      _checkQuotaAndShowWarning();
    } catch (e) {
      // Context detection failure should not block recording
      debugPrint('Context detection failed: $e');
    }
  }

  /// Check quota and show warning if needed
  void _checkQuotaAndShowWarning() {
    final quotaUsed = 3500; // Simulated quota usage
    final quotaLimit = 4000;
    final warningThreshold = 0.8; // 80%

    if (quotaUsed >= quotaLimit * warningThreshold) {
      // Show warning notification
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Row(
            children: [
              const Icon(Icons.warning, color: Colors.orange),
              const SizedBox(width: 8),
              Text('Warning: ${quotaLimit - quotaUsed} words remaining this week'),
            ],
          ),
          action: SnackBarAction(
            label: 'Upgrade',
            onPressed: () {
              // TODO: Navigate to upgrade screen
            },
          ),
          duration: const Duration(seconds: 5),
        ),
      );
    }
  }

  Future<void> _stopRecording() async {
    final notifier = ref.read(voiceStateProvider.notifier);
    await notifier.stopRecording();

    // TODO: Call Rust FFI to stop recording and transcribe
    // For now, simulate transcription
    await Future.delayed(const Duration(seconds: 1));
    notifier.setRawTranscription('This is a sample transcription from the voice input.');

    await Future.delayed(const Duration(milliseconds: 500));
    notifier.startPolishing();

    await Future.delayed(const Duration(seconds: 1));
    notifier.setPolishedText('This is a sample transcription from the voice input.');
  }

  void _cancelSession() {
    final notifier = ref.read(voiceStateProvider.notifier);
    notifier.cancelSession();
  }

  void _showSettings(BuildContext context) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => const SettingsScreen(),
      ),
    );
  }

  void _showHistory(BuildContext context) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => const HistoryScreen(),
      ),
    );
  }
}
