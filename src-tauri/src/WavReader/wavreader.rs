use std::fs::File;
use std::io::BufReader;
use std::io;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WavReader
{
    pub channels: u8,
    pub sample_rate: u64,
    pub bits_per_sample: u8,
    data_size: u32,
    pub bytes_read: usize,
    //file: std::fs::File,
    file: String,
    pub header: Vec<[u8;4]>,
    pub samples: Vec<i16>,
    samples_bytes: Vec<u8>,
    samples_formated: Vec<[i16;2]>,
    //filename: str
}

impl WavReader
{
    //Constructor
    pub fn new(path: &str) -> Self
    {
        Self
        {
            channels: 0,
            sample_rate: 0,
            bits_per_sample: 0,
            data_size: 0,
            bytes_read: 0,
            //file: std::fs::File::open(path).unwrap(),
            file: path.to_string(),
            header: vec![],
            samples: vec![],
            samples_bytes: vec![],
            samples_formated: vec![],
            //filename: path,
        }
    }

    //Read the header
    pub fn spec(&mut self)
    {
        //buffer to stock 4 bytes at each read
        let mut buffer = [0;4];
        let mut n = 0;
        let mut head: Vec<[u8;4]> = vec![];
        //dbg!(self.file.clone());
        let mut file = std::fs::File::open(self.file.clone()).unwrap();
        //Read the 11 blocks of 4 bytes
        for i in 0..11
        {
            n = /*self.*/file.read(&mut buffer).unwrap();
            head.push(buffer);
            self.bytes_read += n;
        }
        //fill the spec data
        self.header = head.clone();
        self.bits_per_sample = (head[8][2]/*+head[8][3]*256*/) as u8;
        self.channels = head[5][2] as u8;
        self.sample_rate = self.compute_from_bytes(&head[6]) as u64;
        self.data_size = self.compute_from_bytes(&head[10]) as u32;
        //let path: Vec<&str> = self.filename.split("/").collect();
        //self.filename = *path[path.len()-1];
    }

    pub fn get_samples(&mut self) -> Vec<i16>
    {
        if self.bytes_read == 0
            {self.spec();}
        let mut n = 1;
        let mut read = 0;
        let mut samples: Vec<i16> = vec![];
        const bytes_to_read: usize = 2;//self.bits_per_sample/8;
        let mut buffer_samples = [0;bytes_to_read];
        let mut file = std::fs::File::open(self.file.clone()).unwrap();
        //println!("data size : {}",self.data_size);
        while n != 0 && read != self.data_size
        {
            n = /*self.*/file.read(&mut buffer_samples).unwrap();
            for byte in &buffer_samples
            {
                self.samples_bytes.push(*byte);
            }
            samples.push(self.to_i16(&buffer_samples.to_vec()));
            self.bytes_read += n;
            read += n as u32;
        }
        self.samples = samples.clone();
        return samples;
    }

    pub fn set_samples(&mut self, sample: Vec<i16>)
    {
        self.samples = sample.iter().map(|&x| x as i16).collect();
    }
    pub fn set_header(&mut self, val: [u8;4], i: usize)
    {
        self.header[i] = val;
    }
    
    //Formated stereo to mono
    pub fn formated_samples(&mut self) -> Vec<[i16; 2]>
    {
        let mut samples_formated: Vec<[i16;2]> = vec![];
        let mut i = 0;
        let mut s = [0,0];
        for val in self.samples.clone()
        {
            if i == 2
            {
                samples_formated.push(s);
                s = [0,0];
                i = 0;
            }
            s[i] = val;
            i+=1;
        }
        return samples_formated;
        //self.samples_formated = samples_formated;
    }

    pub fn write_wav(&mut self, name: &str)
    {
        //create the new file
        let mut fd: File = File::create(name).unwrap();

        //Write the header
        for chunk in &self.header
        {
            fd.write(chunk);
        }
        /*for chunk in &mut self.samples
        {
            *chunk /= 16;
        }*/

        //Extract form samples the bytes
        let mut vec:Vec<u8> = vec![];
        for elt in &self.samples
        {
            vec.push((elt & 0b11111111) as u8);
            vec.push((elt >> 8) as u8);
        }
        
        //write all the samples as bytes
        fd.write(&(vec));
    }

    pub fn show_spec(&mut self)
    {
        if self.bytes_read == 0
            {self.spec();}
        if self.samples.is_empty()
            {let _ = self.get_samples();}
        println!();
        println!("=============== File Specifications ===============");
        //println!("File Specifications");
        println!("channels: {}",self.channels);
        println!("sample rate: {}kHz",self.sample_rate);
        println!("bits per samples: {} bits",self.bits_per_sample);
        println!("number of sample: {}",self.samples.len());
        println!("Duration: {}s",(self.samples.len()/self.channels as usize)/self.sample_rate as usize);
        println!();
    }

    //return the signed value of vec (vector of bytes)
    fn to_i16(&mut self, vec: &Vec<u8> ) -> i16
    {
        let valeur_signee = (vec[0] as i16) | ((vec[1] as i16) << 8);
        return valeur_signee;
    }

    //Return the decimal value of arr (array of bytes)
    fn compute_from_bytes(&mut self, arr: &[u8;4]) -> u64
    {
        let mut res: u64 = 0;
        for i in 0..arr.len()
        {
            res += (arr[i] as u64)*(256_i32.pow(i.try_into().unwrap()) as u64);
        }
        return res;
    }

}

pub fn create_wav(name:&str, samples:Vec<i16>)
    {
        let mut fd: File = File::create([name,".wav"].concat()).unwrap();
        let size:u32 = samples.len() as u32;
        let b1 = (size & 0b11111111) as u8;
        let b2 = (size >> 8) as u8;
        let b3 = (size >> 16) as u8;
        let b4 = (size >> 24) as u8;
        let new_size = [b1,b2,b3,b4];

        let header = [[82, 73, 70, 70], 
        [172, 88, 1, 0], 
        [87, 65, 86, 69], 
        [102, 109, 116, 32], 
        [16, 0, 0, 0], 
        [1, 0, 1, 0], [68, 172, 0, 0], [136, 88, 1, 0], [2, 0, 16, 0], [100, 97, 116, 97], new_size];
        for chunk in &header
        {
            fd.write(chunk);
        }
        let mut vec:Vec<u8> = vec![];
        for elt in &samples
        {
            vec.push((elt & 0b11111111) as u8);
            vec.push((elt >> 8) as u8);
        }
        
        //write all the samples as bytes
        fd.write(&(vec));
    }