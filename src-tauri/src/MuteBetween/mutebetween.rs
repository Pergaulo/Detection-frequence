pub fn mutebetween(samples: Vec<i16>, start: f32, end: f32) -> Vec<i16> {
    let mut s = samples.clone(); // clone les samples d'entrée
    let sample_rate = 44100; // normalement c'est le sample rate
    let start_index = (start * 2.0 * sample_rate as f32) as usize; // début mais en index 
    let end_index = (end * 2.0 * sample_rate as f32) as usize; // fin mais en index

    for i in start_index..=end_index { // met les freqs a 0 entre les deux timers
        s[i] = 0;
    }
    s
}