use crate::WavReader::wavreader::*;
/*
pub fn auto_tune(s:Vec<i16>,note:i64,eps:i64) -> Vec<i16>
{
    let mut res:Vec<i16> = Vec::new();
    let l_note = (44100/note) as i64;
    let e = detection_hauteur(s.clone(),eps);
    let mut sort = Vec::new();
    for l in 2..=110
    {
        if e[l] > 0
        {
            sort.push((e[l],l));
        }
    }
    sort.sort_by_key(|k| k.0);
    sort.reverse();
    let mut l_freq = Vec::new();
    for i in sort
    {
        l_freq.push(i.1 * 8);
    }
    
    let mut temp = s.clone();
    let mut count = 1;
    for i in l_freq.clone()
    {
        println!("{}/{}",count,l_freq.clone().len());
        let mut res:Vec<i16> =Vec::new();
        let lon = temp.len();
        println!("l = {}",lon);
        let mut current_sample = 0;
        let list_freq = detect_freq(temp.clone(),eps,i as i64);//vec start
        count += 1;
        for freq in list_freq
        {
            let start = freq;
            let end = freq + i as u64;
            for samples in current_sample..start
            {
                res.push(temp[samples as usize]);
            }
            
            let mut start_end = Vec::new();
            for samples in start..=end
            {
                start_end.push(temp[samples as usize]);
            }


            let mut modify = Vec::new();
            if start_end.len() > (l_note as usize)
            {
                modify = sup_freq(start_end,l_note);
            }
            else
            {
                modify = add_freq(start_end,l_note);
            }
            
            for samples in modify
            {
                res.push(samples);
            }


            current_sample += (end - start) + 1;
        }
        for samples in (current_sample as usize)..lon
        {
            res.push(temp[samples as usize]);
        }
        temp = res.clone();
    }
    temp

}
*/

pub fn auto_tune(s:Vec<i16>,freq:i16,note:i16) -> Vec<i16>
{
    let l_note = 44100/(note as usize);
    let l_freq = 44100/(freq as usize);
    let n = ((s.clone().len() * l_note) / l_freq) as i64;
    let mut res = Vec::new();
    if s.clone().len() < (n as usize)
    {
        res = add_freq(s,n);
    }
    else
    {
        res = sup_freq(s,n);
    }
    res
}

pub fn add_numbers(start: i16, end: i16, size: usize) -> Vec<i16>
{
    let step = (end - start) as f64 / (size as f64 + 1.0);

    let mut result = Vec::with_capacity(size + 2);
    let mut current = start as f64 + step; 
    for _ in 1..(size + 1) 
    {
        result.push(current.round() as i16);
        current += step;
    }
    result
}

pub fn add_freq(s:Vec<i16>, l_freq:i64) -> Vec<i16>
{
    let l = s.len();
    let moy:f64 = (l_freq as f64)/(l as f64 - 1.0) - 1.0;
    let entier = moy as i16;
    let decimal = moy - (entier as f64);
    let mut retenue = decimal;
    let mut res = Vec::new();
    for i in 0..(l - 1)
    {
        let mut add = 0;
        if retenue >= 1.0 
        {
            add = 1;
            retenue -= 1.0;
        }

        res.push(s[i]);
        let temp = add_numbers(s[i],s[i+1], (entier + add) as usize);
        for j in temp
        {
            res.push(j);
        }
        retenue += decimal;
     }
    res.push(s[l - 1]);
    res
}

pub fn sup_freq(s:Vec<i16>, l_freq:i64) -> Vec<i16>
{
    let l = s.len();
    let moy:f64 = (l as f64)/(l_freq as f64);
    let entier = moy as i16;
    let decimal = moy - (entier as f64);
    let mut retenue = decimal;
    let mut j = 0 as usize;
    let mut res = Vec::new();
    while j < l
    {
        let mut add =0;
        if retenue >= 1.0
        {
            add = 1;
            retenue -= 1.0;
        }
        
        res.push(s[j]);
        j += (entier + add) as usize;
        
        retenue += decimal;
     }
    res
}
/*
pub fn init_l_bis (s:Vec<i16>, l_freq:i64 ,current_sample:i64) -> (i64,i64,i64)
{
    let mut e = 0;
    let mut h = 0;
    let mut res2 = 0;
    let ind = l_freq as usize;
    for i in current_sample..(current_sample  + 2*(l_freq))
    {
        e += (s[i as usize] as i64) * (s[i as usize] as i64);
    }
    for i in current_sample..(current_sample  + l_freq)
    {
        h += (s[i as usize] as i64) * (s[(i as usize - ind)as usize] as i64);
    }
    res2 = e - 2*h;
    (e,h,res2)
}


pub fn detection_hauteur(samples: Vec<i16>, eps: i64) -> Vec<i64>
{
    //new samples list / 8
    let l = samples.len();
    let mut s8: Vec<i16> = Vec::new();
    for i in 0..l
    {
        if i%8 == 0
        {
            s8.push(samples[i]);
        }
    }
    let l8 = s8.len();
    let mut find = false;
    let mut e: Vec<i64> = vec![0; 111];
    let mut res2 = vec![0; 111];
    let mut h: Vec<i64> = vec![0; 111];
    let mut res: Vec<i64> = vec![0; 111];
    

    let mut current_sample = 110;
    let end_sample = (l8 - 221) as i64; 
    for l in 2..=110
    {
        (e[l as usize],h[l as usize],res2[l as usize]) = init_l_bis(s8.clone(),l,current_sample);
    }

    //auto correlation

    //update
    while current_sample < end_sample
    {
        for i in 2..=110
        {
            if res2[i] * eps <= e[i] && res2[i] > 0
            {
                res[i] += 1;
            }
        }
           
        for j in 2..=110
        {
            //update
            let ind = j as usize;
            let ind2 = (current_sample) as usize;
            let ind3 = (current_sample + j + 1) as usize;

            e[ind] -= (s8[ind2] as i64) * (s8[ind2] as i64);
            e[ind] += (s8[ind3] as i64) * (s8[ind3] as i64);

            h[ind] -= (s8[ind2] as i64) * (s8[ind2 - ind] as i64);
            h[ind] += (s8[ind3] as i64) * (s8[ind3 - ind] as i64);
            
            res2[ind] = e[ind] - 2*h[ind];
        }
        
        current_sample += 1;
        
    
    }

    res
}

pub fn detect_freq(s: Vec<i16>, eps: i64, l_freq:i64) -> Vec<u64>
{
    let mut freq_list:Vec<u64> = Vec::new();
    //init
    let mut current_sample:i64 = l_freq;
    let end_sample = (s.len()) as i64 - 2*(l_freq) - 1;
    let mut e = 0;
    let mut h = 0;
    let mut res2 = 0;

    (e,h,res2) = init_l_bis(s.clone(),l_freq,current_sample);

    while current_sample < end_sample
    {
        //check current_sample == l_freq
        // ok ?
        //yes


        if res2 * eps <= e && res2 > 0
        {
            freq_list.push(current_sample as u64);
            current_sample += l_freq;
            if current_sample < end_sample
            {
                (e,h,res2) = init_l_bis(s.clone(),l_freq,current_sample);
            }
        }

        //no
        else
        {
            //update
            let ind = l_freq as usize;
            let ind2 = (current_sample) as usize;
            let ind3 = (current_sample + l_freq + 1) as usize;

            e -= (s[ind2] as i64) * (s[ind2] as i64);
            e += (s[ind3] as i64) * (s[ind3] as i64);

            h -= (s[ind2] as i64) * (s[ind2 - ind] as i64);
            h += (s[ind3] as i64) * (s[ind3 - ind] as i64);
            
            res2 = e - 2*h;

            current_sample += 1;
        }
    }
    freq_list
}
*/
