use clap::{value_parser, Arg, ArgAction, Command};
use pv_recorder::PvRecorderBuilder;
use std::sync::atomic::{AtomicBool, Ordering};
use crossterm::event::{self, KeyCode, KeyEvent};
use crossterm::event::Event;
use std::thread;
use rfd::MessageDialog;

const SAMPLE_RATE: usize = 16000;
static LISTENING: AtomicBool = AtomicBool::new(false);


//Pour voir les périphériques audio dispo
pub fn show_audio_devices() 
{
    let audio_devices = PvRecorderBuilder::default().get_available_devices();
    match audio_devices 
    {
        Ok(audio_devices) => {
            for (idx, device) in audio_devices.iter().enumerate() 
            {
                println!("{}: {:?}", idx, device);
            }
        }
        Err(err) => panic!("Failed to get audio devices: {}", err),
    };
    println!();
}


/*
pub fn audio_record() 
{
    let matches = Command::new("PvRecorder Demo")
        .arg(
            Arg::new("audio_device_index")
                .long("audio_device_index")
                .value_name("INDEX")
                .help("Index of input audio device.")
                .value_parser(value_parser!(i32))
                .default_value("-1"),
        )
        .arg(
            Arg::new("output_wav_path")
                .long("output_wav_path")
                .value_name("PATH")
                .help("Path to write recorded audio wav file to.")
                .default_value("Recorded_audio.wav"),
        )
        .arg(
            Arg::new("show_audio_devices")
                .long("show_audio_devices")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    
    if matches.get_flag("show_audio_devices") 
    { 
        return show_audio_devices(); 
    }
    let audio_device_index = *matches.get_one::<i32>("audio_device_index").unwrap();
    let output_wav_path = matches.get_one::<String>("output_wav_path").unwrap();
    //Choisi le premier micro

    let recorder = PvRecorderBuilder::new(512)
        .device_index(audio_device_index)
        .init()
        .expect("Failed to initialize pvrecorder");
    //ctrlc::set_handler(|| { LISTENING.store(false, Ordering::SeqCst); }).expect("Error"); C'est pour arrêter l'execution avec Ctrl + c

    println!("Start recording...");
    recorder.start().expect("Failed to start audio recording");
    LISTENING.store(true, Ordering::SeqCst);

    let mut audio_data = Vec::new();

    // Fonction pour vérifier si 's' est pressé (ouais on peut faire des fonctions dans des fonctions)
    fn is_stop_key_event(event: &Event) -> bool 
    {
        if let Event::Key(KeyEvent { code, .. }) = event 
        {
            if let KeyCode::Char(c) = code 
            {
                return *c == 's';
            }
        }
        false
    }

    while LISTENING.load(Ordering::SeqCst) 
    {
        if event::poll(std::time::Duration::from_millis(10)).expect("Failed to poll events") 
        {
            if let Event::Key(KeyEvent { code, .. }) = event::read().expect("Failed to read event") 
            {
                if code == KeyCode::Char('s') 
                {
                    LISTENING.store(false, Ordering::SeqCst);
                    break; // Sort de la boucle d'enregistrement quand 's' est appuyé
                }
            }
        }
        let frame = recorder.read().expect("Failed to read audio frame");
        audio_data.extend_from_slice(&frame);
    }

    println!("Stop recording");
    recorder.stop().expect("Failed to stop audio recording");
    println!("Creating the audio file...");

    let spec = hound::WavSpec 
    {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_wav_path, spec).unwrap();

    for sample in audio_data 
    {
        writer.write_sample(sample).unwrap();
    }
}
*/

pub fn audio_record() 
{
    let matches = Command::new("PvRecorder Demo")
        .arg(
            Arg::new("audio_device_index")
                .long("audio_device_index")
                .value_name("INDEX")
                .help("Index of input audio device.")
                .value_parser(value_parser!(i32))
                .default_value("-1"),
        )
        .arg(
            Arg::new("output_wav_path")
                .long("output_wav_path")
                .value_name("PATH")
                .help("Path to write recorded audio wav file to.")
                .default_value("Recorded_audio.wav"),
        )
        .arg(
            Arg::new("show_audio_devices")
                .long("show_audio_devices")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    
    if matches.get_flag("show_audio_devices") 
    { 
        return show_audio_devices(); 
    }

    let audio_device_index = *matches.get_one::<i32>("audio_device_index").unwrap();
    let output_wav_path = matches.get_one::<String>("output_wav_path").unwrap();
    //Choisi le premier micro


    let recorder = PvRecorderBuilder::new(512)
        .device_index(audio_device_index)
        .init()
        .expect("Failed to initialize pvrecorder");

    println!("Start recording...");
    recorder.start().expect("Failed to start audio recording");
    LISTENING.store(true, Ordering::SeqCst);

    let mut audio_data = Vec::new();

    thread::spawn(move || {
        MessageDialog::new()
            .set_title("Recording...")
            .set_description("Click OK to stop the recording.")
            .show();
        LISTENING.store(false, Ordering::SeqCst);
    });

    while LISTENING.load(Ordering::SeqCst) 
    {
        let frame = recorder.read().expect("Failed to read audio frame");
        audio_data.extend_from_slice(&frame);
    }

    println!("Stop recording");
    recorder.stop().expect("Failed to stop audio recording");
    println!("Creating the audio file...");

    let spec = hound::WavSpec 
    {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_wav_path, spec).unwrap();

    for sample in audio_data 
    {
        writer.write_sample(sample).unwrap();
    }
}