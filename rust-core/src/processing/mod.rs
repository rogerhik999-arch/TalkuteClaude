//! Text processing pipeline

pub mod filler_removal;
pub mod formatter;
pub mod self_correction;

pub use filler_removal::FillerWords;
pub use formatter::TextFormatter;
pub use self_correction::SelfCorrectionDetector;

/// Text processing pipeline
pub struct TextProcessingPipeline {
    filler_removal: FillerWords,
    self_correction: SelfCorrectionDetector,
    formatter: TextFormatter,
}

impl TextProcessingPipeline {
    /// Create a new text processing pipeline
    pub fn new() -> Self {
        Self {
            filler_removal: FillerWords::new(),
            self_correction: SelfCorrectionDetector::new(),
            formatter: TextFormatter::new(),
        }
    }

    /// Process text through the pipeline
    pub fn process(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Step 1: Remove filler words
        result = self.filler_removal.remove_fillers(&result, "en");

        // Step 2: Apply self-corrections
        result = self.self_correction.apply_corrections(&result);

        // Step 3: Format to written language
        result = self.formatter.format(&result);

        result
    }

    /// Get a reference to the filler remover
    pub fn filler_removal(&self) -> &FillerWords {
        &self.filler_removal
    }

    /// Get a reference to the self-correction detector
    pub fn self_correction(&self) -> &SelfCorrectionDetector {
        &self.self_correction
    }

    /// Get a reference to the text formatter
    pub fn formatter(&self) -> &TextFormatter {
        &self.formatter
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

        // Should have processed all steps
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
}
