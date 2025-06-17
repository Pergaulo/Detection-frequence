//use rustfft::{FftPlanner, num_complex::Complex};
use crate::WavReader::wavreader::*;
use hound;
use nannou::mesh::channel;
use plotters::prelude::*;
use rustfft::num_traits::Pow;
use rustfft::FftPlanner;
use rustfft::FftDirection;

use rand::thread_rng;
use rand::Rng;

extern crate rodio;
extern crate plotters;


use std::ops::Sub;
use std::{fs::File, ops::Add};
use std::io::BufReader;
use rodio::{Decoder, Source};

use num_complex::{Complex, ComplexFloat};
use std::f64::consts::PI;

//Compute the fft of samples
pub fn __fft(samples: Vec<Complex<f64>>) -> Vec<Complex<f64>>
{
    let n = samples.len();
    if n == 1
        {return samples;}

    let w = Complex::exp(Complex::new(0.0, -2.0 * std::f64::consts::PI / (n as f64)));
    let mut Pe = vec![];
    let mut Po = vec![];
    for i in 0..n
    {
        if i % 2 == 0
            {Pe.push(samples[i]);}
        else
            {Po.push(samples[i]);}
    }
    let ye = __fft(Pe);//dft(Pe.iter().map(|&x| x.re()).collect());//__fft(Pe);
    let yo = __fft(Po);//dft(Po.iter().map(|&x| x.re()).collect());//__fft(Po);
    let mut y = vec![Complex::new(0.0,0.0);n];
    for j in 0..(n/2)
    {
        y[j] = (ye[j] + (w.powi(j as i32)*yo[j]));
        y[j + n/2] = (ye[j] - (w.powi(j as i32)*yo[j]));
    }
    return y;

}

pub fn fft(samples: Vec<i16>, channel: u8) -> Vec<Complex<f64>>
{
    if channel == 2 {
        let formated_samples = mono(formated_samples(samples));
        let completed_samples = formated_samples.clone();
        nextpower(&mut completed_samples.iter().map(|&x| x as i16).collect());

        return __fft(completed_samples.iter().map(|&x| Complex::new(x as f64, 0.0)).collect());
    }

    let mut completed = samples.clone();
    let n = nextpower(&mut completed);

    return __fft(samples.iter().map(|&x| Complex::new(x as f64, 0.0)).collect());
    //let samples_mono = mono(formated_samples(samples));
    /*let mut to_fft: Vec<Complex<f64>> = samples.iter().map(|&x| Complex::new(x as f64,0.0)).collect();
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(to_fft.len());
    
    fft.process(&mut to_fft);
    //let ifft = planner.plan_fft_inverse(to_fft.len());
    //ifft.process(&mut to_fft);
    
    dbg!(to_fft[2000]);
    let res2: Vec<f64> = to_fft.iter().map(|&x| f64::sqrt((x.re()*x.re())+(x.im()*x.im()))).collect();
    dbg!(res2[2000]);
    return res2;

    let mut fft_result: Vec<f64> = vec![];
    //let samples_formated: Vec<[i16; 2]> = formated_samples(samples);
    //let samples_mono = mono(formated_samples(samples));

    //MA FFT
    let mut res = __fft(samples.iter().map(|&x| Complex::new(x as f64, 0.0)).collect());
    //res = res.iter().map(|x| (x/samples.len() as f64)).collect();
    let my_result: Vec<f64> = res.iter().map(|&x| f64::sqrt((x.re()*x.re())+(x.im()*x.im()))).collect();
    dbg!(res[2000]);
    //dbg!(my_result[2000]);
    return my_result;
    //fft_result.push(res.iter().map(|&x| x.re()).collect());

    /*
    for sample in &samples_formated
    {
        let freq: Vec<Complex<f64>> = __fft(sample.to_vec().iter().map(|&x| Complex::new(x as f64, 0.0)).collect());
        fft_result.push(freq.iter().map(|&x| x.re()).collect());
    }*/
    
    //return fft_result;*/
}

pub fn __ifft(fft_result: Vec<Complex<f64>>) -> Vec<Complex<f64>>
{
    let n = fft_result.len();
    if n == 1
        {return fft_result;}
    
    let w = Complex::exp((Complex::new(0.0, 2.0) * Complex::new(PI, 0.0) / Complex::new(n as f64, 0.0)));
    let mut Pe = vec![];
    let mut Po = vec![];
    for i in 0..n
    {
        if i % 2 == 0
            {Pe.push(fft_result[i]);}
        else
            {Po.push(fft_result[i]);}
    }
    let ye = __ifft(Pe);
    let yo = __ifft(Po);
    let mut y = vec![Complex::new(0.0,0.0);n];
    for j in 0..(n/2)
    {
        y[j] = ye[j] + (w.powi(j as i32)*yo[j]);
        y[j + n/2] = ye[j] - (w.powi(j as i32)*yo[j]);
    }
    return y;   
}

pub fn ifft(fft_result: Vec<Complex<f64>>) -> Vec<i16>
{
    let l = fft_result.len();
    let res = __ifft(fft_result);
    let r:Vec<f64> = res.iter().map(|x| (x/l as f64).re().round()).collect();
    //let r: Vec<f64> = res.iter().map(|&x| f64::sqrt((x.re()*x.re())+(x.im()*x.im()))).collect();
    let r2 = r.iter().map(|&x| x as i16).collect();
    return r2;
    /*let res = __ifft(fft_result.to_vec().iter().map(|&x| Complex::new(x, 0.0)).collect());
    let my_result: Vec<f64> = res.iter().map(|&x| f64::sqrt((x.re()*x.re())+(x.im()*x.im()))).collect();
    let samples = my_result.iter().map(|&x| x as i16).collect();
    return samples;*/

    /*let mut ifft_result: Vec<i16> = vec![];
    for sample in &fft_result
    {
        let freq: Vec<Complex<f64>> = __ifft(sample.to_vec().iter().map(|&x| Complex::new(x, 0.0)).collect());
        for f in &freq
        {
            ifft_result.push(((f.re())/freq.len() as f64) as i16);
        }
    }
    return ifft_result;*/

}

pub fn mono(samples: Vec<[i16; 2]>) -> Vec<f64>
{
    let mut m: Vec<f64> = vec![];
    for s in samples
    {
        m.push(((s[0] as f64 +s[1] as f64)/2.0));
    }
    return m;
}

pub fn formated_samples(samples: Vec<i16>) -> Vec<[i16; 2]>
{
    let mut samples_formated: Vec<[i16;2]> = vec![];
    let mut i = 0;
    let mut s = [0,0];
    for val in samples
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
}

pub fn nextpower(samples: &mut Vec<i16>) -> u64
{
    let mut n: u32 = 1;
    while 2_u64.pow(n) < samples.len() as u64
    {
        n+= 1;
    }
    let treshold = 2_u64.pow(n) as usize;
    let mut rd = thread_rng();
    let mut cpt: u64 = 0;
    while samples.len() < treshold
    {
        let r = rd.gen_range(100..200);
        samples.push(r);
        cpt += 1;
    }
    cpt
}

pub fn complete_samples(samples: &mut Vec<i16>, n: u64)
{
    let mut cpt = 0;
    while cpt < n
    {
        samples.push(samples[cpt as usize]);
        cpt += 1;
    }
}

pub fn find_freq(fft_result: Vec<Complex<f64>>, sr: u64) -> f64
{
    let mut k = 0;
    let mut max2 = 0.0;
    for i in 0..fft_result.len()
    {
        if fft_result[i].abs().round() > max2
        {
            max2 = fft_result[i].abs();
            k = i;
        }
    }
    return (k as f64/fft_result.len() as f64)*sr as f64;
}

pub fn normalize(fft_result: Vec<Complex<f64>>) -> Vec<f64>
{
    let mut res = vec![0.0;44100];
    for i in 0..fft_result.len()
    {
        res[((i as f64/fft_result.len() as f64)*44100 as f64) as usize] += fft_result[i].abs().round() as f64;
    }
    let mut max = (0.0,0);
    for i in 0..44100
    {
        if res[i] > max.0 {
            max.0 = res[i];
            max.1 = i;
        }
    }
    /*for i in 0..fft_result.len()
    {
        res.push(fft_result[i].abs().round());
    }*/
    return res.iter().map(|&x| x/max.0 as f64).collect();
}

pub fn dft(echantillons_audio: Vec<f64>) -> Vec<Complex<f64>> {
    let n = echantillons_audio.len();
    let mut dft_result: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); n];
    let w = Complex::exp((Complex::new(0.0, -2.0) * Complex::new(PI, 0.0) / Complex::new(n as f64, 0.0)));

    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for t in 0..n {
            let angle = -2.0 * std::f64::consts::PI * (k as f64) * (t as f64) / (n as f64);
            let exp_term = Complex::new(angle.cos(), angle.sin());
            sum += Complex::new(echantillons_audio[t], 0.0) * w.powf(k as f64 *t as f64);
        }
        dft_result[k] = sum;
    }

    dft_result
}

pub fn fft_radix7(samples: Vec<Complex<f64>>) -> Vec<Complex<f64>>
{
    let n = samples.len();
    if n == 1
        {return samples;}

    let w = Complex::exp(Complex::new(0.0, -2.0 * std::f64::consts::PI / (n as f64)));
    let mut P1 = vec![];
    let mut P2 = vec![];
    let mut P3 = vec![];
    let mut P4 = vec![];
    let mut P5 = vec![];
    let mut P6 = vec![];
    let mut P7 = vec![];

    for i in 0..n/7
    {
        P1.push(samples[7*i]);
        P2.push(samples[7*i+1]);
        P3.push(samples[7*i+2]);
        P4.push(samples[7*i+3]);
        P5.push(samples[7*i+4]);
        P6.push(samples[7*i+5]);
        P7.push(samples[7*i+6]);
    }
    let y1 = fft_radix3(P1);//dft(Pe.iter().map(|&x| x.re()).collect());//__fft(Pe);
    let y2 = fft_radix3(P2);
    let y3 = fft_radix3(P3);
    let y4 = fft_radix5(P4);
    let y5 = fft_radix5(P5);
    let y6 = fft_radix5(P6);
    let y7 = fft_radix5(P7);//dft(Po.iter().map(|&x| x.re()).collect());//__fft(Po);
    let mut y = vec![Complex::new(0.0,0.0);n];
    for j in 0..(n/5)
    {
        let wj = w.powi(j as i32);
        let wj2 = wj.powi(2);
        let wj3 = wj.powi(3);
        let wj4 = wj.powi(4);
        let wj5 = wj.powi(5);
        let wj6 = wj.powi(6);

        y[j] = y1[j] + wj * y2[j] + wj2 * y3[j] + wj3 * y4[j] + wj4 * y5[j] + wj5 * y6[j] + wj6 * y7[j];
        y[j + n/7] = y1[j] + w.powi(j as i32 + n as i32 /7) * y2[j] + w.powi(j as i32 + n as i32/7).powi(2) * y3[j] + w.powi(j as i32 + n as i32/7).powi(3) * y4[j] + w.powi(j as i32 + n as i32/7).powi(4) * y5[j] + w.powi(j as i32 + n as i32/7).powi(5) * y6[j] + w.powi(j as i32 + n as i32/7).powi(6) * y7[j];
        y[j + 2*(n/7)] = y1[j] + w.powi(j as i32 + 2*(n as i32 /7)) * y2[j] + w.powi(j as i32 + 2*(n as i32 /7)).powi(2) * y3[j] + w.powi(j as i32 + 2*(n as i32 /7)).powi(3) * y4[j] + w.powi(j as i32 + 2*(n as i32 /7)).powi(4) * y5[j] + w.powi(j as i32 + 2*(n as i32 /7)).powi(5) * y6[j] + w.powi(j as i32 + 2*(n as i32 /7)).powi(6) * y7[j];
        y[j + 3*(n/7)] = y1[j] + w.powi(j as i32 + 3*(n as i32 /7)) * y2[j] + w.powi(j as i32 + 3*(n as i32 /7)).powi(2) * y3[j] + w.powi(j as i32 + 3*(n as i32 /7)).powi(3) * y4[j] + w.powi(j as i32 + 3*(n as i32 /7)).powi(4) * y5[j] + w.powi(j as i32 + 3*(n as i32 /7)).powi(5) * y6[j] + w.powi(j as i32 + 3*(n as i32 /7)).powi(6) * y7[j];
        y[j + 4*(n/7)] = y1[j] + w.powi(j as i32 + 4*(n as i32 /7)) * y2[j] + w.powi(j as i32 + 4*(n as i32 /7)).powi(2) * y3[j] + w.powi(j as i32 + 4*(n as i32 /7)).powi(3) * y4[j] + w.powi(j as i32 + 4*(n as i32 /7)).powi(4) * y5[j] + w.powi(j as i32 + 4*(n as i32 /7)).powi(5) * y6[j] + w.powi(j as i32 + 4*(n as i32 /7)).powi(6) * y7[j];
        y[j + 5*(n/7)] = y1[j] + w.powi(j as i32 + 5*(n as i32 /7)) * y2[j] + w.powi(j as i32 + 5*(n as i32 /7)).powi(2) * y3[j] + w.powi(j as i32 + 5*(n as i32 /7)).powi(3) * y4[j] + w.powi(j as i32 + 5*(n as i32 /7)).powi(4) * y5[j] + w.powi(j as i32 + 5*(n as i32 /7)).powi(5) * y6[j] + w.powi(j as i32 + 5*(n as i32 /7)).powi(6) * y7[j];
        y[j + 6*(n/7)] = y1[j] + w.powi(j as i32 + 6*(n as i32 /7)) * y2[j] + w.powi(j as i32 + 6*(n as i32 /7)).powi(2) * y3[j] + w.powi(j as i32 + 6*(n as i32 /7)).powi(3) * y4[j] + w.powi(j as i32 + 6*(n as i32 /7)).powi(4) * y5[j] + w.powi(j as i32 + 6*(n as i32 /7)).powi(5) * y6[j] + w.powi(j as i32 + 6*(n as i32 /7)).powi(6) * y7[j];
    }   
    return y;

}

pub fn fft_radix5(samples: Vec<Complex<f64>>) -> Vec<Complex<f64>>
{
    let n = samples.len();
    if n == 1
        {return samples;}

    let w = Complex::exp(Complex::new(0.0, -2.0 * std::f64::consts::PI / (n as f64)));
    let mut P1 = vec![];
    let mut P2 = vec![];
    let mut P3 = vec![];
    let mut P4 = vec![];
    let mut P5 = vec![];

    for i in 0..n/5
    {
        P1.push(samples[5*i]);
        P2.push(samples[5*i+1]);
        P3.push(samples[5*i+2]);
        P4.push(samples[5*i+3]);
        P5.push(samples[5*i+4]);
    }
    let y1 = fft_radix3(P1);//dft(Pe.iter().map(|&x| x.re()).collect());//__fft(Pe);
    let y2 = fft_radix3(P2);
    let y3 = fft_radix3(P3);
    let y4 = fft_radix5(P4);
    let y5 = fft_radix5(P5);//dft(Po.iter().map(|&x| x.re()).collect());//__fft(Po);
    let mut y = vec![Complex::new(0.0,0.0);n];
    for j in 0..(n/5)
    {
        let wj = w.powi(j as i32);
        let wj2 = wj.powi(2);
        let wj3 = wj.powi(3);
        let wj4 = wj.powi(4);

        y[j] = y1[j] + wj * y2[j] + wj2 * y3[j] + wj3 * y4[j] + wj4 * y5[j];
        y[j + n/5] = y1[j] + w.powi(j as i32 + n as i32 /5) * y2[j] + w.powi(j as i32 + n as i32/5).powi(2) * y3[j] + w.powi(j as i32 + n as i32/5).powi(3) * y4[j] + w.powi(j as i32 + n as i32/5).powi(4) * y5[j];
        y[j + 2*(n/5)] = y1[j] + w.powi(j as i32 + 2*(n as i32 /5)) * y2[j] + w.powi(j as i32 + 2*(n as i32 /5)).powi(2) * y3[j] + w.powi(j as i32 + 2*(n as i32 /5)).powi(3) * y4[j] + w.powi(j as i32 + 2*(n as i32 /5)).powi(4) * y5[j];
        y[j + 3*(n/5)] = y1[j] + w.powi(j as i32 + 3*(n as i32 /5)) * y2[j] + w.powi(j as i32 + 3*(n as i32 /5)).powi(2) * y3[j] + w.powi(j as i32 + 3*(n as i32 /5)).powi(3) * y4[j] + w.powi(j as i32 + 3*(n as i32 /5)).powi(4) * y5[j];
        y[j + 4*(n/5)] = y1[j] + w.powi(j as i32 + 4*(n as i32 /5)) * y2[j] + w.powi(j as i32 + 4*(n as i32 /5)).powi(2) * y3[j] + w.powi(j as i32 + 4*(n as i32 /5)).powi(3) * y4[j] + w.powi(j as i32 + 4*(n as i32 /5)).powi(4) * y5[j];
    }
    return y;

}

pub fn fft_radix3(samples: Vec<Complex<f64>>) -> Vec<Complex<f64>>
{
    let n = samples.len();
    if n == 1
        {return samples;}

    let w = Complex::exp(Complex::new(0.0, -2.0 * std::f64::consts::PI / (n as f64)));
    let mut P1 = vec![];
    let mut P2 = vec![];
    let mut P3 = vec![];
    for i in 0..n/3
    {
        P1.push(samples[3*i]);
        P2.push(samples[3*i+1]);
        P3.push(samples[3*i+2]);
    }
    let y1 = fft_radix3(P1);//dft(Pe.iter().map(|&x| x.re()).collect());//__fft(Pe);
    let y2 = fft_radix3(P2);
    let y3 = fft_radix3(P3);//dft(Po.iter().map(|&x| x.re()).collect());//__fft(Po);
    let mut y = vec![Complex::new(0.0,0.0);n];
    for j in 0..(n/3)
    {
        y[j] = y1[j] + w.powi(j as i32) + y2[j] + (w.powi(j as i32)).powi(2) * y3[j];
        y[j + n/3] = y1[j] + w.powi(j as i32 + n as i32/3) * y2[j] + (w.powi(j as i32 + n as i32 /3)).powi(2) * y3[j];
        y[j + 2*(n/3)] = y1[j] + w.powi(j as i32 + 2*(n as i32 /3)) * y2[j] + (w.powi(j as i32 + 2*(n as i32/3))).powi(2) * y3[j];
    }
    return y;

}

pub fn fft_mixed_radix(samples: Vec<Complex<f64>>) -> Vec<Complex<f64>>
{
    let mut n = samples.len();
    if n == 1
        {return samples;}

    let prime = [2,3,5,7];
    let mut factors = vec![];
    for p in &prime
    {
        while n % *p == 0
        {
            factors.push(*p);
            n /= *p;
        }
    }
    let mut p = vec![];
    p.push(factors.iter().filter(|&x| *x == 2 as usize).count());
    p.push(factors.iter().filter(|&x| *x == 3 as usize).count());
    p.push(factors.iter().filter(|&x| *x == 5 as usize).count());
    p.push(factors.iter().filter(|&x| *x == 7 as usize).count());
    let mut res = &samples;
    
    
    let mut A = vec![];
    let mut B = vec![];
    let mut C = vec![];
    let mut D = vec![];
    let mut E = vec![];
    for i in 0..2
    {
        A.push(samples[i]);
    }
    for i in 2..4
    {
        B.push(samples[i]);
    }
    for i in 4..6
    {
        C.push(samples[i]);
    }
    for i in 0..3
    {
        D.push(samples[i]);
    }
    for i in 3..6
    {
        E.push(samples[i]);
    }
    let mut r2 = dft(A.iter().map(|&x| x.re()).collect());
    let mut r22 =dft(B.iter().map(|&x| x.re()).collect());
    let mut r23 = dft(C.iter().map(|&x| x.re()).collect());
    let r3 = dft(D.iter().map(|&x| x.re()).collect());
    let r32 = dft(E.iter().map(|&x| x.re()).collect());

    r2.extend(r22);
    r2.extend(r23);

    //dbg!(&r2);

    r2[0] += r3[0];
    r2[0] += r32[0];
    r2[2] += r3[2];
    r2[2] += r32[2];
    r2[4] += r3[2];
    r2[4] += r32[2];

    return r2;
    /*
    for p in &factors
    {
        if *p == 2
            {
                res = __fft(res);
            }
        /*else if *p == 3
            {res = fft_radix3(res);}*/
        /*else if *p == 5
            {res = fft_radix5(res);}*/
        else if *p == 7
            {res = fft_radix7(res);}
    }*/
    //return res;
}

//Show the signal on graph
pub fn graph() ->Result<(), Box<dyn std::error::Error>>
{
    // Ouvrir le fichier audio
    let file = File::open("sounds/Sample_03.wav")?;
    let source = Decoder::new(BufReader::new(file))?;

    // Lire les échantillons audio dans un vecteur
    let samples: Vec<f32> = source.convert_samples::<f32>().collect();

    let samples_f64: Vec<f64> = samples.iter().map(|&x| x as f64).collect();
    
    // Créer un graphique
    let root = BitMapBackend::new("signal_audio.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Définir les limites de l'axe des x et des y
    let x_min = 0.0;
    let x_max = samples_f64.len() as f64;
    let y_min = -1.0;
    let y_max = 1.0;

    // Créer un système de coordonnées
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("Signal Audio", ("Arial", 20))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Dessiner le signal audio
    chart
        .draw_series(LineSeries::new(
            (0..samples.len()).map(|i| (i as f64, samples_f64[i])),
            &BLACK,
        ))?
        .label("Signal Audio")
        .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], BLACK.filled()));

    // Afficher le graphique dans un fichier
    chart.configure_series_labels().draw()?;
    root.present()?;
        
    Ok(())
}