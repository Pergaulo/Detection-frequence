use num_complex::Complex;

use crate::FFT::fft::*;

pub fn low_pass_rs(samples: Vec<i16>, freq: f64) -> Vec<i16> {
    let mut fft_result = fft(samples, 1);

    for i in 0..fft_result.len() {
        if ((i as f64/fft_result.len() as f64)*44100 as f64) >freq {
            fft_result[i] = Complex::new(0.0, 0.0);
        }
    }

    return ifft(fft_result);
}