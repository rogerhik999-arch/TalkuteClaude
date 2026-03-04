/// Application context model

class ApplicationContext {
  final String contextId;
  final String applicationName;
  final String? applicationTitle;
  final String applicationCategory;
  final String? preferredTone;
  final int usageCount;
  final DateTime detectedAt;
  final DateTime lastUsedAt;
  final String? customInstructions;

  ApplicationContext({
    required this.contextId,
    required this.applicationName,
    this.applicationTitle,
    this.applicationCategory = 'other',
    this.preferredTone,
    this.usageCount = 1,
    required this.detectedAt,
    required this.lastUsedAt,
    this.customInstructions,
  });

  factory ApplicationContext.create(String applicationName, String category) {
    final now = DateTime.now();
    return ApplicationContext(
      contextId: applicationName.hashCode.toString(),
      applicationName: applicationName,
      applicationCategory: category,
      detectedAt: now,
      lastUsedAt: now,
    );
  }

  ApplicationContext copyWith({
    String? contextId,
    String? applicationName,
    String? applicationTitle,
    String? applicationCategory,
    String? preferredTone,
    int? usageCount,
    DateTime? detectedAt,
    DateTime? lastUsedAt,
    String? customInstructions,
  }) {
    return ApplicationContext(
      contextId: contextId ?? this.contextId,
      applicationName: applicationName ?? this.applicationName,
      applicationTitle: applicationTitle ?? this.applicationTitle,
      applicationCategory: applicationCategory ?? this.applicationCategory,
      preferredTone: preferredTone ?? this.preferredTone,
      usageCount: usageCount ?? this.usageCount,
      detectedAt: detectedAt ?? this.detectedAt,
      lastUsedAt: lastUsedAt ?? this.lastUsedAt,
      customInstructions: customInstructions ?? this.customInstructions,
    );
  }

  ApplicationContext incrementUsage() {
    return copyWith(
      usageCount: usageCount + 1,
      lastUsedAt: DateTime.now(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'contextId': contextId,
      'applicationName': applicationName,
      'applicationTitle': applicationTitle,
      'applicationCategory': applicationCategory,
      'preferredTone': preferredTone,
      'usageCount': usageCount,
      'detectedAt': detectedAt.toIso8601String(),
      'lastUsedAt': lastUsedAt.toIso8601String(),
      'customInstructions': customInstructions,
    };
  }

  factory ApplicationContext.fromJson(Map<String, dynamic> json) {
    return ApplicationContext(
      contextId: json['contextId'] as String,
      applicationName: json['applicationName'] as String,
      applicationTitle: json['applicationTitle'] as String?,
      applicationCategory: json['applicationCategory'] as String? ?? 'other',
      preferredTone: json['preferredTone'] as String?,
      usageCount: json['usageCount'] as int? ?? 1,
      detectedAt: DateTime.parse(json['detectedAt'] as String),
      lastUsedAt: DateTime.parse(json['lastUsedAt'] as String),
      customInstructions: json['customInstructions'] as String?,
    );
  }
}

/// Application category enum
enum ApplicationCategory {
  email,
  chat,
  document,
  code,
  browser,
  other,
}

extension ApplicationCategoryExtension on ApplicationCategory {
  String get name {
    switch (this) {
      case ApplicationCategory.email:
        return 'Email';
      case ApplicationCategory.chat:
        return 'Chat';
      case ApplicationCategory.document:
        return 'Document';
      case ApplicationCategory.code:
        return 'Code';
      case ApplicationCategory.browser:
        return 'Browser';
      case ApplicationCategory.other:
        return 'Other';
    }
  }

  String get value {
    switch (this) {
      case ApplicationCategory.email:
        return 'EMAIL';
      case ApplicationCategory.chat:
        return 'CHAT';
      case ApplicationCategory.document:
        return 'DOCUMENT';
      case ApplicationCategory.code:
        return 'CODE';
      case ApplicationCategory.browser:
        return 'BROWSER';
      case ApplicationCategory.other:
        return 'OTHER';
    }
  }
}

/// Tone preference enum
enum TonePreference {
  formal,
  casual,
  technical,
  creative,
}

extension TonePreferenceExtension on TonePreference {
  String get value {
    switch (this) {
      case TonePreference.formal:
        return 'FORMAL';
      case TonePreference.casual:
        return 'CASUAL';
      case TonePreference.technical:
        return 'TECHNICAL';
      case TonePreference.creative:
        return 'CREATIVE';
    }
  }
}
