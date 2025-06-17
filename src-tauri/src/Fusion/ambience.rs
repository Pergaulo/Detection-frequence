use super::fusion_wav;
use fusion_wav::*;
use crate::WavReader::wavreader::*;

pub fn ambience(samp: Vec<i16>, num: i16) -> Vec<i16>
{
    let mut samples = samp.clone();

    match num
    {
        1 => {
            let mut wav1 = WavReader::new("../../../src/sounds/Ambiances/Sample_04.wav");
            let mut samples1 = wav1.get_samples().clone();
            mix_samples(samples1, samples)
        },
        2 => {
            let mut wav2 = WavReader::new("../../../src/sounds/Ambiances/cave.wav");
            let mut samples2 = wav2.get_samples().clone();
            mix_samples(samples2, samples)
        },
        3 => {
            let mut wav3 = WavReader::new("../../../src/sounds/Ambiances/rain.wav");
            let mut samples3 = wav3.get_samples().clone();
            mix_samples(samples3, samples)
        },
        _ => samples,
    }
}