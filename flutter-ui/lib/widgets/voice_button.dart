/// Voice button widget for Talkute
///
/// A button that starts/stops voice recording with visual feedback.
/// Supports both tap-to-toggle and push-to-talk modes.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../state/voice_state.dart';
import '../screens/settings_screen.dart' show pushToTalkProvider;

/// Voice input button widget
class VoiceButton extends ConsumerStatefulWidget {
  final VoidCallback? onStart;
  final VoidCallback? onStop;
  final VoidCallback? onCancel;
  final double size;
  final bool showLabel;

  const VoiceButton({
    super.key,
    this.onStart,
    this.onStop,
    this.onCancel,
    this.size = 80.0,
    this.showLabel = true,
  });

  @override
  ConsumerState<VoiceButton> createState() => _VoiceButtonState();
}

class _VoiceButtonState extends ConsumerState<VoiceButton>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  bool _isPTTActive = false;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1000),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _handleTap() {
    final state = ref.read(voiceStateProvider);
    final isPushToTalk = ref.read(pushToTalkProvider);

    // In push-to-talk mode, tap does nothing (use long press)
    if (isPushToTalk) {
      return;
    }

    if (state.isRecording) {
      widget.onStop?.call();
    } else if (state.status == VoiceRecordingStatus.idle ||
        state.status == VoiceRecordingStatus.completed ||
        state.status == VoiceRecordingStatus.error) {
      widget.onStart?.call();
    }
  }

  void _handleLongPressStart(LongPressStartDetails details) {
    final isPushToTalk = ref.read(pushToTalkProvider);
    final state = ref.read(voiceStateProvider);

    // Only start recording in push-to-talk mode
    if (isPushToTalk && !state.isRecording &&
        (state.status == VoiceRecordingStatus.idle ||
         state.status == VoiceRecordingStatus.completed ||
         state.status == VoiceRecordingStatus.error)) {
      _isPTTActive = true;
      widget.onStart?.call();
    }
  }

  void _handleLongPressEnd(LongPressEndDetails details) {
    final isPushToTalk = ref.read(pushToTalkProvider);

    // Only stop recording in push-to-talk mode
    if (isPushToTalk && _isPTTActive) {
      _isPTTActive = false;
      widget.onStop?.call();
    }
  }

  void _handleLongPress() {
    final state = ref.read(voiceStateProvider);
    if (state.isRecording || state.isProcessing) {
      widget.onCancel?.call();
    }
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(voiceStateProvider);
    final audioLevel = ref.watch(audioLevelProvider);

    // Start animation when recording
    if (state.isRecording && !_controller.isAnimating) {
      _controller.repeat();
    } else if (!state.isRecording && _controller.isAnimating) {
      _controller.stop();
      _controller.reset();
    }

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        GestureDetector(
          onTap: _handleTap,
          onLongPress: _handleLongPress,
          onLongPressStart: _handleLongPressStart,
          onLongPressEnd: _handleLongPressEnd,
          child: AnimatedBuilder(
            animation: _controller,
            builder: (context, child) {
              return Container(
                width: widget.size,
                height: widget.size,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  color: _getBackgroundColor(state, audioLevel),
                  boxShadow: [
                    BoxShadow(
                      color: _getShadowColor(state, audioLevel),
                      blurRadius: state.isRecording ? 20 + (audioLevel * 10) : 10,
                      spreadRadius: state.isRecording ? audioLevel * 5 : 0,
                    ),
                  ],
                ),
                child: _buildIcon(state),
              );
            },
          ),
        ),
        if (widget.showLabel) ...[
          const SizedBox(height: 8),
          Text(
            _getLabel(state),
            style: Theme.of(context).textTheme.bodySmall,
          ),
        ],
      ],
    );
  }

  Color _getBackgroundColor(VoiceState state, double audioLevel) {
    if (state.hasError) {
      return Colors.red;
    } else if (state.isRecording) {
      // Pulsing red based on animation and audio level
      final pulse = (_controller.value * 2 - 1).abs();
      final intensity = 0.5 + (pulse * 0.3) + (audioLevel * 0.2);
      return Color.lerp(Colors.red, Colors.redAccent, intensity)!;
    } else if (state.isProcessing) {
      return Colors.orange;
    } else if (state.status == VoiceRecordingStatus.completed) {
      return Colors.green;
    } else {
      return Theme.of(context).primaryColor;
    }
  }

  Color _getShadowColor(VoiceState state, double audioLevel) {
    if (state.isRecording) {
      return Colors.red.withOpacity(0.3 + audioLevel * 0.4);
    } else if (state.isProcessing) {
      return Colors.orange.withOpacity(0.3);
    }
    return Colors.black.withOpacity(0.2);
  }

  Widget _buildIcon(VoiceState state) {
    if (state.isProcessing) {
      return const CircularProgressIndicator(
        color: Colors.white,
        strokeWidth: 3,
      );
    }

    return Icon(
      _getIcon(state),
      color: Colors.white,
      size: widget.size * 0.5,
    );
  }

  IconData _getIcon(VoiceState state) {
    if (state.isRecording) {
      return Icons.mic;
    } else if (state.hasError) {
      return Icons.error_outline;
    } else if (state.status == VoiceRecordingStatus.completed) {
      return Icons.check;
    }
    return Icons.mic_none;
  }

  String _getLabel(VoiceState state) {
    final isPushToTalk = ref.watch(pushToTalkProvider);

    switch (state.status) {
      case VoiceRecordingStatus.idle:
        return isPushToTalk ? 'Hold to record' : 'Tap to record';
      case VoiceRecordingStatus.recording:
        return isPushToTalk ? 'Release to stop' : 'Tap to stop';
      case VoiceRecordingStatus.transcribing:
        return 'Transcribing...';
      case VoiceRecordingStatus.polishing:
        return 'Polishing...';
      case VoiceRecordingStatus.completed:
        return 'Completed';
      case VoiceRecordingStatus.error:
        return 'Error - Tap to retry';
    }
  }
}
