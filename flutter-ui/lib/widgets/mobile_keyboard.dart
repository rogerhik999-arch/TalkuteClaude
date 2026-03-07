/// Mobile Keyboard Widget
///
/// A custom keyboard widget for mobile platforms with voice input support.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'mobile_audio_visualizer.dart';
import 'intensity_selector.dart';
import '../state/session_state.dart';

/// Mobile keyboard widget with voice input
class MobileKeyboardWidget extends ConsumerStatefulWidget {
  final Function(String)? onTextInserted;
  final Function()? onRequestExpand;

  const MobileKeyboardWidget({
    super.key,
    this.onTextInserted,
    this.onRequestExpand,
  });

  @override
  ConsumerState<MobileKeyboardWidget> createState() => _MobileKeyboardWidgetState();
}

class _MobileKeyboardWidgetState extends ConsumerState<MobileKeyboardWidget> {
  bool _isExpanded = false;

  @override
  Widget build(BuildContext context) {
    final sessionData = ref.watch(sessionStateNotifierProvider);

    return Container(
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        border: Border(
          top: BorderSide(
            color: Theme.of(context).dividerColor,
            width: 0.5,
          ),
        ),
      ),
      child: SafeArea(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (_isExpanded) _buildExpandedView(context, sessionData),
            _buildKeyboardRow(context, sessionData),
          ],
        ),
      ),
    );
  }

  Widget _buildExpandedView(BuildContext context, SessionData sessionData) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 200),
      height: 120,
      child: _buildExpandedContent(context, sessionData),
    );
  }

  Widget _buildExpandedContent(BuildContext context, SessionData sessionData) {
    switch (sessionData.state) {
      case SessionState.idle:
        return const Center(
          child: Text(
            'Tap microphone to start voice input',
            style: TextStyle(color: Colors.grey),
          ),
        );
      case SessionState.recording:
        return Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const MobileAudioVisualizer(width: 200, height: 40),
            const SizedBox(height: 8),
            Text(
              'Listening...',
              style: TextStyle(color: Theme.of(context).colorScheme.primary),
            ),
          ],
        );
      case SessionState.processing:
        return const Center(
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              SizedBox(
                width: 24,
                height: 24,
                child: CircularProgressIndicator(strokeWidth: 2),
              ),
              SizedBox(width: 12),
              Text('Processing...'),
            ],
          ),
        );
      case SessionState.error:
        return Center(
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Icon(Icons.error_outline, color: Colors.red),
              const SizedBox(width: 8),
              Text(sessionData.errorMessage ?? 'Error'),
              const SizedBox(width: 8),
              TextButton(
                onPressed: () {
                  ref.read(sessionStateNotifierProvider.notifier).retry();
                },
                child: const Text('Retry'),
              ),
            ],
          ),
        );
    }
  }

  Widget _buildKeyboardRow(BuildContext context, SessionData sessionData) {
    return Container(
      height: 50,
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      child: Row(
        children: [
          _buildMicButton(context, sessionData),
          const SizedBox(width: 8),
          IconButton(
            icon: const Icon(Icons.translate),
            onPressed: () {},
            tooltip: 'Translate',
          ),
          IconButton(
            icon: const Icon(Icons.settings_outlined),
            onPressed: () {},
            tooltip: 'Settings',
          ),
          const Spacer(),
          TextButton(
            onPressed: () {},
            child: const Text('ABC'),
          ),
        ],
      ),
    );
  }

  Widget _buildMicButton(BuildContext context, SessionData sessionData) {
    final isRecording = sessionData.state == SessionState.recording;

    return GestureDetector(
      onTapDown: (_) {
        if (sessionData.state == SessionState.idle) {
          ref.read(sessionStateNotifierProvider.notifier).startRecording('mobile');
        }
      },
      onTapUp: (_) {
        if (sessionData.state == SessionState.recording) {
          ref.read(sessionStateNotifierProvider.notifier).startProcessing();
        }
      },
      onTapCancel: () {
        if (sessionData.state == SessionState.recording) {
          ref.read(sessionStateNotifierProvider.notifier).cancel();
        }
      },
      child: Container(
        width: 56,
        height: 44,
        decoration: BoxDecoration(
          color: isRecording
              ? Theme.of(context).colorScheme.error
              : Theme.of(context).colorScheme.primary,
          borderRadius: BorderRadius.circular(22),
        ),
        child: Icon(
          isRecording ? Icons.mic : Icons.mic_none,
          color: Theme.of(context).colorScheme.onPrimary,
        ),
      ),
    );
  }
}