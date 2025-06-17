// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use echo::Only_left_right::only_left;
use echo::Only_left_right::only_right;
use echo::Tools::inverse::inverse_sound;
use echo::WavReader::wavreader::*;
use echo::Tools::gain::*;
use echo::Tools::speed::*;
use echo::Tools::generate::*;
use echo::Tools::generate::Note::*;
use echo::Tools::filter::*;
use echo::Record::sound_record::*;
use echo::noise_reduct::threshold::*;
use echo::ConvertToMp3::mp3::*;
use echo::Only_left_right::leftNright::*;
use echo::Only_left_right::only_left::*;
use echo::Only_left_right::only_right::*;
use echo::noise_reduct::pass::*;
use echo::Tools::generate_melody::*;

use echo::Distorsion::distorsion::*;
use echo::Fade::fade::*;
use echo::Fusion::ambience::*;
use echo::Fusion::fusion_wav::*;
use echo::MuteBetween::mutebetween::*;
use echo::Repeat::repeat::*;
use echo::Resonance::resonance::*;
use echo::Vibrato::vibrato::*;
use echo::noise_reduct::auto_tune::*;
use echo::noise_reduct::music_detect::*;


//use echo::noise_reduct::pass::*;
use echo::FFT::fft::*;
use echo::ConvertToMp3::convert_wav_mp3::*;
use nannou::wgpu::bytes;
use rustfft::num_traits::sign;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::channel;

use num_complex::{Complex, ComplexFloat};
use rustfft::FftPlanner;
use rustfft::FftDirection;

use std::time::Instant;

use midly::Smf;
// use midi_reader_writer::{
//     ConvertTicksToMicroseconds, ConvertMicroSecondsToTicks,
//     midly::{exports::Smf, merge_tracks, TrackSeparator},
// };

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load(path: &str)  -> (Vec<i16>, u8, u64, u8, u16, Vec<[u8;4]>,usize,String){
    println!("{}",path);
    let start = Instant::now();
    println!("Loading audio ");

    let exe_path = &std::env::current_exe().unwrap();
    let dir_path = exe_path.parent().unwrap().parent().unwrap().parent().unwrap().as_os_str().to_str().unwrap().to_string();


    //let mut wav = WavReader::new(&["../src/sounds/",path].concat());
    let mut wav = WavReader::new(path);

    let samples = wav.get_samples();
    let sample_rate = wav.sample_rate;
    let channels = wav.channels;
    let bits_per_sample = wav.bits_per_sample;
    let header = wav.header;
    let bytes = wav.bytes_read;
    let time = ((samples.len()/channels as usize)/sample_rate as usize) as u16;
    
    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return (samples, channels, sample_rate, bits_per_sample, time, header,bytes,dir_path);
}

#[tauri::command]
fn speedUp(header: Vec<[u8;4]>, samplesRate:u64, n:f64) -> Vec<[u8;4]>
{
    println!("Speed ");
    let start = Instant::now();

    let s = speed2(header, samplesRate, n);
    
    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return s;
}

#[tauri::command]
fn gainUp(samples: Vec<i16>, db: f64) -> Vec<i16> {
    println!("Gain ");
    let start = Instant::now();

    let g = gain(samples, db);
    
    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());
    return g;
}

#[tauri::command]
fn save(name: &str, samples: Vec<i16>, header:Vec<[u8;4]>){
    println!("Save ");
    let mut bytes = 0;
    let start = Instant::now();

    //create the new file
    let mut fd: File = File::create(name).unwrap();

    //Write the header
    for chunk in &header
    {
        bytes += 4;
        fd.write(chunk);
    }

    //Extract form samples the bytes
    let mut vec:Vec<u8> = vec![];
    for elt in samples
    {
        bytes += 2;
        vec.push((elt & 0b11111111) as u8);
        vec.push((elt >> 8) as u8);
    }
    
    //write all the samples as bytes
    fd.write(&(vec));

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());
}

#[tauri::command]
fn apply_change(name: &str, samples: Vec<i16>, header:Vec<[u8;4]>){
    println!("Apply change ");
    let start = Instant::now();
    //create the new file
    let mut fd: File = File::create(name).unwrap();

    //Write the header
    for chunk in &header
    {
        fd.write(chunk);
    }

    //Extract form samples the bytes
    let mut vec:Vec<u8> = vec![];
    for elt in samples
    {
        vec.push((elt & 0b11111111) as u8);
        vec.push((elt >> 8) as u8);
    }
    
    //write all the samples as bytes
    fd.write(&(vec));

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());
}

#[tauri::command]
fn FFT(samples: Vec<i16>, samplesRate: u64, channels:u8) -> Vec<f64> {
    println!("FFT ");
    let start = Instant::now();

    /*let n = samples.len();
    let mut completed_samples = samples;
    nextpower(&mut completed_samples);*/

    let fft_result = fft(samples,channels);

    //let slice_fft = fft_result[0..n].to_vec();

    let res = normalize(fft_result);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());
    return res[0..1000].to_vec();
    //return find_freq(fft_result, samplesRate);
}

#[tauri::command]
fn generate_tone(name: &str, duration: u8, freq: f64) {
    println!("Generate tone ");
    let start = Instant::now();

    let signal = generate_wave(duration,freq);
    create_wav(name, signal);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());
}

#[tauri::command]
fn noise_reduction(samples: Vec<i16>, m: usize) -> Vec<i16> {
    println!("Noise reduction ");
    let start = Instant::now();

    let compress_samples = lissage_mobile(samples, m);
    
    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return compress_samples;
}

#[tauri::command]
fn record() {
    println!("Record");
    let start = Instant::now();

    audio_record();

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());
}

#[tauri::command]
fn compress(samples:Vec<i16>) -> (Vec<i16>,i32) {
    println!("Compress");
    let start = Instant::now();

    //process_wav_to_mp3(filepath, "compress.mp3");
    let (comp,bytes) = compression(samples);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    //dbg!(bytes);

    return (comp,bytes);
}

#[tauri::command]
fn reverse_sound(samples:Vec<i16>) -> Vec<i16> {
    println!("Reverse");
    let start = Instant::now();
    
    let reversed_samples = inverse_sound(samples);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());
    return reversed_samples;
}

#[tauri::command]
fn filtre_pass_bas(samples:Vec<i16>, freq:f32, samplesRate: u32) -> Vec<i16> {
    println!("Low pass");
    let start = Instant::now();

    let res = passebas(samples, samplesRate as u64, freq);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return res;
}

#[tauri::command]
fn filtre_pass_haut(samples:Vec<i16>, freq:f32, samplesRate: u32) -> Vec<i16> {
    println!("High pass");
    let start = Instant::now();

    let high_pass_samples = passehaut(samples, samplesRate, freq);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return high_pass_samples;
}

#[tauri::command]
fn generate_melody(duration: u8) {
    println!("Generate melody");
    let start = Instant::now();

    // let melody = vec![Sol,Mi,Mi,Mi,Mi,Re,Re,Re,Do,Do,Do
    // ,Mi,Mi,Mi,Mi,Re,Re,Re,Do,Do,Do,Do,
    // Re,Do,Re,Mi,La,Fa,Fa,Fa,Fa,Mi,Mi,Mi,
    // Re,Re,Re,Mi,Fa,Fa,Fa,Fa,Mi,Mi,Re,Re,Re,Do,
    // Re,Do,Re,Mi,Do,Re,Do,Re,Mi,Do,Re,Do,Re,Mi,
    // Mi,Fa,Mi,Si,Fa,Mi,Mi,Re,Do,Si,Si,Do,Re,Sol,
    // Do,Re,Mi,Mi,Fa,Mi,Si,Fa,Mi,Mi,Re,Do,Do,Re,Do,
    // Sol,Do,Do];

    // let mut transition_matrix = transition_matrix(melody);

    // train_data(&mut transition_matrix, 100000, duration,false);
    let transition_matrix = load_data("train_data.txt");
    let melody_samples = random_melody(transition_matrix, 55 as u16);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    create_wav("generated_melody", melody_samples);
}

#[tauri::command]
fn left_only(samples:Vec<i16>) -> Vec<i16> {
    println!("Cut right");
    let start = Instant::now();

    let cut = left_wav(samples);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return cut;
}

#[tauri::command]
fn right_only(samples:Vec<i16>) -> Vec<i16> {
    println!("Cut left");
    let start = Instant::now();

    let cut = right_wav(samples);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return cut;
}

#[tauri::command]
fn alternate(samples:Vec<i16>) -> Vec<i16> {
    println!("Alternate channels");
    let start = Instant::now();

    let alter = leftnright(samples);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return alter;
}

#[tauri::command]
fn egalise(s:Vec<i16>,eps:i64,db:f64,l_freq_min:i64,l_freq_max:i64) -> Vec<i16> {
    println!("Egaliseur");
    let start = Instant::now();

    let egalised_samples = egaliseur(s, eps, db, l_freq_min, l_freq_max);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());

    return egalised_samples;
}

#[tauri::command]
fn generate_melody_midi(style:String,metric:u16) {

    println!("Generating melody");
    let start = Instant::now();
    
    generate_melody_mid(style,metric);

    let end = Instant::now();
    println!("Finished ✅ ({}s)",(end.duration_since(start)).as_secs_f64());
}

#[tauri::command]
fn distortion(samples: Vec<i16>, disto: f32) -> Vec<i16> {
    distorsion(samples, disto)
}

#[tauri::command]
fn fade(samples: Vec<i16>) -> Vec<i16> {
    fade_start_end(samples, 5, 5, 44100)
}

#[tauri::command]
fn ambiance(samples: Vec<i16>, num: i16) -> Vec<i16> {
    ambience(samples, num)
}

#[tauri::command]
fn fusion(samples1: Vec<i16>, samples2: Vec<i16>) -> Vec<i16> {
    mix_samples(samples1, samples2)
}

#[tauri::command]
fn mute(samples: Vec<i16>, start: f32, end: f32) -> Vec<i16> {
    mutebetween(samples, start, end)
}

#[tauri::command]
fn repetition1(samples: Vec<i16>, start: f64, end: f64, n: usize) -> Vec<i16> {
    repeat_section(samples, start, end, n, 44100)
}

#[tauri::command]
fn repetition2(samples: Vec<i16>, start: f32, end: f32, n: usize) -> Vec<i16> {
    repeat_segment(samples, start, end, n, 44100)
}


#[tauri::command]
fn echo(samples: Vec<i16>) -> Vec<i16> {
    reso_echo(samples)
}

#[tauri::command]
fn vibration(samples: Vec<i16>, freq: f32) -> Vec<i16> {
    vibrato(samples, freq)
}

#[tauri::command]
fn recognize_music(samples:Vec<i16>) -> String {
    list_wav(samples, 10000)
}

#[tauri::command]
fn autotune(samples:Vec<i16>,freq:i16,note:i16) -> Vec<i16> {
    auto_tune(samples, freq, note)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,load,speedUp,save,gainUp,FFT,apply_change,generate_tone,noise_reduction,record,compress,reverse_sound,generate_melody,right_only,left_only,alternate,filtre_pass_bas,filtre_pass_haut,egalise,generate_melody_midi,distortion,fade,ambiance,fusion,mute,repetition1,repetition2,echo,vibration,recognize_music,autotune])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    
    // let samples = generate_music(30);
    // create_wav("generate_music", samples);

    let melody:Vec<Note> = vec![Mi,Mi,Fa,Sol,Sol,Fa,Mi,Re,Do,Do,Re,Mi,Mi,Re,Re,
                                Mi,Mi,Fa,Sol,Sol,Fa,Mi,Re,
                                Do,Do,Re,Mi,Re,Do,Do,
                                Re,Re,Mi,Do,Re,Mi,Fa,Mi,Do,
                                Re,Mi,Fa,Mi,Re,Do,Re,Sol,
                                Mi,Mi,Fa,Sol,Sol,Fa,Mi,Re,Do,Do,Re,Mi,Re,Do,Do];

    let melody2 = vec![Sol,Pause,Mi,Mi,Mi,Mi,Pause,Re,Re,Re,Do,Do,Do
                                ,Pause,Mi,Mi,Mi,Mi,Re,Re,Re,Do,Do,Do,Do,
                                Re,Do,Re,Mi,La,Fa,Fa,Fa,Fa,Mi,Mi,Mi,
                                Re,Re,Re,Mi,Fa,Fa,Fa,Fa,Pause,Mi,Mi,Re,Re,Re,Pause,Do,
                                Re,Do,Pause,Re,Mi,Do,Re,Do,Re,Mi,Do,Re,Do,Re,Mi,
                                Mi,Fa,Mi,Si,Fa,Mi,Mi,Re,Do,Si,Si,Do,Re,Sol,
                                Do,Re,Mi,Mi,Pause,Fa,Mi,Si,Fa,Mi,Mi,Re,Do,Pause,Do,Re,Do,
                                Sol,Do,Do];

    let melody4 = vec![Sol,Mi,Mi,Mi,Mi,Re,Re,Re,Do,Do,Do
    ,Mi,Mi,Mi,Mi,Re,Re,Re,Do,Do,Do,Do,
    Re,Do,Re,Mi,La,Fa,Fa,Fa,Fa,Mi,Mi,Mi,
    Re,Re,Re,Mi,Fa,Fa,Fa,Fa,Mi,Mi,Re,Re,Re,Do,
    Re,Do,Re,Mi,Do,Re,Do,Re,Mi,Do,Re,Do,Re,Mi,
    Mi,Fa,Mi,Si,Fa,Mi,Mi,Re,Do,Si,Si,Do,Re,Sol,
    Do,Re,Mi,Mi,Fa,Mi,Si,Fa,Mi,Mi,Re,Do,Do,Re,Do,
    Sol,Do,Do];

    let melody3 = vec![Do,
    Re,
    Mi,
    Sol,
    Sol,
    Fa,
    Mi,
    Re,
    Do,
    Do,
    Re,
    Mi,
    Mi,
    Re,
    Re,
    Do];

    // let dict = transition_matrix(melody);
    // // let samples = random_melody(dict, 15);
    // let samples = random_melody(dict, 27);

    // let mut dict2 = transition_matrix(melody4);
    
    // train_data(&mut dict2, 100000, 55,true);

    // let mut dict2 = load_data("train_data.txt");

    // let samples = random_melody(dict2, 55);
    
    // create_wav("melody_markov", samples);



    // let mut wav = WavReader::new("generate_music.wav");
    // let samples = wav.get_samples();
    // speed(&mut wav,2.0);
    // wav.write_wav("generate_music.wav");

}
