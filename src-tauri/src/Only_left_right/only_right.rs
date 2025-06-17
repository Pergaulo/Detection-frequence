pub fn right_wav(samples: Vec<i16>) -> Vec<i16> {
    let mut samp_cop = samples.clone();
    let mut sample_count = 0;

    for value in samp_cop.iter_mut() 
    {
        if sample_count % 2 == 0 
        {
            *value = 0;
        }
        sample_count += 1;
    }

    return samp_cop;
}