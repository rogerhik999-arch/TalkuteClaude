/// Home screen for Talkute
///
/// Main screen with voice input functionality.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../state/voice_state.dart';
import '../widgets/voice_button.dart';
import '../widgets/transcription_preview.dart';

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
    // TODO: Show detected application context
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Row(
        children: [
          const Icon(Icons.apps, size: 16),
          const SizedBox(width: 8),
          Text(
            'Ready',
            style: Theme.of(context).textTheme.bodySmall,
          ),
        ],
      ),
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

    // TODO: Call Rust FFI to start actual recording
    // For now, simulate recording with a timer
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
    // TODO: Navigate to settings screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Settings coming soon')),
    );
  }

  void _showHistory(BuildContext context) {
    // TODO: Navigate to history screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('History coming soon')),
    );
  }
}
