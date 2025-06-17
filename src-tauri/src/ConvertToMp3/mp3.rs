use crate::WavReader::wavreader::*;

pub fn compression(sample : Vec<i16>) -> (Vec<i16>,i32) {
    let mut bytes = 0;
    let mut res = Vec::new();
    let mut odd = true;
    for i in sample {
        if odd {
            res.push(i);
            bytes += 2;
            odd = false;
        }
        else {
            odd = true;
        }
    }
    (res,bytes)
}

pub fn compression2(sample : Vec<i16>) -> Vec<i16> {
    let mut res = Vec::new();
    let mut odd = 0;
    for i in sample {
        if odd == 2 {
            odd = 0;
        }
        else {
            res.push(i);
            odd+=1;
        }
    }
    res
}
