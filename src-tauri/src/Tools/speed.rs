use crate::WavReader::wavreader::*;

pub fn speed(wav: &mut WavReader, speed: f64)
{
    let Fe = (wav.sample_rate as f64 * speed) as u32;
    let b1 = (Fe & 0b11111111) as u8;
    let b2 = (Fe >> 8) as u8;
    let b3 = (Fe >> 16) as u8;
    let b4 = (Fe >> 24) as u8;
    let newFe = [b1,b2,b3,b4];
    wav.set_header(newFe, 6);
}

pub fn speed2(header: Vec<[u8;4]>, sample_rate: u64, speed: f64) -> Vec<[u8;4]>
{
    let mut header2 = header.clone();
    let Fe = (sample_rate as f64 * speed) as u32;
    let b1 = (Fe & 0b11111111) as u8;
    let b2 = (Fe >> 8) as u8;
    let b3 = (Fe >> 16) as u8;
    let b4 = (Fe >> 24) as u8;
    let newFe = [b1,b2,b3,b4];
    header2[6] = newFe;
    return header2;
}