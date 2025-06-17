pub fn distorsion(samples: Vec<i16>, disto: f32) -> Vec<i16> {
    samples.into_iter().map(|sample| {
        let sf32 = sample as f32 / i16::MAX as f32;
        let disto_s = (sf32 * disto).tanh(); // j'ai mis a 80.0 pour avoir un résultat pas mal (a tester ~+20)
        (disto_s * i16::MAX as f32) as i16
    }).collect()
}

pub fn reduce_volume(samples: Vec<i16>, gain: f32) -> Vec<i16> {
    samples.into_iter().map(|sample| (sample as f32 * gain) as i16).collect() // Le son est cher fort donc j'ai fait ca pour le test avec 0.05 en valeur
}