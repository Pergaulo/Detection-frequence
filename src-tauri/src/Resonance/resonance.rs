pub fn reso_echo(samples: Vec<i16>) -> Vec<i16> 
{
    let mut cop = samples.clone();
    let delai_samples = 11025; // Delai de 0.25 secondes (44100 samples par seconde)
    let num_repetitions = 2; // Nombres de repetitions (ça marche pas punaise)
    let init_attenuation = 0.5; // Attenuation de base
    let attenuation_repet = 0.5; // Attenuation à chaque repetition

    let mut res = Vec::with_capacity(cop.len() + (delai_samples * num_repetitions));
    res.extend(cop.iter().cloned());

    // L'effet d'echo
    let mut attenuation = init_attenuation;
    for rep in 1..=num_repetitions
    {
        for i in 0..samples.len() 
        {
            let echo_index = i + delai_samples * rep;
            if echo_index < samples.len() 
            {
                let sample_debase = cop[i] as f32;
                let echo_s = (cop[echo_index] as f32) + (sample_debase * attenuation);
                res[echo_index] = echo_s as i16;
            }
        }
        attenuation *= attenuation_repet;
    }

    res
}