use std::{fs::File, time::Duration};
use std::io::BufReader;
use rodio::{Sink, SpatialSink};
use rodio::{Decoder, OutputStream, source::{Source, SineWave}};

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    
    let file = File::open("assets/test.mp3").unwrap();
    let source = Decoder::new_mp3(file).unwrap();
    
    // let duration_in_secs = source.total_duration().unwrap().as_secs();
    // println!("Total duration of current music source is {}:{}", duration_in_secs / 60, duration_in_secs % 60);
    
    let sink = SpatialSink::try_new(&stream_handle, [0.0, 0.0, 2.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap();
    sink.append(source);

    let mut now = std::time::Instant::now();
    let mut t = 0.0;
    loop {
        t += now.elapsed().as_millis() as f32;
        let cor = t * 3.14 / 1000.0;
        now = std::time::Instant::now();
        sink.set_emitter_position([2.0 * cor.sin(), 0.0, 2.0 * cor.cos()]);
        println!("{:?}", t);
    }
    
    sink.sleep_until_end();
}