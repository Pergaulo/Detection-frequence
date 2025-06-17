use crate::WavReader::wavreader::*;
/*
#[derive(Debug,Clone)]
pub struct WavReader
{
    pub channels: u8,
    pub sample_rate: u64,
    pub bits_per_sample: u8,
    data_size: u32,
    bytes_read: usize,
    //file: std::fs::File,
    header: Vec<[u8;4]>,
    samples: Vec<i16>,
    samples_bytes: Vec<u8>,
    //filename: str
}*/

pub fn threshold(wav: &mut WavReader, threshold: u16, high: bool, low:bool) 
{
    let t = threshold as i16;
    for sample in &mut wav.samples 
    {
        if (*sample > t && high) || (*sample < -t && low) 
        {
            *sample = 0 as i16;
        }
    }
}

pub fn lissage_mobile(samples: Vec<i16>, m: usize) -> Vec<i16>
{

    if m%2 != 1
    {
        panic!("m must be odd")
    }
    else
    {
        let dist= (m  - 1)/2;
        let s = samples;
        let len = s.len();
        let mut res = vec![0;len];
        //dbg!(len);
        for i in 1..(len - 1) 
        {
            let ii = i as i64;
            let mut average = 0;
            //dbg!(i);
            //dbg!(dist);
            if ii - (dist as i64) < 0 
            {
                let n = (2 * i + 1);
                for y in 0..n
                {
                    (s[y as usize] / n as i16);
                }
                //average /= n; 
            }
            else if i + dist > len - 1
            {
                let d = len - 1 - i;
                let n = (2 * d + 1);
                for y in (i-d)..len
                {
                    average +=  (s[y as usize] / n as i16);
                }
                //average /= n; 
            }

           else
            {
                for y in (i - dist)..=(i + dist)
                {
                    average += (s[y as usize] / m as i16);
                }
                //average /= m as i16;
            } 
            res[i] = average;
        }
        res[0] = s[0];
        res[len - 1] = s[len - 1];
        res
    }
    

}


pub fn egaliseur(s:Vec<i16>,eps:i64,db:f64,freq_min:i64,freq_max:i64) -> Vec<i16>
{
    let mut change = Vec::new();
    let l_freq_min:i64 = 44100/freq_min +1 ;
    let l_freq_max:i64 = 44100/freq_max -1;
    //dbg!(l_freq_max,l_freq_min);
    //dbg!(l_freq_max,l_freq_min);
    for i in 0..s.len()
    {
        change.push(0);
    }
    for i in l_freq_max..=l_freq_min
    {
        //dbg!(i);
        change = change_amp_freq(s.clone(), eps,i,0,db,change);//db5 eps 4
    }
    apply_gain(s.clone(),change,db)
}
pub fn apply_gain(s:Vec<i16>,change:Vec<i16>,db:f64) -> Vec<i16>
{
    let mut res = Vec::new();
    let g = 10_f64.powf(db/20.0);
    for i in 0..s.len()
    {
        if change[i] == 1
        {
            res.push(((s[i as usize] as f64 * g)as i16))
        }
        else
        {
            res.push(s[i]);
        }
    }
    res
}
pub fn init_l (s:Vec<i16>, l_freq:i64, dist:i64,current_sample:i64) -> (Vec<i64>,Vec<i64>,Vec<i64>)
{
    let mut e = Vec::new();
    let mut h = Vec::new();
    let mut res2 = Vec::new();
    for i in 0..= l_freq
    {
        e.push(0);
        h.push(0);
        res2.push(0);
    }
    let ind = l_freq as usize;
    for i in current_sample..(current_sample  + 2*(l_freq))
    {
        e[ind] += (s[i as usize] as i64) * (s[i as usize] as i64);
    }
    for i in current_sample..(current_sample  + l_freq)
    {
        h[ind] += (s[i as usize] as i64) * (s[(i as usize - ind)as usize] as i64);
    }
    res2[ind] = e[ind] - 2*h[ind];
    (e,h,res2)
}
pub fn change_amp_freq(s: Vec<i16>, eps: i64, l_freq:i64,dist: i64,db:f64,change:Vec<i16>) -> Vec<i16>
{
    let mut change = change;
    //init
    let mut current_sample:i64 = l_freq;
    let end_sample = (s.len()) as i64 - 2*(l_freq) - 1;
    let mut e = Vec::new();
    let mut h = Vec::new();
    let mut res2 = Vec::new();
    let g = 10_f64.powf(db/20.0);
  
    (e,h,res2) = init_l(s.clone(),l_freq,dist,current_sample);

    while current_sample < end_sample
    {
        //check current_sample == l_freq
        // ok ?
        //yes
       

        if res2[l_freq as usize] * eps <= e[l_freq as usize] && res2[l_freq as usize] > 0
        {
            //dbg!(44100/l_freq,current_sample);
            if change[current_sample as usize] != 1
            {
                //dbg!(current_sample,44100/l_freq);
            }
            for i in current_sample..(current_sample + l_freq)
            {
                //dbg!(s[i as usize]);
                change[i as usize] = 1;
                //dbg!((s[i as usize] as f64 * g)as i16);
            }
            current_sample += l_freq;
            if current_sample < end_sample
            {
                (e,h,res2) = init_l(s.clone(),l_freq, dist,current_sample);
            }   
        }
        //no
        else
        {
            //update
            let ind = l_freq as usize;
            let ind2 = (current_sample) as usize;
            let ind3 = (current_sample + l_freq + 1) as usize;
            
            e[ind] -= (s[ind2] as i64) * (s[ind2] as i64);
            e[ind] += (s[ind3] as i64) * (s[ind3] as i64);
    
            h[ind] -= (s[ind2] as i64) * (s[ind2 - ind] as i64);
            h[ind] += (s[ind3] as i64) * (s[ind3 - ind] as i64);

            res2[ind] = e[ind] - 2*h[ind];   
 
            current_sample += 1;
        }
    }
    change
}



/*
fn main() {
    let v = [5,10,15,20,-2,32,45,89,65,4].to_vec();
    let mut w: WavReader = WavReader.new("/home/robin/epita/auda/Echo/bruit_440hz.wav");
    let a = lissage_mobile(& mut w, 51); 
    w.set_samples(a);
    w.write_wav("moy51");
    //threshold(&mut w, 3,true,false);
    //let a = w.samples;
    //dbg!(a);
}*/