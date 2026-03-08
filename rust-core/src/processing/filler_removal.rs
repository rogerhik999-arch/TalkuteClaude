//! Filler word removal module with multi-language support

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
    korean: HashSet<&'static str>,
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
            korean: Self::korean_fillers(),
        }
    }

    /// English filler words
    fn english_fillers() -> HashSet<&'static str> {
        [
            "um", "uh", "like", "you know", "sort of", "kind of",
            "well", "so", "basically", "literally", "actually",
            "right", "okay", "yeah", "honestly", "definitely",
        ]
            .iter()
            .cloned()
            .collect()
    }

    /// Chinese filler words
    fn chinese_fillers() -> HashSet<&'static str> {
        [
            "嗯", "啊", "额", "那个", "这个", "就是", "然后",
            "就是说", "其实", "所以", "对吧", "是吧", "嘛",
        ]
            .iter()
            .cloned()
            .collect()
    }

    /// Japanese filler words
    fn japanese_fillers() -> HashSet<&'static str> {
        [
            "えっと", "あの", "その", "まあ", "なんか",
            "実は", "結構", "わりと", "ちょっと", "やっぱり",
        ]
            .iter()
            .cloned()
            .collect()
    }

    /// Spanish filler words
    fn spanish_fillers() -> HashSet<&'static str> {
        [
            "eh", "este", "pues", "o sea", "bueno", "vale",
            "tipo", "digamos", "así que", "¿sabes?", "¿verdad?",
        ]
            .iter()
            .cloned()
            .collect()
    }

    /// French filler words
    fn french_fillers() -> HashSet<&'static str> {
        [
            "euh", "ben", "du coup", "en fait", "quoi",
            "genre", "bah", "donc", "alors", "tu vois",
        ]
            .iter()
            .cloned()
            .collect()
    }

    /// German filler words
    fn german_fillers() -> HashSet<&'static str> {
        [
            "äh", "öhm", "also", "halt", "eben",
            "so", "na", "gewissermaßen", "quasi", "irgendwie",
        ]
            .iter()
            .cloned()
            .collect()
    }

    /// Korean filler words
    fn korean_fillers() -> HashSet<&'static str> {
        [
            "음", "그", "저", "뭐냐", "있잖아",
            "그니까", "근데", "솔직히", "진짜", "왜냐하면",
        ]
            .iter()
            .cloned()
            .collect()
    }

    /// Get fillers for a specific language
    pub fn get_fillers_for_language(&self, language: &str) -> Vec<String> {
        let fillers = match language {
            "zh" | "zh-CN" => &self.chinese,
            "ja" | "ja-JP" => &self.japanese,
            "es" | "es-ES" => &self.spanish,
            "fr" | "fr-FR" => &self.french,
            "de" | "de-DE" => &self.german,
            "ko" | "ko-KR" => &self.korean,
            _ => &self.english,
        };

        fillers.iter().map(|s| s.to_string()).collect()
    }

    /// Remove filler words from text
    pub fn remove_fillers(&self, text: &str, language: &str) -> String {
        let fillers = match language {
            "zh" | "zh-CN" => &self.chinese,
            "ja" | "ja-JP" => &self.japanese,
            "es" | "es-ES" => &self.spanish,
            "fr" | "fr-FR" => &self.french,
            "de" | "de-DE" => &self.german,
            "ko" | "ko-KR" => &self.korean,
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
            "ko" | "ko-KR" => &self.korean,
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

    #[test]
    fn test_get_fillers_for_language() {
        let filler = FillerWords::new();

        let en_fillers = filler.get_fillers_for_language("en");
        assert!(en_fillers.contains(&"um".to_string()));

        let zh_fillers = filler.get_fillers_for_language("zh");
        assert!(zh_fillers.contains(&"嗯".to_string()));
    }
}