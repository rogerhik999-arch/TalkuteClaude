// AI module
pub mod client;
pub mod polisher;
pub mod prompts;
pub mod translator;

pub use prompts::AIPrompts;
pub use translator::{TranslationService, TranslationRequest, TranslationResult};
