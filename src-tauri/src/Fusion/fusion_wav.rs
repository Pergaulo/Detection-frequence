pub fn mix_samples(samples1: Vec<i16>, samples2: Vec<i16>) -> Vec<i16> 
{
    let mut samp_cop1 = samples1.clone();
    let mut samp_cop2 = samples2.clone();
    let len = samp_cop1.len().max(samp_cop2.len());
    let mut res = Vec::with_capacity(len);

    // En gros je rajoute à la liste de samples la plus grande la liste la plus petite

    for i in 0..len 
    {
        let sample1 = samp_cop1.get(i).cloned().unwrap_or(0);
        let sample2 = samp_cop2.get(i).cloned().unwrap_or(0);

        let combi_sample = match sample1.checked_add(sample2) 
        {
            Some(sum) => sum,
            None => {
                if sample1 > 0 
                {
                    i16::MAX
                } 
                else 
                {
                    i16::MIN
                }
            }
        };

        res.push(combi_sample);
    }
    res
}