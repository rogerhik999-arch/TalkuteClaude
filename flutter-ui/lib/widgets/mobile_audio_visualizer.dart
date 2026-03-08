/// Mobile Audio Visualizer
///
/// A simplified audio visualization widget for mobile platforms.

import 'dart:math';
import 'package:flutter/material.dart';

/// Mobile-optimized audio visualizer widget
class MobileAudioVisualizer extends StatefulWidget {
  final double width;
  final double height;

  const MobileAudioVisualizer({
    super.key,
    required this.width,
    required this.height,
  });

  @override
  State<MobileAudioVisualizer> createState() => _MobileAudioVisualizerState();
}

class _MobileAudioVisualizerState extends State<MobileAudioVisualizer>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  final List<double> _barHeights = List.generate(20, (_) => 0.3);
  final Random _random = Random();

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 100),
    )..repeat();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _controller,
      builder: (context, child) {
        // Update bar heights with random values for demo
        for (int i = 0; i < _barHeights.length; i++) {
          _barHeights[i] = 0.2 + _random.nextDouble() * 0.8;
        }

        return CustomPaint(
          size: Size(widget.width, widget.height),
          painter: _BarPainter(
            barHeights: _barHeights,
            color: Theme.of(context).colorScheme.primary,
          ),
        );
      },
    );
  }
}

class _BarPainter extends CustomPainter {
  final List<double> barHeights;
  final Color color;

  _BarPainter({
    required this.barHeights,
    required this.color,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = color
      ..style = PaintingStyle.fill;

    const barCount = 20;
    final barWidth = size.width / barCount;
    const barSpacing = 2.0;
    const cornerRadius = 2.0;

    for (int i = 0; i < barCount; i++) {
      final x = i * barWidth;
      final height = barHeights[i] * size.height;
      final y = (size.height - height) / 2;

      final rect = RRect.fromRectAndRadius(
        Rect.fromLTWH(
          x + barSpacing / 2,
          y,
          barWidth - barSpacing,
          height,
        ),
        const Radius.circular(cornerRadius),
      );

      canvas.drawRRect(rect, paint);
    }
  }

  @override
  bool shouldRepaint(covariant _BarPainter oldDelegate) {
    return barHeights != oldDelegate.barHeights || color != oldDelegate.color;
  }
}

/// Compact waveform indicator for keyboard accessory
class CompactWaveform extends StatefulWidget {
  final bool isActive;
  final Color? color;

  const CompactWaveform({
    super.key,
    this.isActive = false,
    this.color,
  });

  @override
  State<CompactWaveform> createState() => _CompactWaveformState();
}

class _CompactWaveformState extends State<CompactWaveform>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 150),
    );
    if (widget.isActive) {
      _controller.repeat();
    }
  }

  @override
  void didUpdateWidget(covariant CompactWaveform oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isActive && !oldWidget.isActive) {
      _controller.repeat();
    } else if (!widget.isActive && oldWidget.isActive) {
      _controller.stop();
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final color = widget.color ?? Theme.of(context).colorScheme.primary;

    return AnimatedBuilder(
      animation: _controller,
      builder: (context, child) {
        return Row(
          mainAxisSize: MainAxisSize.min,
          children: List.generate(5, (index) {
            final height = widget.isActive
                ? 4.0 + (_controller.value * 8.0).abs() * ((index % 2) + 1)
                : 4.0;
            return Container(
              width: 3,
              height: height,
              margin: const EdgeInsets.symmetric(horizontal: 1),
              decoration: BoxDecoration(
                color: color,
                borderRadius: BorderRadius.circular(1.5),
              ),
            );
          }),
        );
      },
    );
  }
}