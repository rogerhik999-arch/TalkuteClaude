//! Language detection module
//!
//! Provides automatic language detection from text and audio.

use std::collections::HashMap;

/// Language detection result
#[derive(Debug, Clone)]
pub struct LanguageDetectionResult {
    /// Detected language code (e.g., "en", "zh", "es")
    pub language: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
}

/// Language detector for text and audio
pub struct LanguageDetector {
    /// Language profiles for detection
    profiles: HashMap<String, LanguageProfile>,
}

/// Language profile containing characteristic patterns
#[derive(Debug, Clone)]
struct LanguageProfile {
    /// Common character ranges for the language
    char_ranges: Vec<(char, char)>,
    /// Common words/patterns for the language
    common_patterns: Vec<String>,
    /// Relative frequency weights
    weight: f32,
}

impl LanguageDetector {
    /// Create a new language detector
    pub fn new() -> Self {
        Self {
            profiles: Self::build_profiles(),
        }
    }

    /// Build language profiles for detection
    fn build_profiles() -> HashMap<String, LanguageProfile> {
        let mut profiles = HashMap::new();

        // English profile
        profiles.insert("en".to_string(), LanguageProfile {
            char_ranges: vec![('a', 'z'), ('A', 'Z')],
            common_patterns: vec![
                "the".to_string(), "be".to_string(), "to".to_string(),
                "of".to_string(), "and".to_string(), "a".to_string(),
                "in".to_string(), "that".to_string(), "have".to_string(),
                "I".to_string(), "it".to_string(), "for".to_string(),
                "not".to_string(), "on".to_string(), "with".to_string(),
            ],
            weight: 1.0,
        });

        // Chinese profile
        profiles.insert("zh".to_string(), LanguageProfile {
            char_ranges: vec![
                ('\u{4E00}', '\u{9FFF}'), // CJK Unified Ideographs
                ('\u{3400}', '\u{4DBF}'), // CJK Extension A
            ],
            common_patterns: vec![
                "的".to_string(), "是".to_string(), "不".to_string(),
                "我".to_string(), "你".to_string(), "他".to_string(),
                "这".to_string(), "那".to_string(), "有".to_string(),
                "和".to_string(), "就".to_string(),
            ],
            weight: 1.0,
        });

        // Spanish profile
        profiles.insert("es".to_string(), LanguageProfile {
            char_ranges: vec![('a', 'z'), ('A', 'Z')],
            common_patterns: vec![
                "el".to_string(), "la".to_string(), "de".to_string(),
                "que".to_string(), "y".to_string(), "a".to_string(),
                "en".to_string(), "un".to_string(), "ser".to_string(),
                "se".to_string(), "no".to_string(), "haber".to_string(),
            ],
            weight: 1.0,
        });

        // Japanese profile
        profiles.insert("ja".to_string(), LanguageProfile {
            char_ranges: vec![
                ('\u{3040}', '\u{309F}'), // Hiragana
                ('\u{30A0}', '\u{30FF}'), // Katakana
                ('\u{4E00}', '\u{9FFF}'), // Kanji
            ],
            common_patterns: vec![
                "の".to_string(), "に".to_string(), "は".to_string(),
                "を".to_string(), "た".to_string(), "が".to_string(),
                "で".to_string(), "て".to_string(), "と".to_string(),
                "し".to_string(), "れ".to_string(), "さ".to_string(),
            ],
            weight: 1.0,
        });

        // German profile
        profiles.insert("de".to_string(), LanguageProfile {
            char_ranges: vec![('a', 'z'), ('A', 'Z')],
            common_patterns: vec![
                "der".to_string(), "die".to_string(), "und".to_string(),
                "in".to_string(), "den".to_string(), "von".to_string(),
                "zu".to_string(), "das".to_string(), "mit".to_string(),
                "sich".to_string(), "des".to_string(), "auf".to_string(),
            ],
            weight: 1.0,
        });

        // French profile
        profiles.insert("fr".to_string(), LanguageProfile {
            char_ranges: vec![('a', 'z'), ('A', 'Z')],
            common_patterns: vec![
                "le".to_string(), "de".to_string(), "un".to_string(),
                "être".to_string(), "et".to_string(), "à".to_string(),
                "il".to_string(), "la".to_string(), "les".to_string(),
                "des".to_string(), "en".to_string(), "que".to_string(),
            ],
            weight: 1.0,
        });

        // Korean profile
        profiles.insert("ko".to_string(), LanguageProfile {
            char_ranges: vec![
                ('\u{AC00}', '\u{D7AF}'), // Hangul Syllables
                ('\u{1100}', '\u{11FF}'), // Hangul Jamo
            ],
            common_patterns: vec![
                "이".to_string(), "그".to_string(), "저".to_string(),
                "를".to_string(), "은".to_string(), "는".to_string(),
                "에".to_string(), "와".to_string(), "과".to_string(),
            ],
            weight: 1.0,
        });

        profiles
    }

    /// Detect language from text
    pub fn detect_from_text(&self, text: &str) -> LanguageDetectionResult {
        if text.is_empty() {
            return LanguageDetectionResult {
                language: "en".to_string(),
                confidence: 0.0,
            };
        }

        // Check for CJK characters first (high confidence detection)
        let cjk_result = self.detect_cjk(text);
        if cjk_result.confidence > 0.8 {
            return cjk_result;
        }

        // Score each language based on patterns
        let mut scores: HashMap<String, f32> = HashMap::new();

        for (lang, profile) in &self.profiles {
            let score = self.score_language(text, profile);
            scores.insert(lang.clone(), score);
        }

        // Find the language with highest score
        let (language, score) = scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap_or(("en".to_string(), 0.0));

        // Normalize confidence
        let confidence = (score * 2.0).min(1.0);

        LanguageDetectionResult { language, confidence }
    }

    /// Detect CJK languages based on character ranges
    fn detect_cjk(&self, text: &str) -> LanguageDetectionResult {
        let mut cjk_counts: HashMap<String, usize> = HashMap::new();
        let mut total_cjk = 0;

        for ch in text.chars() {
            // Check for Japanese-specific characters
            if ('\u{3040}'..='\u{309F}').contains(&ch) || // Hiragana
               ('\u{30A0}'..='\u{30FF}').contains(&ch) {   // Katakana
                *cjk_counts.entry("ja".to_string()).or_insert(0) += 1;
                total_cjk += 1;
            }
            // Check for Korean Hangul
            else if ('\u{AC00}'..='\u{D7AF}').contains(&ch) {
                *cjk_counts.entry("ko".to_string()).or_insert(0) += 1;
                total_cjk += 1;
            }
            // Check for Chinese/Japanese Kanji
            else if ('\u{4E00}'..='\u{9FFF}').contains(&ch) {
                // Could be Chinese or Japanese - need to check for hiragana/katakana
                *cjk_counts.entry("zh".to_string()).or_insert(0) += 1;
                total_cjk += 1;
            }
        }

        if total_cjk == 0 {
            return LanguageDetectionResult {
                language: "en".to_string(),
                confidence: 0.0,
            };
        }

        // Find dominant CJK language
        let (language, count) = cjk_counts
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or(("zh".to_string(), 0));

        // Check for Japanese (hiragana/katakana present)
        if language == "ja" || (language == "zh" && self.has_japanese_specific(text)) {
            return LanguageDetectionResult {
                language: "ja".to_string(),
                confidence: 0.95,
            };
        }

        // Check for Korean
        if language == "ko" {
            return LanguageDetectionResult {
                language: "ko".to_string(),
                confidence: 0.95,
            };
        }

        // Default to Chinese for CJK characters
        LanguageDetectionResult {
            language: "zh".to_string(),
            confidence: (count as f32 / total_cjk as f32).min(0.95),
        }
    }

    /// Check if text has Japanese-specific characters
    fn has_japanese_specific(&self, text: &str) -> bool {
        text.chars().any(|ch| {
            ('\u{3040}'..='\u{309F}').contains(&ch) || // Hiragana
            ('\u{30A0}'..='\u{30FF}').contains(&ch)    // Katakana
        })
    }

    /// Score a language based on patterns
    fn score_language(&self, text: &str, profile: &LanguageProfile) -> f32 {
        let text_lower = text.to_lowercase();
        let mut score = 0.0;

        // Count matching patterns with word boundary check
        let words: Vec<&str> = text.split_whitespace().collect();
        for word in &words {
            let word_lower = word.to_lowercase();
            for pattern in &profile.common_patterns {
                if word_lower == pattern.to_lowercase() {
                    score += profile.weight * 2.0; // Exact match bonus
                } else if text_lower.contains(&pattern.to_lowercase()) {
                    score += profile.weight * 0.5;
                }
            }
        }

        // Normalize by text length
        let word_count = words.len().max(1);
        score / word_count as f32
    }

    /// Detect language with hints (narrow down candidates)
    pub fn detect_with_hints(&self, text: &str, hints: &[String]) -> LanguageDetectionResult {
        let result = self.detect_from_text(text);

        // If detected language is in hints, return it
        if hints.contains(&result.language) {
            return result;
        }

        // Otherwise, return the first hint with lower confidence
        if !hints.is_empty() {
            return LanguageDetectionResult {
                language: hints[0].clone(),
                confidence: 0.5,
            };
        }

        result
    }

    /// Get list of supported languages
    pub fn get_supported_languages(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_english() {
        let detector = LanguageDetector::new();
        let result = detector.detect_from_text("Hello, how are you doing today?");
        assert_eq!(result.language, "en");
    }

    #[test]
    fn test_detect_chinese() {
        let detector = LanguageDetector::new();
        let result = detector.detect_from_text("你好，今天天气怎么样？");
        assert_eq!(result.language, "zh");
    }

    #[test]
    fn test_detect_japanese() {
        let detector = LanguageDetector::new();
        let result = detector.detect_from_text("こんにちは、お元気ですか？");
        assert_eq!(result.language, "ja");
    }

    #[test]
    fn test_detect_korean() {
        let detector = LanguageDetector::new();
        let result = detector.detect_from_text("안녕하세요, 오늘 날씨가 어떤가요?");
        assert_eq!(result.language, "ko");
    }

    #[test]
    fn test_supported_languages() {
        let detector = LanguageDetector::new();
        let languages = detector.get_supported_languages();
        assert!(languages.contains(&"en".to_string()));
        assert!(languages.contains(&"zh".to_string()));
        assert!(languages.contains(&"ja".to_string()));
        assert!(languages.contains(&"ko".to_string()));
    }
}