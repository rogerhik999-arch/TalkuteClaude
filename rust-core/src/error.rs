//! Error types for the Talkute core library

use thiserror::Error;

/// Result type alias for standard error handling
pub type Result<T> = std::result::Result<T, Error>;

/// Core error types for the Talkute AI Voice Input Assistant
#[derive(Error, Debug)]
pub enum Error {
    /// Audio capture related errors
    #[error("Audio error: {0}")]
    Audio(#[from] AudioError),

    /// Speech recognition API errors
    #[error("Speech API error: {0}")]
    SpeechApi(#[from] SpeechApiError),

    /// AI service errors
    #[error("AI service error: {0}")]
   AiService(#[from] AiServiceError),

    /// Storage/database errors
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    /// Context detection errors
    #[error("Context error: {0}")]
    Context(#[from] ContextError),

    /// Quota exceeded errors
    #[error("Quota exceeded")]
    QuotaExceeded,

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Unknown errors
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Audio capture error types
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Microphone access denied")]
    AccessDenied,
    #[error("Invalid audio format")]
    InvalidFormat,
    #[error("Audio device not found")]
    DeviceNotFound,
    #[error("Audio buffer overflow")]
    BufferOverflow,
    #[error("Audio engine initialization failed")]
    InitializationFailed(String),
}

/// Speech API error types
#[derive(Error, Debug)]
pub enum SpeechApiError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API throttled - rate limit exceeded")]
    RateLimited,
    #[error("API authentication failed")]
    AuthenticationFailed,
    #[error("API unavailable: {0}")]
    Unavailable(String),
    #[error("Invalid response from API")]
    InvalidResponse,
}

/// AI service error types
#[derive(Error, Debug)]
pub enum AiServiceError {
    #[error("Prompt too long")]
    PromptTooLong,
    #[error("Token limit exceeded")]
    TokenLimitExceeded,
    #[error("Invalid prompt template")]
    InvalidPrompt,
    #[error("API timeout after waiting")]
    Timeout,
    #[error("AI service unavailable")]
    ServiceUnavailable,
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API authentication failed")]
    AuthenticationFailed,
    #[error("Invalid response from API")]
    InvalidResponse,
}

/// Type alias for AI errors (used by client modules)
pub type AIError = AiServiceError;

/// Storage error types
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database connection failed")]
    ConnectionFailed,
    #[error("Query execution failed: {0}")]
    QueryFailed(String),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Record already exists")]
    RecordAlreadyExists,
    #[error("Migration failed: {0}")]
    MigrationFailed(String),
    #[error("Data validation failed: {0}")]
    ValidationError(String),
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),
}

/// Context detection error types
#[derive(Error, Debug)]
pub enum ContextError {
    #[error("Platform not supported")]
    PlatformNotSupported,
    #[error("Permission denied for context detection")]
    PermissionDenied,
    #[error("Failed to detect active window")]
    DetectionFailed(String),
    #[error("Invalid application category")]
    InvalidCategory,
}
