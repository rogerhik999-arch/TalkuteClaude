//! AI prompt templates

/// AI prompt templates for different contexts
pub struct AIPrompts {
    /// Default prompt for text polishing
    pub default_polish: String,
    /// Email tone prompt
    pub email: String,
    /// Chat tone prompt
    pub chat: String,
    /// Document tone prompt
    pub document: String,
    /// Code tone prompt
    pub code: String,
    /// Browser/General tone prompt
    pub browser: String,
}

impl AIPrompts {
    /// Create new AI prompt templates
    pub fn new() -> Self {
        Self {
            default_polish: Self::default_polish_prompt(),
            email: Self::email_prompt(),
            chat: Self::chat_prompt(),
            document: Self::document_prompt(),
            code: Self::code_prompt(),
            browser: Self::browser_prompt(),
        }
    }

    /// Default polishing prompt
    fn default_polish_prompt() -> String {
        "Convert this spoken text to polished written text. Remove filler words \
         (um, uh, like, you know), apply self-corrections, and format for \
         clear written communication while preserving the original meaning. \
         Present only the final polished text without any additional commentary.\
         \n\nSpoken text:\n\"{input}\""
            .to_string()
    }

    /// Formal email tone prompt
    fn email_prompt() -> String {
        "Convert this spoken text to polished written text suitable for email \
         communication. Use formal tone, proper grammar, and business-appropriate \
         language. Structure the text as a proper email message.\
         \n\nSpoken text:\n\"{input}\""
            .to_string()
    }

    /// Casual chat tone prompt
    fn chat_prompt() -> String {
        "Convert this spoken text to polished written text suitable for chat \
         messaging. Use a casual, conversational tone that matches the original \
         intent. Keep it natural and friendly.\
         \n\nSpoken text:\n\"{input}\""
            .to_string()
    }

    /// Professional document tone prompt
    fn document_prompt() -> String {
        "Convert this spoken text to polished written text suitable for \
         professional documentation. Use clear, structured language with \
         proper formatting. Make it suitable for inclusion in reports or \
         documentation.\
         \n\nSpoken text:\n\"{input}\""
            .to_string()
    }

    /// Technical/code tone prompt
    fn code_prompt() -> String {
        "Convert this spoken text to polished written text suitable for \
         technical documentation or code comments. Use precise, technical \
         language with correct terminology. Maintain technical accuracy.\
         \n\nSpoken text:\n\"{input}\""
            .to_string()
    }

    /// Browser/general tone prompt
    fn browser_prompt() -> String {
        "Convert this spoken text to polished written text suitable for \
         general web browsing context. Use clear, accessible language that \
         is easy to read and understand.\
         \n\nSpoken text:\n\"{input}\""
            .to_string()
    }

    /// Get prompt for a specific context
    pub fn for_context(&self, context: &str) -> String {
        match context.to_lowercase().as_str() {
            "email" | "outlook" | "gmail" => self.email.clone(),
            "chat" | "slack" | "discord" | "whatsapp" | "wechat" => self.chat.clone(),
            "document" | "notes" | "report" => self.document.clone(),
            "code" | "editor" | "ide" | "programming" => self.code.clone(),
            "browser" | "web" => self.browser.clone(),
            _ => self.default_polish.clone(),
        }
    }

    /// Format prompt with input
    pub fn format(&self, prompt: &str, input: &str) -> String {
        prompt.replace("{input}", input)
    }
}

impl Default for AIPrompts {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_prompt() {
        let prompts = AIPrompts::new();
        assert!(prompts.default_polish.contains("polished written text"));
    }

    #[test]
    fn test_context_specific_prompts() {
        let prompts = AIPrompts::new();

        assert!(prompts.for_context("email").contains("formal"));
        assert!(prompts.for_context("chat").contains("casual"));
        assert!(prompts.for_context("code").contains("technical"));
    }

    #[test]
    fn test_prompt_formatting() {
        let prompts = AIPrompts::new();
        let formatted = prompts.format(&prompts.default_polish, "Hello world");

        assert!(formatted.contains("Hello world"));
    }
}
