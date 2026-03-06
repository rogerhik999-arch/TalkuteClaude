/// Language model for multi-language support
///
/// Represents a language option for voice input and translation.

/// Supported language for voice input and translation
class SupportedLanguage {
  /// Language code (e.g., "en", "zh", "ja")
  final String code;

  /// Display name in English (e.g., "English", "Chinese")
  final String name;

  /// Native name (e.g., "English", "中文", "日本語")
  final String nativeName;

  /// Flag emoji (e.g., "🇺🇸", "🇨🇳", "🇯🇵")
  final String flagEmoji;

  const SupportedLanguage({
    required this.code,
    required this.name,
    required this.nativeName,
    required this.flagEmoji,
  });

  /// Create from JSON
  factory SupportedLanguage.fromJson(Map<String, dynamic> json) {
    return SupportedLanguage(
      code: json['code'] as String,
      name: json['name'] as String,
      nativeName: json['nativeName'] as String,
      flagEmoji: json['flagEmoji'] as String,
    );
  }

  /// Convert to JSON
  Map<String, dynamic> toJson() {
    return {
      'code': code,
      'name': name,
      'nativeName': nativeName,
      'flagEmoji': flagEmoji,
    };
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is SupportedLanguage && other.code == code;
  }

  @override
  int get hashCode => code.hashCode;

  @override
  String toString() => 'SupportedLanguage($code: $name)';
}

/// Predefined supported languages
class Languages {
  static const List<SupportedLanguage> all = [
    SupportedLanguage(
      code: 'en',
      name: 'English',
      nativeName: 'English',
      flagEmoji: '🇺🇸',
    ),
    SupportedLanguage(
      code: 'zh',
      name: 'Chinese',
      nativeName: '中文',
      flagEmoji: '🇨🇳',
    ),
    SupportedLanguage(
      code: 'es',
      name: 'Spanish',
      nativeName: 'Español',
      flagEmoji: '🇪🇸',
    ),
    SupportedLanguage(
      code: 'ja',
      name: 'Japanese',
      nativeName: '日本語',
      flagEmoji: '🇯🇵',
    ),
    SupportedLanguage(
      code: 'de',
      name: 'German',
      nativeName: 'Deutsch',
      flagEmoji: '🇩🇪',
    ),
    SupportedLanguage(
      code: 'fr',
      name: 'French',
      nativeName: 'Français',
      flagEmoji: '🇫🇷',
    ),
    SupportedLanguage(
      code: 'ko',
      name: 'Korean',
      nativeName: '한국어',
      flagEmoji: '🇰🇷',
    ),
    SupportedLanguage(
      code: 'pt',
      name: 'Portuguese',
      nativeName: 'Português',
      flagEmoji: '🇧🇷',
    ),
    SupportedLanguage(
      code: 'it',
      name: 'Italian',
      nativeName: 'Italiano',
      flagEmoji: '🇮🇹',
    ),
    SupportedLanguage(
      code: 'ru',
      name: 'Russian',
      nativeName: 'Русский',
      flagEmoji: '🇷🇺',
    ),
  ];

  /// Find language by code
  static SupportedLanguage? byCode(String code) {
    try {
      return all.firstWhere((lang) => lang.code == code);
    } catch (_) {
      return null;
    }
  }

  /// Get default language (English)
  static SupportedLanguage get defaultLanguage => all.first;
}