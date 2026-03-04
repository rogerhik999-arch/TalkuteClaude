//! Text polishing service using Claude API

use crate::ai::client::ClaudeClient;
use crate::ai::prompts::AIPrompts;
use crate::error::Result;

/// Text polishing service
pub struct TextPolisher {
    client: ClaudeClient,
    prompts: AIPrompts,
}

impl TextPolisher {
    /// Create a new text polisher
    pub fn new() -> Result<Self> {
        let client = ClaudeClient::new()?;
        let prompts = AIPrompts::new();

        Ok(Self { client, prompts })
    }

    /// Polish text using the default prompt
    pub async fn polish(&self, text: &str) -> Result<String> {
        let system_prompt = self.prompts.default_polishing();
        let user_message = self.prompts.format(&system_prompt, text);

        self.client.complete(&system_prompt, &user_message).await
    }

    /// Polish text with context-aware tone
    pub async fn polish_with_context(&self, text: &str, context: &str) -> Result<String> {
        let system_prompt = self.prompts.for_context(context);
        let user_message = self.prompts.format(&system_prompt, text);

        self.client.complete(&system_prompt, &user_message).await
    }

    /// Translate text to target language
    pub async fn translate(&self, text: &str, target_language: &str) -> Result<String> {
        let system_prompt = self.prompts.translation(target_language);
        let user_message = self.prompts.format(&system_prompt, text);

        self.client.complete(&system_prompt, &user_message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polisher_creation() {
        // This test requires ANTHROPIC_API_KEY env var
        // Skip if not available
        if std::env::var("ANTHROPIC_API_KEY").is_err() {
            return;
        }

        let result = TextPolisher::new();
        assert!(result.is_ok());
    }
}
