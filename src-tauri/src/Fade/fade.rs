pub fn fade_start(samples: Vec<i16>, fade_duration: usize, sample_rate: usize) -> Vec<i16> {
    // Clone the original samples vector to keep the original data intact
    let mut faded_samples = samples.clone();
    let fade_time = fade_duration * sample_rate;
    // Apply the fade-in effect
    for i in 0..fade_time.min(samples.len()) {
        // Calculate the fade multiplier (linearly increases from 0.0 to 1.0)
        let fade_multiplier = i as f32 / fade_time as f32;

        // Apply the fade multiplier to the sample
        faded_samples[i] = (samples[i] as f32 * fade_multiplier) as i16;
    }

    faded_samples
}

pub fn fade_end(samples: Vec<i16>, fade_duration: usize, sample_rate: usize) -> Vec<i16> {
    // Clone the original samples vector to keep the original data intact
    let mut faded_samples = samples.clone();
    let len = samples.len();
    let fade_time = fade_duration * sample_rate;
    // Apply the fade-out effect
    for i in 0..fade_time.min(len) {
        // Calculate the fade multiplier (linearly decreases from 1.0 to 0.0)
        let fade_multiplier = (fade_time - i) as f32 / fade_time as f32;

        // Apply the fade multiplier to the sample
        faded_samples[len - fade_time + i] = (samples[len - fade_time + i] as f32 * fade_multiplier) as i16;
    }

    faded_samples
}

pub fn fade_start_end(samples: Vec<i16>, fade_in_duration: usize, fade_out_duration: usize, sample_rate: usize) -> Vec<i16> {
    let mut faded_samples = samples.clone();
    let len = samples.len();

    let fade_time_start = fade_in_duration * sample_rate;

    // Apply the fade-in effect
    for i in 0..fade_time_start.min(len) {
        let fade_multiplier = i as f32 / fade_time_start as f32;
        faded_samples[i] = (samples[i] as f32 * fade_multiplier) as i16;
    }

    let fade_time_end = fade_out_duration * sample_rate;
    // Apply the fade-out effect
    for i in 0..fade_time_end.min(len) {
        let fade_multiplier = (fade_time_end - i) as f32 / fade_time_end as f32;
        faded_samples[len - fade_time_end + i] = (samples[len - fade_time_end + i] as f32 * fade_multiplier) as i16;
    }

    faded_samples
}