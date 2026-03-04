# AI Prompt Templates Contract

**Feature**: 001-ai-voice-input | **Date**: 2026-03-04 | **Spec**: [spec.md](../spec.md)

## Overview

This document defines the AI prompt templates, versioning strategy, and contract for text polishing using Large Language Models (LLMs). All prompts are designed to be context-aware, deterministic, and optimized for low latency.

## Prompt Architecture

```text
User Input (Raw Speech)
         ↓
Context Detection (Application + Tone)
         ↓
Prompt Template Selection
         ↓
Prompt Assembly (System + User + Context)
         ↓
LLM API Call (Claude/GPT)
         ↓
Polished Output
```

## Core Principles

1. **Context-Aware**: Prompts adapt based on detected application category and user preferences
2. **Deterministic**: Use low temperature (0.3) for consistent, predictable outputs
3. **Concise**: Minimize token usage to reduce latency and cost
4. **Structured**: Use XML tags for clear input/output boundaries
5. **Versioned**: All prompts versioned for A/B testing and rollback capability

## System Prompts

### Base System Prompt (v1.0)

```xml
<system>
You are a professional text polishing assistant. Your task is to transform raw speech transcriptions into polished, well-formatted text while preserving the speaker's original meaning and intent.

Rules:
1. Remove filler words (um, uh, like, you know, etc.)
2. Detect and apply self-corrections (e.g., "I went to the... I mean I visited the store" → "I visited the store")
3. Add appropriate punctuation and capitalization
4. Fix grammatical errors
5. Maintain the speaker's tone and style
6. Do NOT add new information or change the meaning
7. Do NOT translate unless explicitly requested
8. Output ONLY the polished text, no explanations

Language: {language_code}
Tone: {tone_preference}
Context: {application_category}
</system>
```

### Context-Specific System Prompts

#### Email Context (v1.0)

```xml
<system>
You are polishing text for an email. Apply these additional rules:
- Use formal, professional language
- Structure content with proper paragraphs
- Add appropriate greetings/closings if missing
- Use complete sentences
- Avoid contractions unless the tone is explicitly casual
</system>
```

#### Chat Context (v1.0)

```xml
<system>
You are polishing text for a chat message. Apply these additional rules:
- Keep the casual, conversational tone
- Preserve contractions and informal language
- Keep messages concise (1-3 sentences typical)
- Maintain natural flow
- Emojis are acceptable if present in original
</system>
```

#### Document Context (v1.0)

```xml
<system>
You are polishing text for a document. Apply these additional rules:
- Use formal, clear language
- Structure content with proper paragraphs
- Ensure logical flow between sentences
- Use complete, well-formed sentences
- Maintain consistency in terminology
</system>
```

#### Code Context (v1.0)

```xml
<system>
You are polishing text for code comments or documentation. Apply these additional rules:
- Use technical, precise language
- Keep explanations concise and clear
- Preserve technical terms exactly as spoken
- Use imperative mood for instructions
- Avoid unnecessary verbosity
</system>
```

#### Browser Context (v1.0)

```xml
<system>
You are polishing text for web content (search queries, forms, social media). Apply these additional rules:
- Keep text concise and direct
- Preserve search intent for queries
- Use natural, conversational language
- Maintain appropriate formality for the platform
</system>
```

## User Prompt Templates

### Standard Polishing Prompt (v1.0)

```xml
<user>
<raw_transcription>
{raw_text}
</raw_transcription>

<instructions>
Polish the above transcription according to the system rules. Output only the polished text.
</instructions>
</user>
```

### With Personal Dictionary (v1.1)

```xml
<user>
<raw_transcription>
{raw_text}
</raw_transcription>

<personal_dictionary>
{dictionary_entries}
</personal_dictionary>

<instructions>
Polish the above transcription according to the system rules. Apply personal dictionary replacements where applicable. Output only the polished text.
</instructions>
</user>
```

### With Custom Context Instructions (v1.2)

```xml
<user>
<raw_transcription>
{raw_text}
</raw_transcription>

<context_instructions>
{custom_instructions}
</context_instructions>

<instructions>
Polish the above transcription according to the system rules and the custom context instructions. Output only the polished text.
</instructions>
</user>
```

## Prompt Assembly Logic

```rust
pub struct PromptBuilder {
    language: String,
    tone: Option<TonePreference>,
    app_category: AppCategory,
    custom_instructions: Option<String>,
    dictionary_entries: Vec<DictionaryEntry>,
}

impl PromptBuilder {
    pub fn build_system_prompt(&self) -> String {
        let base = self.get_base_system_prompt();
        let context = self.get_context_system_prompt();
        format!("{}\n\n{}", base, context)
    }

    pub fn build_user_prompt(&self, raw_text: &str) -> String {
        let mut prompt = format!(
            "<user>\n<raw_transcription>\n{}\n</raw_transcription>\n",
            raw_text
        );

        if !self.dictionary_entries.is_empty() {
            prompt.push_str("<personal_dictionary>\n");
            for entry in &self.dictionary_entries {
                prompt.push_str(&format!("- \"{}\" → \"{}\"\n", entry.phrase, entry.replacement));
            }
            prompt.push_str("</personal_dictionary>\n\n");
        }

        if let Some(instructions) = &self.custom_instructions {
            prompt.push_str(&format!(
                "<context_instructions>\n{}\n</context_instructions>\n\n",
                instructions
            ));
        }

        prompt.push_str("<instructions>\nPolish the above transcription according to the system rules");
        if !self.dictionary_entries.is_empty() {
            prompt.push_str(". Apply personal dictionary replacements where applicable");
        }
        if self.custom_instructions.is_some() {
            prompt.push_str(" and the custom context instructions");
        }
        prompt.push_str(". Output only the polished text.\n</instructions>\n</user>");

        prompt
    }
}
```

## API Configuration

### Claude API (Primary)

```rust
pub struct ClaudeConfig {
    pub model: String,              // "claude-3-5-sonnet-20241022"
    pub max_tokens: u32,            // 1024
    pub temperature: f32,           // 0.3
    pub top_p: f32,                 // 0.9
    pub timeout_seconds: u32,       // 10
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 1024,
            temperature: 0.3,
            top_p: 0.9,
            timeout_seconds: 10,
        }
    }
}
```

### OpenAI API (Fallback)

```rust
pub struct OpenAiConfig {
    pub model: String,              // "gpt-4o-mini"
    pub max_tokens: u32,            // 1024
    pub temperature: f32,           // 0.3
    pub top_p: f32,                 // 0.9
    pub timeout_seconds: u32,       // 10
}

impl Default for OpenAiConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4o-mini".to_string(),
            max_tokens: 1024,
            temperature: 0.3,
            top_p: 0.9,
            timeout_seconds: 10,
        }
    }
}
```

## Response Parsing

### Expected Response Format

```text
This is the polished text output from the LLM.
```

### Error Handling

```rust
pub enum PolishingError {
    EmptyResponse,
    InvalidFormat,
    TooLong,
    ApiError(String),
}

pub fn parse_polished_text(response: &str) -> Result<String, PolishingError> {
    let trimmed = response.trim();

    if trimmed.is_empty() {
        return Err(PolishingError::EmptyResponse);
    }

    if trimmed.len() > 10000 {
        return Err(PolishingError::TooLong);
    }

    // Remove any XML tags if present
    let cleaned = remove_xml_tags(trimmed);

    Ok(cleaned.to_string())
}
```

## Prompt Versioning

### Version Format

`{template_name}-v{major}.{minor}`

Examples:
- `base-system-v1.0`
- `email-context-v1.0`
- `standard-polishing-v1.1`

### Version Storage

```rust
pub struct PromptVersion {
    pub template_name: String,
    pub version: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

// Stored in SQLite for A/B testing
CREATE TABLE prompt_versions (
    id TEXT PRIMARY KEY,
    template_name TEXT NOT NULL,
    version TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    UNIQUE (template_name, version)
);
```

### A/B Testing Strategy

```rust
pub struct PromptExperiment {
    pub experiment_id: String,
    pub template_name: String,
    pub version_a: String,
    pub version_b: String,
    pub traffic_split: f32,  // 0.0-1.0 (e.g., 0.5 = 50/50 split)
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

pub fn select_prompt_version(
    template_name: &str,
    device_id: &str,
) -> String {
    // Check for active experiment
    if let Some(experiment) = get_active_experiment(template_name) {
        // Deterministic assignment based on device_id hash
        let hash = hash_device_id(device_id);
        if hash < experiment.traffic_split {
            return experiment.version_a;
        } else {
            return experiment.version_b;
        }
    }

    // Return default active version
    get_active_version(template_name)
}
```

## Performance Optimization

### Prompt Caching

```rust
pub struct PromptCache {
    cache: HashMap<String, String>,
}

impl PromptCache {
    pub fn get_or_build(&mut self, key: &str, builder: impl FnOnce() -> String) -> String {
        self.cache.entry(key.to_string())
            .or_insert_with(builder)
            .clone()
    }
}
```

### Token Estimation

```rust
pub fn estimate_tokens(text: &str) -> u32 {
    // Rough estimation: 1 token ≈ 4 characters
    (text.len() / 4) as u32
}

pub fn validate_prompt_size(system: &str, user: &str) -> Result<(), PromptError> {
    let total_tokens = estimate_tokens(system) + estimate_tokens(user);

    if total_tokens > 4000 {
        return Err(PromptError::TooLarge(total_tokens));
    }

    Ok(())
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder_basic() {
        let builder = PromptBuilder {
            language: "en-US".to_string(),
            tone: Some(TonePreference::Formal),
            app_category: AppCategory::Email,
            custom_instructions: None,
            dictionary_entries: vec![],
        };

        let system = builder.build_system_prompt();
        assert!(system.contains("email"));
        assert!(system.contains("formal"));
    }

    #[test]
    fn test_prompt_builder_with_dictionary() {
        let builder = PromptBuilder {
            language: "en-US".to_string(),
            tone: None,
            app_category: AppCategory::Chat,
            custom_instructions: None,
            dictionary_entries: vec![
                DictionaryEntry {
                    phrase: "btw".to_string(),
                    replacement: "by the way".to_string(),
                },
            ],
        };

        let user = builder.build_user_prompt("btw I'm running late");
        assert!(user.contains("personal_dictionary"));
        assert!(user.contains("btw"));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_end_to_end_polishing() {
    let raw = "um so I was thinking uh we should like meet tomorrow";
    let polished = polish_text(raw, AppCategory::Email, None).await.unwrap();

    assert!(!polished.contains("um"));
    assert!(!polished.contains("uh"));
    assert!(!polished.contains("like"));
}
```

## Monitoring & Analytics

### Prompt Performance Metrics

```rust
pub struct PromptMetrics {
    pub template_name: String,
    pub version: String,
    pub total_calls: u64,
    pub avg_latency_ms: f32,
    pub avg_input_tokens: f32,
    pub avg_output_tokens: f32,
    pub error_rate: f32,
}
```

### Logging

```rust
pub fn log_prompt_execution(
    template_name: &str,
    version: &str,
    input_tokens: u32,
    output_tokens: u32,
    latency_ms: u64,
    success: bool,
) {
    // Log to local analytics database
    // Used for A/B testing analysis and performance monitoring
}
```

## Security Considerations

1. **No PII in Prompts**: Never include device IDs, user names, or other PII in prompts
2. **Input Sanitization**: Escape XML special characters in user input
3. **Output Validation**: Verify LLM output doesn't contain injection attempts
4. **Rate Limiting**: Enforce per-device rate limits to prevent abuse
5. **API Key Security**: Store API keys in secure platform-specific storage

## Future Enhancements

1. **Multi-Language Support**: Expand to 10+ languages with language-specific prompts
2. **Custom Tone Training**: Allow users to train custom tone preferences
3. **Prompt Optimization**: Use reinforcement learning to optimize prompts based on user feedback
4. **Streaming Responses**: Support streaming for real-time polishing preview
5. **Local LLM Fallback**: Support on-device LLM for offline operation
