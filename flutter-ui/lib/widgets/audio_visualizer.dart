/// Audio Visualizer Widget
///
/// Displays real-time audio waveform visualization.

import 'dart:math';
import 'package:flutter/material.dart';

/// Audio visualizer showing animated bars representing audio levels
class AudioVisualizer extends StatefulWidget {
  final double width;
  final double height;
  final int barCount;
  final Color color;

  const AudioVisualizer({
    super.key,
    this.width = 100,
    this.height = 30,
    this.barCount = 5,
    this.color = Colors.white,
  });

  @override
  State<AudioVisualizer> createState() => _AudioVisualizerState();
}

class _AudioVisualizerState extends State<AudioVisualizer>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  final List<double> _barHeights = [];
  final Random _random = Random();

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 100),
    )..repeat();

    // Initialize bar heights
    for (int i = 0; i < widget.barCount; i++) {
      _barHeights.add(0.3);
    }
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
        // Update bar heights with random values to simulate audio
        for (int i = 0; i < _barHeights.length; i++) {
          _barHeights[i] = 0.2 + _random.nextDouble() * 0.8;
        }

        return SizedBox(
          width: widget.width,
          height: widget.height,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: List.generate(widget.barCount, (index) {
              return Container(
                width: widget.width / (widget.barCount * 2),
                height: widget.height * _barHeights[index],
                decoration: BoxDecoration(
                  color: widget.color.withOpacity(0.7 + _barHeights[index] * 0.3),
                  borderRadius: BorderRadius.circular(2),
                ),
              );
            }),
          ),
        );
      },
    );
  }
}