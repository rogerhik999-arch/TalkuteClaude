//! Tests for audio capture module

use crate::speech::audio_capture::AudioCapture;

#[test]
fn test_audio_capture_creation() {
    let capture = AudioCapture::new("test-device");

    assert_eq!(capture.device_id, "test-device");
    assert!(!capture.is_recording);
}

#[test]
fn test_default_format() {
    let format = crate::speech::audio_capture::AudioFormat::default();

    assert_eq!(format.sample_rate, 16000);
    assert_eq!(format.channels, 1);
}

#[test]
fn test_high_fidelity_format() {
    let format = crate::speech::audio_capture::AudioFormat::high_fidelity();

    assert_eq!(format.sample_rate, 44100);
    assert_eq!(format.channels, 2);
}
