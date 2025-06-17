use std::f32::consts::PI;
use crate::WavReader::wavreader::*;

pub fn passebas(samples: Vec<i16>, samplerate: u64, cofreq: f32) -> Vec<i16> {
    let rc = 1.0 / (cofreq * 2.0 * std::f32::consts::PI);
    let dt = 1.0 / samplerate as f32;
    let alpha = dt / (rc + dt);
    let mut samples = samples.clone();

    for  _ in 0..8 {
        let mut last = samples[0] as f32;

        for sample in samples.iter_mut() {
            let current = *sample as f32;
            let filtered = last + alpha * (current - last);
            *sample = filtered as i16;
            last = filtered;
        }
    }

    samples
}

pub fn passehaut(samples: Vec<i16>, sample_rate: u32, cofreq: f32) -> Vec<i16> 
{
    let rc = 1.0 / (cofreq * 2.0 * std::f32::consts::PI);
    let dt = 1.0 / sample_rate as f32;
    let alpha = dt / (rc + dt);
    let mut new_samples = samples.clone();

    for _ in 0..8 
    {
        let mut last = samples[0] as f32;

        for sample in samples.iter() 
        {
            let mtn = *sample as f32;
            let filtre = (1.0 - alpha) * last + alpha * mtn;
            new_samples.push(filtre as i16);
            last = mtn;
        }
    }

    new_samples
}