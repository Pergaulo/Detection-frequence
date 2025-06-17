pub fn leftnright(samples: Vec<i16>) -> Vec<i16> {
    let mut samp = samples.clone();
    let mut change_or = false; // false = gauche, true = droite
    let mut samples_count = 0;
    let period = 122400; // 44100 samples par sec, donc periode de 3 sec
    let mid_period = period;

    for sample in samp.iter_mut() 
    {
        // Si jamais la periode est passé, on change d'oreille
        if samples_count >= period 
        {
            change_or = !change_or;
            samples_count = 0;
        }

        // L'effet de transition wave (comme un effet de vague), baisse le son et l'augmente selon le moment de la periode active
        let wave_effect = if samples_count < mid_period 
        {
            (samples_count as f64 / mid_period as f64) * 2.0 - 1.0
        } 
        else 
        {
            ((period - samples_count) as f64 / mid_period as f64) * 2.0 - 1.0
        };

        if change_or 
        {
            // Droite
            if samples_count % 2 != 0 
            {
                *sample = (*sample as f64 * wave_effect) as i16;
            }
        } 
        else 
        {
            // Gauche
            if samples_count % 2 == 0 
            {
                *sample = (*sample as f64 * wave_effect) as i16;
            }
        }
        samples_count += 1;
    }
    samp
}

// Version sans transition
/*pub fn leftnright(mut samples: Vec<i16>) -> Vec<i16> {
    let mut change_or = false; 
    let mut samples_count = 0;

    for sample in samples.iter_mut() 
    {
        if samples_count >= 44100 
        {
            change_or = !change_or;
            samples_count = 0;
        }

        if change_or
        {
            if samples_count % 2 != 0 
            {
                *sample = 0;
            }
        } 
        else 
        {
            if samples_count % 2 == 0 
            {
                *sample = 0;
            }
        }
        samples_count += 1;
    }
    samples
}*/
