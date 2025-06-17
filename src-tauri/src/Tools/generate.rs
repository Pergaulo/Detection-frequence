use std::{f64::consts::PI, fs::{read_dir, DirEntry, File}, io::{Read, Write}};

use rand::thread_rng;
use rand::Rng;

use std::collections::HashMap;

use crate::{WavReader::wavreader::WavReader, FFT::fft::nextpower};

#[derive(Debug, Clone, Copy)]
pub enum Note {
    Do = 0,
    Re = 4,
    Mi = 3,
    Fa = 1,
    Sol = 6,
    La = 2,
    Si = 5,
    Pause = 7,
}


pub fn generate_wave(time: u8, freq: f64) -> Vec<i16>
{

    let n:usize = 44100*(time*2) as usize;
    let mut signal = vec![0.0;n];
    
    for i in 0..n
    {
        signal[i] = i as f64/44100.0;
    }
    for i in 0..n
    {
        signal[i] = (2.0 * PI * freq *signal[i] as f64).sin();
    }

    //Normalize f64 -> i16
    for i in 0..n
    {
        signal[i] = (signal[i]*32767.0);
    }

    return signal.iter().map(|&x| x as i16).collect();
}

fn merge(samples1: &mut Vec<i16>, samples2: &Vec<i16>) {
    let mut i = 0;
    if samples1.len() > 0 {
        i = samples1.len()-100;
    }
    // for sample in samples2 {
    //     if i < samples1.len() {
    //         samples1[i] = *sample;
    //         i += 1;
    //     }
    //     else {
    //         samples1.push(*sample);
    //     }
    // }
    let mut samples3 = samples2.clone();
    nextpower(&mut samples3);
    let mut rd = thread_rng();
    let end:usize = rd.gen_range(20000..samples3.len());

    for j in 10000..end {
        if i < samples1.len() {
            samples1[i] = samples3[j];
            i += 1;
        }
        else {
            samples1.push(samples3[j]);
        }
    }
}

fn crossfade(samples1: &Vec<i16>, samples2: &Vec<i16>) -> Vec<i16>{
    let crossfade_duration = 0.5*44100.0;
    let mut res = vec![];
    

    for (index,sample) in samples1.iter().enumerate() {
        let fade_factor = if (index as f64) < crossfade_duration {
            index as f32 / crossfade_duration as f32
        } else if (index as f64) >= (samples1.len() as f64) - crossfade_duration {
            1.0 - (index as f32 - (samples1.len() as f64 - crossfade_duration) as f32) / crossfade_duration as f32
        } else {
            1.0
        };

        let sample2 = samples2[index];
        let sample_out = (*sample as f32 * fade_factor + sample2 as f32 * (1.0 - fade_factor)) as i16;
        res.push(sample_out);
    }
    let mut i = crossfade_duration as usize;
    while i < samples2.len() {
        res.push(samples2[i]);
        i += 1;
    }
    return res;
}

fn crossfade2(samples1: &Vec<i16>, samples2: &Vec<i16>) -> Vec<i16> {
    let crossfade_duration = 0.5*44100.0;
    let min_length = samples1.len().min(samples2.len());
    let mut res = vec![];

    for i in 0..min_length {
        let fade_in = (i as f64) / (crossfade_duration as f64);
        let fade_out = 1.0 - fade_in;

        // Calculer les échantillons après le crossfade
        let sample = (fade_out * samples1[i] as f64 + fade_in * samples2[i] as f64) as i16;

        // Écrire l'échantillon dans le fichier de sortie
        res.push(sample);
    }
    for i in min_length..samples1.len() {
        res.push(samples1[i]);
    }
    for i in min_length..samples2.len() {
        res.push(samples2[i]);
    }
    return res;
}

//Generate a music without following any rules
pub fn generate_music(duration:u8) -> Vec<i16> {
    let dir = read_dir("../src/sounds/notes");
    
    if dir.is_ok() {
        let notes = dir.unwrap();
        let mut path_notes = vec![];
        let mut rd = thread_rng();
        let mut build_music = vec![];

        for note in notes {
            if note.is_ok() {
                path_notes.push(note.unwrap());
            }
        }
        let mut m = [false;7];

        while build_music.len()/44100 < duration as usize {
            let index:usize = rd.gen_range(0..7);
            if !m[index] {
                m[index] = false;
                let mut wav = WavReader::new(path_notes[index].path().to_str().unwrap());
                let mut samples1:Vec<i16> = wav.get_samples();
                let mut i = 0;
                
                //build_music = crossfade2(&build_music, &samples1);
                
                merge(&mut build_music, &samples1);
            }
            else {
                m[index] = false;
            }

            // nextpower(&mut samples1);
            // for sample in &samples1 {
            //     build_music.push(*sample);
            // }
            // for sample in &samples {
            //     if i < samples.len()/2 {
            //         build_music.push(*sample);
            //     }
            //     i += 1;
            // }

        }
        return build_music;
    }




    vec![]

}

fn merge2(samples1: &mut Vec<i16>, samples2: &Vec<i16>) {
    let mut i = 0;
    if samples1.len() > 0 {
        i = samples1.len()-100;
    }

    let mut samples3 = samples2.clone();
    //nextpower(&mut samples3);
    let mut rd = thread_rng();
    let end:usize = rd.gen_range(20000..samples3.len());

    for j in 10000..samples3.len() {
        if i < samples1.len() {
            samples1[i] = samples3[j];
            i += 1;
        }
        else {
            samples1.push(samples3[j]);
        }
        //samples1.push(samples3[j]);
    }
}

pub fn build_from(melody:Vec<Note>) -> Vec<i16> {
    let dir = read_dir("../src/sounds/notes");
    
    if dir.is_ok() {
        let notes = dir.unwrap();
        let mut path_notes = vec![];
        let mut rd = thread_rng();
        let mut build_music = vec![];

        for note in notes {
            if note.is_ok() {
                path_notes.push(note.unwrap());
            }
        }
        let mut n2 = 0;
        for n in melody {
            match n {
                Note::Pause => {
                    for i in 0..22050{
                        build_music.push(0);
                    }
                },
                _ => {
                    let mut wav = WavReader::new(path_notes[n as usize].path().to_str().unwrap());
                    let mut samples1:Vec<i16> = wav.get_samples();
                    merge2(&mut build_music, &samples1);
                }
            }
            
            // if n2 == 0 {
            //     for sample in samples1 {
            //         build_music.push(sample);
            //     }
            // }
            // else {
            //     build_music = crossfade5(&build_music, &samples1,1000);
            // }
            // n2 += 1;
        }
        return build_music;
    }
    vec![]
}

//Build the transition matrix from a list of note for markov chain
pub fn transition_matrix(data:Vec<Note>) -> HashMap<u8, Vec<u8>> {
    let mut dictionary: HashMap<u8, Vec<u8>> = HashMap::new();
    
    for i in 0..(data.len()-1) {
        let current_note = data[i] as u8;
        let next_note = data[i+1] as u8;
        if !dictionary.contains_key(&current_note) {
            dictionary.insert(current_note, vec![next_note]);
        }
        else {
            dictionary.entry(current_note).or_insert_with(Vec::new).push(next_note);
        }
    }
    return dictionary;
}

//Generate a random melody with the transition matrix and with a time of duration
pub fn random_melody(matrix_transition:HashMap<u8, Vec<u8>>, duration:u16) -> Vec<i16> {
    let mut rd = thread_rng();
    let mut current_note = rd.gen_range(0..7);
    while !matrix_transition.contains_key(&current_note) {
        current_note = rd.gen_range(0..7);
    }
    let mut notes = vec![];

    for i in 0..duration {
        match current_note {
            0 => notes.push(Note::Do),
            1 => notes.push(Note::Fa),
            2 => notes.push(Note::La),
            3 => notes.push(Note::Mi),
            4 => notes.push(Note::Re),
            5 => notes.push(Note::Si),
            6 => notes.push(Note::Sol),
            7 => notes.push(Note::Pause),
            _ => ()
        }
        current_note = matrix_transition.get(&current_note).unwrap()[rd.gen_range(0..matrix_transition[&current_note].len())];

    }
    return build_from(notes);
}

//Train on the initial transition matrix and enhance it
pub fn train_data(matrix_transition: &mut HashMap<u8,Vec<u8>>, epoch:u32, duration:u8, save:bool) {
    let mut rd = thread_rng();

    for x in 0..epoch {
        let mut current_note = rd.gen_range(0..7);
        while !matrix_transition.contains_key(&current_note) {
            current_note = rd.gen_range(0..7);
        }
        let mut notes = vec![];

        for _i in 0..duration {
            match current_note {
                0 => notes.push(Note::Do),
                1 => notes.push(Note::Fa),
                2 => notes.push(Note::La),
                3 => notes.push(Note::Mi),
                4 => notes.push(Note::Re),
                5 => notes.push(Note::Si),
                6 => notes.push(Note::Sol),
                7 => notes.push(Note::Pause),
                _ => ()
            }
            current_note = matrix_transition.get(&current_note).unwrap()[rd.gen_range(0..matrix_transition[&current_note].len())];
        }

        for i in 0..(notes.len()-1) {
            let current_note = notes[i] as u8;
            let next_note = notes[i+1] as u8;
            if !matrix_transition.contains_key(&current_note) {
                matrix_transition.insert(current_note, vec![]);
            }
            else {
                matrix_transition.entry(current_note).or_insert_with(Vec::new).push(next_note);
            }
        }
        // if x % 100 == 0 {
        //     println!("Finished epoch {x}");
        // }
    }
    
    if save {
        let mut data_file = File::create("train_data.txt").unwrap();

        for (key, val) in matrix_transition {
            let _ = data_file.write(&*key.to_string().as_bytes());
            let _ = data_file.write(b":");

            for note in val {
                let _ = data_file.write(&*note.to_string().as_bytes());
                let _ = data_file.write(b"");
            }
            let _ = data_file.write(b"\n");
        }
    }
}

pub fn load_data(path:&str) -> HashMap<u8,Vec<u8>> {
    let mut fd = File::open(path).unwrap();
    let mut matrix_transition:HashMap<u8, Vec<u8>> = HashMap::new();

    let mut buf:[u8;1] = [0;1];
    let mut n = 1;
    let mut key = true;
    let mut notes:Vec<u8> = vec![];
    let mut note:u8 = 0;
    let mut key_val = 0;

    while n != 0 {

        n = fd.read(&mut buf).unwrap();

        if key && buf[0] as char != '\n' {
            if !notes.is_empty() {
                matrix_transition.insert(key_val, notes.clone());
                notes = vec![];
            }
            key_val = buf[0]-48;
            key = false;
        }
        else if buf[0] as char != ' ' && buf[0] as char != '\n' && buf[0] as char != ':' && !key {
            notes.push(buf[0]-48);
        }

        if buf[0] as char == '\n' {
            key = true;
        }
    }

    if !notes.is_empty() {
        matrix_transition.insert(key_val, notes.clone());
    }
    return matrix_transition;
}

fn crossfade3(audio1: &[i16], audio2: &[i16], fade_length: usize) -> Vec<i16> {

    let mut audio1 = audio1.to_vec();
    let mut audio2 = audio2.to_vec();

    adjust_audio_length(&mut audio1, &mut audio2);

    let mut result = Vec::new();
    let max_fade_length = audio2.len() / 2;
    let fade_length = fade_length.min(max_fade_length);

    // Appliquer le fondu sortant sur le premier audio
    for i in 0..fade_length {
        let volume = i as f64 / fade_length as f64;
        let faded_sample = (audio1[i] as f64) * (1.0 - volume) + (audio2[i] as f64) * volume;
        result.push(faded_sample);
    }

    // Concaténer les parties non fondues et fondues des deux audios
    for i in fade_length..audio1.len() - fade_length {
        result.push(audio1[i] as f64);
    }

    // Appliquer le fondu entrant sur le deuxième audio
    for i in (audio2.len() - fade_length)..audio2.len() {
        // dbg!("HERE");
        // println!("{} - ({} - {} + {}) / {}",fade_length,i,audio2.len(),fade_length,fade_length);
        let volume = (fade_length - (audio2.len() - i)) as f64 / fade_length as f64;
        let faded_sample = (audio1[i] as f64) * (1.0 - volume) + (audio2[i] as f64)* volume;
        result.push(faded_sample);
    }

    result.iter().map(|&x| x as i16).collect()
}

fn adjust_audio_length(audio1: &mut Vec<i16>, audio2: &mut Vec<i16>) {
    let max_len = audio1.len().max(audio2.len());

    audio1.resize(max_len, 0);
    audio2.resize(max_len, 0);
}

fn crossfade4(audio1: &[i16], audio2: &[i16], fade_length: usize) -> Vec<i16> {
    let mut audio1 = audio1.to_vec();
    let mut audio2 = audio2.to_vec();

    adjust_audio_length(&mut audio1, &mut audio2);

    let mut result = Vec::with_capacity(audio1.len());

    let max_fade_length = audio2.len() / 2;
    let fade_length = fade_length.min(max_fade_length);

    // Appliquer le fondu sortant sur le premier audio
    for i in 0..fade_length {
        let volume = i as f32 / fade_length as f32;
        let faded_sample = (audio1[i] as f32) * (1.0 - volume) + (audio2[i] as f32) * volume;
        result.push(faded_sample);
    }

    // Concaténer les parties non fondues du premier audio
    for i in fade_length..(audio1.len() - fade_length) {
        result.push(audio1[i] as f32);
    }

    // Ajouter le deuxième audio
    for i in 0..audio2.len() {
        result.push(audio2[i] as f32);
    }

    // Appliquer le fondu entrant sur le deuxième audio
    for i in (audio2.len() - fade_length)..audio2.len() {
        let volume = (fade_length - (audio2.len() - i)) as f32 / fade_length as f32;
        let faded_sample = (audio1[i] as f32) * (1.0 - volume) + (audio2[i] as f32) * volume;
        result.push(faded_sample);
    }

    result.iter().map(|&x| x as i16).collect()
}

fn crossfade5(audio1: &[i16], audio2: &[i16], fade_length: usize) -> Vec<i16> {
    let mut audio1 = audio1.to_vec();
    let mut audio2 = audio2.to_vec();

    adjust_audio_length(&mut audio1, &mut audio2);

    let mut result = Vec::new();

    let max_fade_length = audio2.len() / 2;
    let fade_length = fade_length.min(max_fade_length);
    dbg!("HERE");
    // Ajouter l'audio 1 (sans la partie qui sera fondu)
    for i in 0..(audio1.len() - fade_length) {
        result.push(audio1[i] as f32);
    }

    // Appliquer le fondu sortant sur la fin de l'audio 1
    for i in 0..fade_length {
        let volume = i as f32 / fade_length as f32;
        let faded_sample = (audio1[audio1.len() - fade_length + i] as f32) * (1.0 - volume);
        result.push(faded_sample);
    }

    // Appliquer le fondu entrant sur le début de l'audio 2
    for i in 0..fade_length {
        let volume = i as f32 / fade_length as f32;
        let faded_sample = (audio2[i] as f32) * volume;
        result.push(faded_sample);
    }

    // Ajouter le reste de l'audio 2
    for i in fade_length..audio2.len() {
        result.push(audio2[i] as f32);
    }

    result.iter().map(|&x| x as i16).collect()
}