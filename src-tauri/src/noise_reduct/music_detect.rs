use crate::WavReader::wavreader::*;
use std::fs;


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
        if i%16 == 0
        {
            s8.push(samples[i]);
        }
    }
    let l8 = s8.len();
    let mut find = false;
    let mut e: Vec<i64> = vec![0; 56];
    let mut res2 = vec![0; 56];
    let mut h: Vec<i64> = vec![0; 56];
    let mut res: Vec<i64> = vec![0; 56];
    

    let mut current_sample = 55;
    let end_sample = (l8 - 111) as i64; 
    for l in 2..=55
    {
        (e[l as usize],h[l as usize],res2[l as usize]) = init_l_bis(s8.clone(),l,current_sample);
    }
     //auto correlation

    //update
    while current_sample < end_sample
    {
        for i in 2..=55
        {
            if res2[i] * eps <= e[i] && res2[i] > 0
            {
                res[i] += 1;
            }
        }
           
        for j in 2..=55
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

pub fn find_music(sounds:Vec<Vec<i16>>, reff:Vec<i16>,eps:i64) -> usize
{
    let res_ref = detection_hauteur(reff,eps);
    let mut res_ref_tuple:Vec<(usize, usize)> = vec![];
    for i in 2..=55
    {
        res_ref_tuple.push((i,res_ref[i] as usize));
    }
    res_ref_tuple.sort_by_key(|t| t.1);
    res_ref_tuple.reverse();
    let mut res_ref_five = vec![];
    for i in 0..5
    {
        res_ref_five.push(res_ref_tuple[i].0);
    }


    let mut list_res = vec![];
    for sound in sounds
    {
        list_res.push(detection_hauteur(sound,eps));
    }
    let mut list_grades = Vec::new();

    for res in list_res
    {
        let mut res_tuple:Vec<(usize, usize)> = vec![];
        for i in 2..=55
        {
            res_tuple.push((i,res[i] as usize));
        }
        res_tuple.sort_by_key(|t| t.1);
        res_tuple.reverse();
        let mut res_five = vec![];
        for i in 0..5
        {
            res_five.push(res_tuple[i].0);
        }
    
        let mut grade = 0;
        for freq in 0..5
        {
            for i in 0..5
            {
                if res_five[freq] == res_ref_five[i]
                {
                    if freq != i
                    {
                        grade += (10 - (2 * ((i as i16 - (freq as i16)).abs() as usize))) * (5 - freq) ;
                    }
                    else 
                    {
                        grade += (5 - i) * 15;
                    }
                }
            }
        }
        list_grades.push(grade);
    }
    let mut maxi = 0;
    for i in 1..list_grades.clone().len()
    {
        if list_grades[i] > list_grades[maxi]
        {
            maxi = i;
        }
    }
    //dbg!(list_grades);
    maxi
}


pub fn list_wav(sound_ref:Vec<i16>,eps:i64) -> String
{
    let folder_path = "../../../src/sounds/Shazam";
    let mut wav_readers = Vec::new();
    let mut sounds = Vec::new();
    let fichiers = match fs::read_dir(folder_path) 
    {
        Ok(entries) => 
        {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "wav"))
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect::<Vec<String>>()
        }
        Err(e) => {
            return "err open wav".to_string();
        }
    };
    
    let temp = fichiers.clone();
    for fichier in &temp 
    {
        wav_readers.push(fichier);
        //println!("{}",fichier);
    }
    
    for samp in &wav_readers
    {
        let mut temp = WavReader::new((folder_path.to_owned() + "/" + samp).as_str());
        sounds.push(temp.get_samples())
    }
    let res = find_music(sounds,sound_ref,eps);
    (*fichiers[res]).to_string()
}


