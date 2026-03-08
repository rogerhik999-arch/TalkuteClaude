//! Audio capture module for voice input

use crate::error::{Result, AudioError};

/// Audio capture session
pub struct AudioCapture {
    device_id: String,
    is_recording: bool,
    audio_format: AudioFormat,
    noise_cancellation_enabled: bool,
}

/// Audio format configuration
#[derive(Debug, Clone)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub encoding: AudioEncoding,
}

/// Audio encoding types
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum AudioEncoding {
    Pcm,
    /// Opus encoding for better compression
    Opus,
    /// AAC encoding for mobile
    Aac,
}

/// Noise cancellation settings
#[derive(Debug, Clone)]
pub struct NoiseCancellationSettings {
    /// Enable noise cancellation
    pub enabled: bool,
    /// Noise suppression level (0.0 - 1.0)
    pub suppression_level: f32,
    /// Enable automatic gain control
    pub auto_gain_control: bool,
    /// Enable echo cancellation
    pub echo_cancellation: bool,
}

impl Default for NoiseCancellationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            suppression_level: 0.8,
            auto_gain_control: true,
            echo_cancellation: true,
        }
    }
}

impl AudioCapture {
    /// Create a new audio capture session
    pub fn new(device_id: &str) -> Self {
        Self {
            device_id: device_id.to_string(),
            is_recording: false,
            audio_format: AudioFormat::default(),
            noise_cancellation_enabled: true,
        }
    }

    /// Create audio capture with custom noise cancellation settings
    pub fn with_noise_cancellation(device_id: &str, enabled: bool) -> Self {
        Self {
            device_id: device_id.to_string(),
            is_recording: false,
            audio_format: AudioFormat::default(),
            noise_cancellation_enabled: enabled,
        }
    }

    /// Start audio capture
    pub async fn start(&mut self) -> Result<()> {
        if self.is_recording {
            return Err(AudioError::AccessDenied.into());
        }

        // Platform-specific initialization
        self.init_platform()?;

        // Apply noise cancellation if enabled
        if self.noise_cancellation_enabled {
            self.enable_noise_cancellation();
        }

        self.is_recording = true;
        Ok(())
    }

    /// Stop audio capture
    pub async fn stop(&mut self) -> Result<Vec<u8>> {
        if !self.is_recording {
            return Err(AudioError::DeviceNotFound.into());
        }

        self.is_recording = false;
        self.shutdown_platform()?;

        // Return captured audio data
        Ok(self.get_audio_data())
    }

    /// Check if capture is active
    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    /// Get the device ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Get the audio format being used
    pub fn get_format(&self) -> &AudioFormat {
        &self.audio_format
    }

    /// Check if noise cancellation is enabled
    pub fn is_noise_cancellation_enabled(&self) -> bool {
        self.noise_cancellation_enabled
    }

    /// Enable or disable noise cancellation
    pub fn set_noise_cancellation(&mut self, enabled: bool) {
        self.noise_cancellation_enabled = enabled;

        // If currently recording, apply the change immediately
        if self.is_recording {
            if enabled {
                self.enable_noise_cancellation();
            } else {
                self.disable_noise_cancellation();
            }
        }
    }

    /// Get current noise cancellation settings
    pub fn get_noise_cancellation_settings(&self) -> NoiseCancellationSettings {
        let mut settings = NoiseCancellationSettings::default();
        settings.enabled = self.noise_cancellation_enabled;
        settings
    }

    /// Update noise cancellation settings
    pub fn update_noise_cancellation(&mut self, settings: NoiseCancellationSettings) {
        self.noise_cancellation_enabled = settings.enabled;
        // Additional settings would be applied to the platform-specific audio pipeline
    }

    /// Initialize platform-specific audio capture
    fn init_platform(&self) -> Result<()> {
        // Platform-specific implementation
        // - Windows: WASAPI capture
        // - macOS: AVFoundation
        // - Linux: PulseAudio/PipeWire
        // - iOS: AVAudioRecorder
        // - Android: AudioRecord
        Ok(())
    }

    /// Shutdown platform-specific audio capture
    fn shutdown_platform(&self) -> Result<()> {
        // Platform-specific cleanup
        Ok(())
    }

    /// Get the captured audio data
    fn get_audio_data(&self) -> Vec<u8> {
        // Return captured audio buffer
        Vec::new()
    }

    /// Get the current audio level (0.0 - 1.0)
    pub async fn get_audio_level(&self) -> Result<f64> {
        if !self.is_recording {
            return Ok(0.0);
        }

        // Platform-specific audio level detection
        Ok(0.0)
    }

    /// Enable noise cancellation on the audio stream
    fn enable_noise_cancellation(&self) {
        // Platform-specific noise cancellation:
        // - Windows: Use Windows Audio Session API (WASAPI) effects
        // - macOS: Use AVAudioEngine with noise reduction node
        // - Linux: Use PulseAudio module-echo-cancel or PipeWire filters
        // - iOS: Use AVAudioSession voice processing
        // - Android: Use AcousticEchoCanceler and NoiseSuppressor
    }

    /// Disable noise cancellation on the audio stream
    fn disable_noise_cancellation(&self) {
        // Platform-specific cleanup of noise cancellation
    }
}

impl AudioFormat {
    /// Create a default audio format for speech recognition
    pub fn default() -> Self {
        Self {
            sample_rate: 16000, // 16kHz for speech recognition
            channels: 1,        // Mono
            bits_per_sample: 16,
            encoding: AudioEncoding::Pcm,
        }
    }

    /// Create a high-fidelity audio format
    pub fn high_fidelity() -> Self {
        Self {
            sample_rate: 44100, // 44.1kHz
            channels: 2,        // Stereo
            bits_per_sample: 16,
            encoding: AudioEncoding::Pcm,
        }
    }
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_capture_creation() {
        let capture = AudioCapture::new("test-device");

        assert_eq!(capture.device_id, "test-device");
        assert!(!capture.is_recording);
        assert!(capture.noise_cancellation_enabled);
    }

    #[test]
    fn test_audio_capture_without_noise_cancellation() {
        let capture = AudioCapture::with_noise_cancellation("test-device", false);

        assert_eq!(capture.device_id, "test-device");
        assert!(!capture.is_noise_cancellation_enabled());
    }

    #[test]
    fn test_noise_cancellation_toggle() {
        let mut capture = AudioCapture::new("test-device");

        assert!(capture.is_noise_cancellation_enabled());

        capture.set_noise_cancellation(false);
        assert!(!capture.is_noise_cancellation_enabled());

        capture.set_noise_cancellation(true);
        assert!(capture.is_noise_cancellation_enabled());
    }

    #[test]
    fn test_audio_format_defaults() {
        let format = AudioFormat::default();

        assert_eq!(format.sample_rate, 16000);
        assert_eq!(format.channels, 1);
        assert_eq!(format.bits_per_sample, 16);
        assert_eq!(format.encoding, AudioEncoding::Pcm);
    }

    #[test]
    fn test_high_fidelity_format() {
        let format = AudioFormat::high_fidelity();

        assert_eq!(format.sample_rate, 44100);
        assert_eq!(format.channels, 2);
    }

    #[test]
    fn test_noise_cancellation_settings_default() {
        let settings = NoiseCancellationSettings::default();

        assert!(settings.enabled);
        assert!(settings.auto_gain_control);
        assert!(settings.echo_cancellation);
        assert!(settings.suppression_level > 0.0 && settings.suppression_level <= 1.0);
    }
}
