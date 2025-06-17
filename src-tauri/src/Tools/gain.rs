pub fn gain(samples: Vec<i16>, db: f64) -> Vec<i16>
{
    let mut samples2 = samples.clone();
    let g = 10_f64.powf(db/20.0);
    return samples2.iter_mut().map(|x| ((*x as f64)*(g)) as i16).collect();
}