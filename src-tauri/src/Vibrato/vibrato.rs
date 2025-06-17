use std::f32::consts::PI;

pub fn vibrato(samples: Vec<i16>, freq: f32) -> Vec<i16> {
    let sr = 44100; // toujours la même sample rate
    let sinuso = 2.0 * PI * freq; // établis la frequence de la fonction sin

    samples
        .into_iter()
        .enumerate()
        .map(|(i, sample)| {
            let t = i as f32 / sr as f32;
            let gain = (sinuso * t).sin() * 0.5 + 0.5; // applique la fonction sin au sample en fonction de sinuso (la freq ect t'as capté)
            (sample as f32 * gain) as i16
        })
        .collect()
} // pour le main : la freq c'est le nombre de repetitions par secondes : plus c'est grand plus c'est rapide