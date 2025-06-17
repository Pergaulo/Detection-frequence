use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use mp3lame_encoder::{encoder, Builder, DualPcm, FlushNoGap, Id3Tag};
use hound::{WavReader, WavSpec};
use std::convert::TryInto;
use crate::WavReader::wavreader::*;
use crate::Tools::speed::*;


pub fn process_wav_to_mp3(input_wav_path: &str, output_mp3_path: &str) -> io::Result<()> {
    // Attempt to open the WAV file
    let mut reader = match WavReader::open(input_wav_path) {
        Ok(reader) => reader,
        Err(err) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to open WAV file: {}", err),
            ));
        }
    };

    // Read PCM data from the input .wav file
    let spec = reader.spec();
    let pcm_data: Vec<_> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    // Initialize LAME encoder
    let mut mp3_encoder = Builder::new().expect("Create LAME builder");
    mp3_encoder
        .set_num_channels((spec.channels as u8).try_into().unwrap())
        .expect("set channels");
    mp3_encoder
        .set_sample_rate((spec.sample_rate as u32).try_into().unwrap())
        .expect("set sample rate");
    mp3_encoder
        .set_brate(mp3lame_encoder::Bitrate::Kbps320)
        .expect("set brate");
    mp3_encoder
        .set_quality(mp3lame_encoder::Quality::Best)
        .expect("set quality");

    let mut mp3_encoder = mp3_encoder.build().expect("To initialize LAME encoder");

    // Encode PCM to MP3
    let mut mp3_out_buffer = Vec::new();
    mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(pcm_data.len()));
    for chunk in pcm_data.chunks(spec.channels as usize) {
        let input = DualPcm {
            left: &chunk,
            right: &chunk,
        };
        let encoded_size = mp3_encoder
            .encode(input, mp3_out_buffer.spare_capacity_mut())
            .expect("Failed to encode");
        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }
    }

    // Flush remaining MP3 data
    let encoded_size = mp3_encoder
        .flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut())
        .expect("Failed to flush");
    unsafe {
        mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
    }

    // Write MP3 data to a file
    let mut file = File::create(output_mp3_path)?;
    file.write_all(&mp3_out_buffer)?;

    Ok(())
}
