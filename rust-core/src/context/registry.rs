//! Shared application registry for consistent categorization
//!
//! This module provides a centralized registry for mapping application names
//! to categories, ensuring consistent behavior across all platforms.

use std::collections::HashMap;

/// Application category types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppCategory {
    Email,
    Chat,
    Document,
    Code,
    Browser,
    Other,
}

impl AppCategory {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AppCategory::Email => "email",
            AppCategory::Chat => "chat",
            AppCategory::Document => "document",
            AppCategory::Code => "code",
            AppCategory::Browser => "browser",
            AppCategory::Other => "other",
        }
    }
}

impl std::fmt::Display for AppCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Application registry for mapping application names to categories
pub struct ApplicationRegistry {
    /// Map of application name patterns to categories
    patterns: HashMap<String, AppCategory>,
}

impl ApplicationRegistry {
    /// Create a new application registry with default mappings
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Email applications
        for app in &["gmail", "outlook", "mail", "thunderbird", "protonmail", "yahoo mail", "apple mail"] {
            patterns.insert(app.to_string(), AppCategory::Email);
        }

        // Chat applications
        for app in &["slack", "discord", "telegram", "whatsapp", "wechat", "messenger", "teams", "zoom", "skype", "signal"] {
            patterns.insert(app.to_string(), AppCategory::Chat);
        }

        // Document applications
        for app in &["word", "docs", "notion", "typora", "pages", "writer", "onenote", "evernote", "obsidian"] {
            patterns.insert(app.to_string(), AppCategory::Document);
        }

        // Code editors
        for app in &["vscode", "visual studio code", "idea", "intellij", "vim", "neovim", "emacs", "sublime", "atom", "eclipse", "xcode", "android studio", "pycharm", "webstorm", "clion", "rustrover"] {
            patterns.insert(app.to_string(), AppCategory::Code);
        }

        // Browsers
        for app in &["chrome", "firefox", "safari", "edge", "opera", "brave", "vivaldi", "chromium"] {
            patterns.insert(app.to_string(), AppCategory::Browser);
        }

        Self { patterns }
    }

    /// Categorize an application by name
    pub fn categorize(&self, name: &str) -> AppCategory {
        let name_lower = name.to_lowercase();

        // Check for exact match first
        if let Some(category) = self.patterns.get(&name_lower) {
            return category.clone();
        }

        // Check for partial matches (application name contains a known pattern)
        for (pattern, category) in &self.patterns {
            if name_lower.contains(pattern) {
                return category.clone();
            }
        }

        AppCategory::Other
    }

    /// Get the category string for an application
    pub fn get_category(&self, name: &str) -> String {
        self.categorize(name).as_str().to_string()
    }

    /// Check if an application is in a specific category
    pub fn is_category(&self, name: &str, category: AppCategory) -> bool {
        self.categorize(name) == category
    }

    /// Get all known applications for a category
    pub fn get_applications_for_category(&self, category: &AppCategory) -> Vec<String> {
        self.patterns
            .iter()
            .filter(|(_, c)| *c == category)
            .map(|(name, _)| name.clone())
            .collect()
    }
}

impl Default for ApplicationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ApplicationRegistry::new();
        assert!(!registry.patterns.is_empty());
    }

    #[test]
    fn test_categorize_email() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.categorize("Gmail"), AppCategory::Email);
        assert_eq!(registry.categorize("Microsoft Outlook"), AppCategory::Email);
        assert_eq!(registry.categorize("Apple Mail"), AppCategory::Email);
    }

    #[test]
    fn test_categorize_chat() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.categorize("Slack"), AppCategory::Chat);
        assert_eq!(registry.categorize("Discord"), AppCategory::Chat);
        assert_eq!(registry.categorize("WhatsApp"), AppCategory::Chat);
    }

    #[test]
    fn test_categorize_code() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.categorize("VSCode"), AppCategory::Code);
        assert_eq!(registry.categorize("IntelliJ IDEA"), AppCategory::Code);
        assert_eq!(registry.categorize("Vim"), AppCategory::Code);
    }

    #[test]
    fn test_categorize_document() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.categorize("Microsoft Word"), AppCategory::Document);
        assert_eq!(registry.categorize("Google Docs"), AppCategory::Document);
        assert_eq!(registry.categorize("Notion"), AppCategory::Document);
    }

    #[test]
    fn test_categorize_browser() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.categorize("Google Chrome"), AppCategory::Browser);
        assert_eq!(registry.categorize("Firefox"), AppCategory::Browser);
        assert_eq!(registry.categorize("Safari"), AppCategory::Browser);
    }

    #[test]
    fn test_categorize_unknown() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.categorize("RandomApp"), AppCategory::Other);
        assert_eq!(registry.categorize("Unknown Application"), AppCategory::Other);
    }

    #[test]
    fn test_case_insensitive() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.categorize("GMAIL"), AppCategory::Email);
        assert_eq!(registry.categorize("slack"), AppCategory::Chat);
        assert_eq!(registry.categorize("VSCODE"), AppCategory::Code);
    }

    #[test]
    fn test_partial_match() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.categorize("Microsoft Outlook 2021"), AppCategory::Email);
        assert_eq!(registry.categorize("Visual Studio Code - project"), AppCategory::Code);
        assert_eq!(registry.categorize("Google Chrome - webpage"), AppCategory::Browser);
    }

    #[test]
    fn test_get_category_string() {
        let registry = ApplicationRegistry::new();
        assert_eq!(registry.get_category("Gmail"), "email");
        assert_eq!(registry.get_category("Slack"), "chat");
        assert_eq!(registry.get_category("VSCode"), "code");
    }
}