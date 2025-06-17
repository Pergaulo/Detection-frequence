use rodio::{Decoder, OutputStream, source::Source};

fn playSound()
{ 
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("sounds/Sample_03.wav").unwrap();
    //let source = Decoder::new(file).unwrap();
    //let rate = rodio::source::Source::sample_rate(&source);
    //dbg!(rate);
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}