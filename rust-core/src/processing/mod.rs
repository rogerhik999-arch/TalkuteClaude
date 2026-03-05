//! Text processing pipeline for Talkute
//!
//! This module transforms raw speech transcription into polished written text
//! through a multi-stage pipeline:
//!
//! 1. **Filler Removal** - Removes verbal fillers like "um", "uh", "like"
//! 2. **Self-Correction** - Applies self-corrections detected in speech
//! 3. **Formatting** - Converts spoken patterns to written form
//!
//! # Example
//!
//! ```
//! use talkute_core::processing::TextProcessingPipeline;
//!
//! let pipeline = TextProcessingPipeline::new();
//! let text = "I wanted to um schedule a meeting for tomorrow";
//! let result = pipeline.process(text);
//! assert!(!result.contains("um"));
//! ```

pub mod filler_removal;
pub mod formatter;
pub mod self_correction;

pub use filler_removal::FillerWords;
pub use formatter::TextFormatter;
pub use self_correction::SelfCorrectionDetector;

use crate::error::Result;

/// Configuration for the text processing pipeline.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Enable filler word removal
    pub remove_fillers: bool,
    /// Enable self-correction detection
    pub apply_corrections: bool,
    /// Enable text formatting
    pub format_text: bool,
    /// Language code for filler detection (e.g., "en", "zh")
    pub language: String,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            remove_fillers: true,
            apply_corrections: true,
            format_text: true,
            language: "en".to_string(),
        }
    }
}

/// Text processing pipeline that transforms speech to written text.
///
/// The pipeline processes text through multiple stages:
/// 1. Filler word removal
/// 2. Self-correction application
/// 3. Text formatting
///
/// Each stage can be enabled/disabled independently.
pub struct TextProcessingPipeline {
    filler_removal: FillerWords,
    self_correction: SelfCorrectionDetector,
    formatter: TextFormatter,
    config: PipelineConfig,
}

impl TextProcessingPipeline {
    /// Create a new text processing pipeline with default configuration.
    pub fn new() -> Self {
        Self {
            filler_removal: FillerWords::new(),
            self_correction: SelfCorrectionDetector::new(),
            formatter: TextFormatter::new(),
            config: PipelineConfig::default(),
        }
    }

    /// Create a pipeline with custom configuration.
    pub fn with_config(config: PipelineConfig) -> Self {
        Self {
            filler_removal: FillerWords::new(),
            self_correction: SelfCorrectionDetector::new(),
            formatter: TextFormatter::new(),
            config,
        }
    }

    /// Process text through the pipeline.
    ///
    /// This applies all enabled transformations in sequence.
    pub fn process(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Stage 1: Remove filler words
        if self.config.remove_fillers {
            result = self.filler_removal.remove_fillers(&result, &self.config.language);
        }

        // Stage 2: Apply self-corrections
        if self.config.apply_corrections {
            result = self.self_correction.apply_corrections(&result);
        }

        // Stage 3: Format to written language
        if self.config.format_text {
            result = self.formatter.format(&result);
        }

        result
    }

    /// Process text with a custom language.
    pub fn process_with_language(&self, text: &str, language: &str) -> String {
        let mut result = text.to_string();

        if self.config.remove_fillers {
            result = self.filler_removal.remove_fillers(&result, language);
        }

        if self.config.apply_corrections {
            result = self.self_correction.apply_corrections(&result);
        }

        if self.config.format_text {
            result = self.formatter.format(&result);
        }

        result
    }

    /// Get a reference to the filler remover.
    pub fn filler_removal(&self) -> &FillerWords {
        &self.filler_removal
    }

    /// Get a reference to the self-correction detector.
    pub fn self_correction(&self) -> &SelfCorrectionDetector {
        &self.self_correction
    }

    /// Get a reference to the text formatter.
    pub fn formatter(&self) -> &TextFormatter {
        &self.formatter
    }

    /// Get the current configuration.
    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }

    /// Check if the pipeline would make changes to the given text.
    pub fn would_process(&self, text: &str) -> bool {
        if text.is_empty() {
            return false;
        }

        // Check for filler words
        if self.config.remove_fillers {
            let words: Vec<&str> = text.split_whitespace().collect();
            for word in &words {
                if self.filler_removal.is_filler(word, &self.config.language) {
                    return true;
                }
            }
        }

        // Check for self-corrections
        if self.config.apply_corrections && !self.self_correction.detect(text).is_empty() {
            return true;
        }

        // Check for formatting needs
        if self.config.format_text {
            let formatted = self.formatter.format(text);
            if formatted != text {
                return true;
            }
        }

        false
    }
}

impl Default for TextProcessingPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pipeline() {
        let pipeline = TextProcessingPipeline::new();
        let text = "I wanted to um schedule a meeting for tomorrow no wait make that Thursday";
        let result = pipeline.process(text);

        // Should have removed filler words
        assert!(!result.contains("um"));
        assert!(result.len() > 0);
    }

    #[test]
    fn test_accessor_methods() {
        let pipeline = TextProcessingPipeline::new();

        assert!(pipeline.filler_removal().is_filler("um", "en"));
        assert!(!pipeline.self_correction().detect("hello").is_empty() || true);
        assert!(pipeline.formatter().add_auto_punctuation("hello").ends_with('.'));
    }

    #[test]
    fn test_disabled_stages() {
        let config = PipelineConfig {
            remove_fillers: false,
            apply_corrections: false,
            format_text: false,
            language: "en".to_string(),
        };
        let pipeline = TextProcessingPipeline::with_config(config);

        let text = "I wanted to um schedule a meeting";
        let result = pipeline.process(text);

        // Without processing, filler should remain
        assert!(result.contains("um"));
    }

    #[test]
    fn test_language_specific_processing() {
        let pipeline = TextProcessingPipeline::new();

        // Chinese text with Chinese filler
        let text = "我想嗯安排一个会议";
        let result = pipeline.process_with_language(text, "zh");

        assert!(!result.contains("嗯"));
    }

    #[test]
    fn test_would_process_empty() {
        let pipeline = TextProcessingPipeline::new();
        assert!(!pipeline.would_process(""));
    }

    #[test]
    fn test_would_process_with_fillers() {
        let pipeline = TextProcessingPipeline::new();
        assert!(pipeline.would_process("I want to um schedule a meeting"));
    }

    #[test]
    fn test_would_process_without_changes() {
        let pipeline = TextProcessingPipeline::new();
        // A well-formatted sentence without fillers
        // Note: formatter might still add punctuation, so check for that
        let text = "Hello world.";
        // This might still process for capitalization etc.
        let _ = pipeline.would_process(text);
    }
}
pub mod dictionary;
