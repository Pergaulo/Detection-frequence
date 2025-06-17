pub fn repeat_section(samples: Vec<i16>, start_time: f64, end_time: f64, repeat_count: usize, sample_rate: usize) -> Vec<i16> {
     // Check if end_time > start_time
     if start_time >= end_time {
        panic!("Le temps de début doit être inférieur au temps de fin");
    }

    // Convert start and end times to sample indices
    let start_index = (start_time  * sample_rate as f64) as usize;
    let end_index = (end_time   * sample_rate as f64) as usize;

    //Check that samples are included in the time interval
    if start_index >= samples.len() || end_index >= samples.len() {
        panic!("Les temps spécifiés ne sont pas dans la plage de l'audio");
    }
   
    // Extract the section to be repeated
    let section = &samples[start_index..=end_index]; //Include last element

    // Create a new vector to hold the repeated section
    let mut repeated_samples = Vec::with_capacity(section.len() * repeat_count);

    // Repeat the section the specified number of times
    for _ in 0..repeat_count {
        repeated_samples.extend_from_slice(section);
    }

    repeated_samples
}

pub fn repeat_segment(samples: Vec<i16>, start_time: f32, end_time: f32, repeat_count: usize, sample_rate: u64) -> Vec<i16> {
     // Check if end_time > start_time
     if start_time >= end_time {
        panic!("Le temps de début doit être inférieur au temps de fin");
    }

    // Convert start and end times to sample indices
    let start_index = (start_time * sample_rate as f32) as usize;
    let end_index = (end_time * sample_rate as f32) as usize;

    //Check that samples are included in the time interval
    if start_index >= samples.len() || end_index >= samples.len() {
        panic!("Les temps spécifiés ne sont pas dans la plage de l'audio");
    }

    // Extract the segment to be repeated
    let segment: Vec<i16> = samples[start_index..end_index].to_vec();

    // Repeat the segment n times
    let repeated_segment: Vec<i16> = segment.iter()
                                            .cloned()
                                            .cycle()
                                            .take(segment.len() * repeat_count)
                                            .collect();

    // Create a new vector to hold the result
    let mut result_samples: Vec<i16> = Vec::with_capacity(samples.len() + repeated_segment.len());

    // Insert the samples before the start index
    result_samples.extend_from_slice(&samples[..start_index]);

    // Insert the repeated segment
    result_samples.extend_from_slice(&repeated_segment);

    // Insert the samples after the end index
    result_samples.extend_from_slice(&samples[end_index..]);

    result_samples
}