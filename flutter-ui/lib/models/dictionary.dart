/// Personal dictionary entry model
///
/// Represents a custom term in the user's personal dictionary.

/// Dictionary entry category
enum DictionaryCategory {
  technical,
  business,
  medical,
  general;

  String get displayName {
    switch (this) {
      case DictionaryCategory.technical:
        return 'Technical';
      case DictionaryCategory.business:
        return 'Business';
      case DictionaryCategory.medical:
        return 'Medical';
      case DictionaryCategory.general:
        return 'General';
    }
  }

  static DictionaryCategory fromString(String value) {
    switch (value.toLowerCase()) {
      case 'technical':
        return DictionaryCategory.technical;
      case 'business':
        return DictionaryCategory.business;
      case 'medical':
        return DictionaryCategory.medical;
      default:
        return DictionaryCategory.general;
    }
  }
}

/// Personal dictionary entry
class PersonalDictionaryEntry {
  final String entryId;
  final String deviceId;
  final String phrase;
  final String replacement;
  final bool caseSensitive;
  final bool wholeWordOnly;
  final DictionaryCategory category;
  final DateTime createdAt;
  final DateTime? lastUsedAt;
  final int usageCount;

  const PersonalDictionaryEntry({
    required this.entryId,
    required this.deviceId,
    required this.phrase,
    required this.replacement,
    this.caseSensitive = false,
    this.wholeWordOnly = true,
    this.category = DictionaryCategory.general,
    required this.createdAt,
    this.lastUsedAt,
    this.usageCount = 0,
  });

  /// Create a copy with updated fields
  PersonalDictionaryEntry copyWith({
    String? entryId,
    String? deviceId,
    String? phrase,
    String? replacement,
    bool? caseSensitive,
    bool? wholeWordOnly,
    DictionaryCategory? category,
    DateTime? createdAt,
    DateTime? lastUsedAt,
    bool clearLastUsedAt = false,
    int? usageCount,
  }) {
    return PersonalDictionaryEntry(
      entryId: entryId ?? this.entryId,
      deviceId: deviceId ?? this.deviceId,
      phrase: phrase ?? this.phrase,
      replacement: replacement ?? this.replacement,
      caseSensitive: caseSensitive ?? this.caseSensitive,
      wholeWordOnly: wholeWordOnly ?? this.wholeWordOnly,
      category: category ?? this.category,
      createdAt: createdAt ?? this.createdAt,
      lastUsedAt: clearLastUsedAt ? null : (lastUsedAt ?? this.lastUsedAt),
      usageCount: usageCount ?? this.usageCount,
    );
  }

  /// Convert to JSON
  Map<String, dynamic> toJson() {
    return {
      'entryId': entryId,
      'deviceId': deviceId,
      'phrase': phrase,
      'replacement': replacement,
      'caseSensitive': caseSensitive,
      'wholeWordOnly': wholeWordOnly,
      'category': category.name,
      'createdAt': createdAt.toIso8601String(),
      'lastUsedAt': lastUsedAt?.toIso8601String(),
      'usageCount': usageCount,
    };
  }

  /// Create from JSON
  factory PersonalDictionaryEntry.fromJson(Map<String, dynamic> json) {
    return PersonalDictionaryEntry(
      entryId: json['entryId'] as String,
      deviceId: json['deviceId'] as String,
      phrase: json['phrase'] as String,
      replacement: json['replacement'] as String,
      caseSensitive: json['caseSensitive'] as bool? ?? false,
      wholeWordOnly: json['wholeWordOnly'] as bool? ?? true,
      category: DictionaryCategory.fromString(json['category'] as String? ?? 'general'),
      createdAt: DateTime.parse(json['createdAt'] as String),
      lastUsedAt: json['lastUsedAt'] != null
          ? DateTime.parse(json['lastUsedAt'] as String)
          : null,
      usageCount: json['usageCount'] as int? ?? 0,
    );
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is PersonalDictionaryEntry && other.entryId == entryId;
  }

  @override
  int get hashCode => entryId.hashCode;

  @override
  String toString() {
    return 'PersonalDictionaryEntry(entryId: $entryId, phrase: $phrase, replacement: $replacement)';
  }
}