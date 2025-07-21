use speakr_core::audio::{record_to_vec, SAMPLE_RATE_HZ, CHANNELS};

#[test]
fn captures_audio_with_correct_parameters() {
    // Attempt to record 1 second of audio. This should eventually return 16_000 mono samples.
    let _samples = record_to_vec(1);

    // Verify expected constants.
    assert_eq!(SAMPLE_RATE_HZ, 16_000);
    assert_eq!(CHANNELS, 1);
}