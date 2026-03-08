/// Floating Capsule Widget
///
/// The main desktop floating capsule that shows recording state,
/// audio visualization, and processing status.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../state/session_state.dart';
import 'audio_visualizer.dart';
import 'intensity_selector.dart';

/// Floating capsule widget with four states:
/// - Idle: Ready to record
/// - Recording: Shows audio waveform
/// - Processing: Shows spinner
/// - Error: Shows error message with retry/dismiss buttons
class FloatingCapsule extends ConsumerWidget {
  const FloatingCapsule({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final sessionData = ref.watch(sessionStateNotifierProvider);

    return AnimatedContainer(
      duration: const Duration(milliseconds: 200),
      width: _getWidth(sessionData.state),
      height: 64,
      decoration: BoxDecoration(
        color: _getBackgroundColor(sessionData.state),
        borderRadius: BorderRadius.circular(32),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.2),
            blurRadius: 16,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: _buildContent(context, ref, sessionData),
    );
  }

  Widget _buildContent(
    BuildContext context,
    WidgetRef ref,
    SessionData sessionData,
  ) {
    switch (sessionData.state) {
      case SessionState.idle:
        return const _IdleContent();
      case SessionState.recording:
        return const _RecordingContent();
      case SessionState.processing:
        return _ProcessingContent(progress: sessionData.progress);
      case SessionState.error:
        return _ErrorContent(
          errorMessage: sessionData.errorMessage ?? 'Unknown error',
          onRetry: () {
            ref.read(sessionStateNotifierProvider.notifier).retry();
          },
          onDismiss: () {
            ref.read(sessionStateNotifierProvider.notifier).dismissError();
          },
        );
    }
  }

  double _getWidth(SessionState state) {
    switch (state) {
      case SessionState.idle:
        return 200;
      case SessionState.recording:
        return 200;
      case SessionState.processing:
        return 160;
      case SessionState.error:
        return 280;
    }
  }

  Color _getBackgroundColor(SessionState state) {
    switch (state) {
      case SessionState.idle:
        return Colors.grey[800]!;
      case SessionState.recording:
        return Colors.red[400]!;
      case SessionState.processing:
        return Colors.blue[400]!;
      case SessionState.error:
        return Colors.orange[400]!;
    }
  }
}

/// Idle state content
class _IdleContent extends ConsumerWidget {
  const _IdleContent();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Center(
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(Icons.mic, color: Colors.white70, size: 20),
          const SizedBox(width: 8),
          const Text(
            'Press to talk',
            style: TextStyle(
              color: Colors.white70,
              fontSize: 14,
            ),
          ),
          const SizedBox(width: 12),
          const IntensityIndicator(),
        ],
      ),
    );
  }
}

/// Recording state content with audio visualizer
class _RecordingContent extends StatelessWidget {
  const _RecordingContent();

  @override
  Widget build(BuildContext context) {
    return const Center(
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          AudioVisualizer(width: 80, height: 24),
          SizedBox(width: 12),
          Icon(Icons.mic, color: Colors.white, size: 20),
        ],
      ),
    );
  }
}

/// Processing state content with spinner
class _ProcessingContent extends StatelessWidget {
  final double? progress;

  const _ProcessingContent({this.progress});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const SizedBox(
            width: 20,
            height: 20,
            child: CircularProgressIndicator(
              strokeWidth: 2,
              valueColor: AlwaysStoppedAnimation<Color>(Colors.white),
            ),
          ),
          const SizedBox(width: 12),
          Text(
            progress != null ? '${(progress! * 100).toInt()}%' : 'Processing',
            style: const TextStyle(
              color: Colors.white,
              fontSize: 14,
            ),
          ),
        ],
      ),
    );
  }
}

/// Error state content with retry/dismiss buttons
class _ErrorContent extends StatelessWidget {
  final String errorMessage;
  final VoidCallback onRetry;
  final VoidCallback onDismiss;

  const _ErrorContent({
    required this.errorMessage,
    required this.onRetry,
    required this.onDismiss,
  });

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: [
          const Icon(Icons.error_outline, color: Colors.white, size: 20),
          Expanded(
            child: Text(
              errorMessage,
              style: const TextStyle(color: Colors.white, fontSize: 12),
              overflow: TextOverflow.ellipsis,
            ),
          ),
          TextButton(
            onPressed: onRetry,
            style: TextButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              minimumSize: const Size(50, 24),
            ),
            child: const Text('Retry', style: TextStyle(fontSize: 12)),
          ),
          TextButton(
            onPressed: onDismiss,
            style: TextButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              minimumSize: const Size(50, 24),
            ),
            child: const Text('Dismiss', style: TextStyle(fontSize: 12)),
          ),
        ],
      ),
    );
  }
}