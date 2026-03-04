//! Filler word removal module

use regex::Regex;
use std::collections::HashSet;

/// Filler words by language
pub struct FillerWords {
    english: HashSet<&'static str>,
    chinese: HashSet<&'static str>,
    japanese: HashSet<&'static str>,
    spanish: HashSet<&'static str>,
    french: HashSet<&'static str>,
    german: HashSet<&'static str>,
}

impl FillerWords {
    /// Create a new FillerWords instance
    pub fn new() -> Self {
        Self {
            english: Self::english_fillers(),
            chinese: Self::chinese_fillers(),
            japanese: Self::japanese_fillers(),
            spanish: Self::spanish_fillers(),
            french: Self::french_fillers(),
            german: Self::german_fillers(),
        }
    }

    /// English filler words
    fn english_fillers() -> HashSet<&'static str> {
        ["um", "uh", "like", "you know", "sort of", "kind of", "well", "so", "basically", "literally"]
            .iter()
            .cloned()
            .collect()
    }

    /// Chinese filler words
    fn chinese_fillers() -> HashSet<&'static str> {
        ["嗯", "啊", "额", "那个", "这个", "就是", "然后", "就是说"]
            .iter()
            .cloned()
            .collect()
    }

    /// Japanese filler words
    fn japanese_fillers() -> HashSet<&'static str> {
        ["えっと", "あの", "実は", "結構", "わりと"]
            .iter()
            .cloned()
            .collect()
    }

    /// Spanish filler words
    fn spanish_fillers() -> HashSet<&'static str> {
        ["tipo", "pues", "eh", "este", "bueno"]
            .iter()
            .cloned()
            .collect()
    }

    /// French filler words
    fn french_fillers() -> HashSet<&'static str> {
        ["euh", "genre", "bah", "donc", "alors"]
            .iter()
            .cloned()
            .collect()
    }

    /// German filler words
    fn german_fillers() -> HashSet<&'static str> {
        ["aeh", "so", "na", "also", "halt"]
            .iter()
            .cloned()
            .collect()
    }

    /// Remove filler words from text
    pub fn remove_fillers(&self, text: &str, language: &str) -> String {
        let fillers = match language {
            "zh" | "zh-CN" => &self.chinese,
            "ja" | "ja-JP" => &self.japanese,
            "es" | "es-ES" => &self.spanish,
            "fr" | "fr-FR" => &self.french,
            "de" | "de-DE" => &self.german,
            _ => &self.english,
        };

        // Simple removal - in production, use more sophisticated NLP
        let mut result = text.to_string();
        for filler in fillers {
            result = result.replace(filler, "");
        }

        // Clean up extra spaces
        let re = Regex::new(r"\s+").unwrap();
        re.replace_all(&result, " ").trim().to_string()
    }

    /// Check if a word is a filler
    pub fn is_filler(&self, word: &str, language: &str) -> bool {
        let fillers = match language {
            "zh" | "zh-CN" => &self.chinese,
            "ja" | "ja-JP" => &self.japanese,
            "es" | "es-ES" => &self.spanish,
            "fr" | "fr-FR" => &self.french,
            "de" | "de-DE" => &self.german,
            _ => &self.english,
        };

        fillers.contains(word)
    }
}

impl Default for FillerWords {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_english_fillers() {
        let filler = FillerWords::new();
        let text = "I wanted to um schedule a meeting for tomorrow";
        let cleaned = filler.remove_fillers(text, "en");

        assert!(!cleaned.contains("um"));
    }

    #[test]
    fn test_remove_chinese_fillers() {
        let filler = FillerWords::new();
        let text = "我想嗯安排一个会议";
        let cleaned = filler.remove_fillers(text, "zh");

        assert!(!cleaned.contains("嗯"));
    }

    #[test]
    fn test_is_filler_detection() {
        let filler = FillerWords::new();

        assert!(filler.is_filler("um", "en"));
        assert!(filler.is_filler("uh", "en"));
        assert!(filler.is_filler("嗯", "zh"));
    }
}
