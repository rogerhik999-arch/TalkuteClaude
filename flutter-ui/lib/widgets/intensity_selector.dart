/// Polishing Intensity Selector Widget
///
/// Allows users to select between Light, Standard, and Deep polishing modes.

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Polishing intensity levels
enum PolishingIntensity {
  light,
  standard,
  deep,
}

/// Provider for polishing intensity
final polishingIntensityProvider = StateProvider<PolishingIntensity>((ref) => PolishingIntensity.standard);

/// Extension for PolishingIntensity
extension PolishingIntensityExtension on PolishingIntensity {
  /// Get display name
  String get displayName => switch (this) {
    PolishingIntensity.light => 'Light',
    PolishingIntensity.standard => 'Standard',
    PolishingIntensity.deep => 'Deep',
  };

  /// Get description
  String get description => switch (this) {
    PolishingIntensity.light => 'Filler removal only',
    PolishingIntensity.standard => 'Grammar & logic improvements',
    PolishingIntensity.deep => 'Full rewrite',
  };

  /// Get icon
  IconData get icon => switch (this) {
    PolishingIntensity.light => Icons.tune_outlined,
    PolishingIntensity.standard => Icons.auto_fix_high,
    PolishingIntensity.deep => Icons.auto_awesome,
  };

  /// Parse from string
  static PolishingIntensity? fromString(String s) => switch (s.toLowerCase()) {
    'light' => PolishingIntensity.light,
    'standard' => PolishingIntensity.standard,
    'deep' => PolishingIntensity.deep,
    _ => null,
  };
}

/// Intensity selector widget for settings
class IntensitySelector extends ConsumerWidget {
  const IntensitySelector({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final intensity = ref.watch(polishingIntensityProvider);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Padding(
          padding: EdgeInsets.fromLTRB(16, 16, 16, 8),
          child: Text(
            'Polishing Intensity',
            style: TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
              color: Colors.grey,
            ),
          ),
        ),
        ...PolishingIntensity.values.map((i) => RadioListTile<PolishingIntensity>(
          title: Row(
            children: [
              Icon(i.icon, size: 20),
              const SizedBox(width: 12),
              Text(i.displayName),
            ],
          ),
          subtitle: Text(i.description),
          value: i,
          groupValue: intensity,
          onChanged: (value) {
            if (value != null) {
              ref.read(polishingIntensityProvider.notifier).state = value;
            }
          },
        )),
      ],
    );
  }
}

/// Quick intensity toggle for floating capsule
class QuickIntensityToggle extends ConsumerWidget {
  const QuickIntensityToggle({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final intensity = ref.watch(polishingIntensityProvider);

    return Container(
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(20),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: PolishingIntensity.values.map((i) {
          final isSelected = i == intensity;
          return GestureDetector(
            onTap: () {
              ref.read(polishingIntensityProvider.notifier).state = i;
            },
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
              decoration: BoxDecoration(
                color: isSelected
                    ? Theme.of(context).colorScheme.primary
                    : Colors.transparent,
                borderRadius: BorderRadius.circular(16),
              ),
              child: Text(
                i.displayName,
                style: TextStyle(
                  color: isSelected
                      ? Theme.of(context).colorScheme.onPrimary
                      : Theme.of(context).colorScheme.onSurface,
                  fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                  fontSize: 12,
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }
}

/// Compact intensity indicator
class IntensityIndicator extends ConsumerWidget {
  const IntensityIndicator({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final intensity = ref.watch(polishingIntensityProvider);

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.primaryContainer,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(
            intensity.icon,
            size: 14,
            color: Theme.of(context).colorScheme.onPrimaryContainer,
          ),
          const SizedBox(width: 4),
          Text(
            intensity.displayName,
            style: TextStyle(
              fontSize: 11,
              color: Theme.of(context).colorScheme.onPrimaryContainer,
            ),
          ),
        ],
      ),
    );
  }
}