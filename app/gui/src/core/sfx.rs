use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::fs::File;
use rodio::{source::Buffered, Decoder, Source};
use static_init::{dynamic};

type SoundBuffer = Buffered<Decoder<Cursor<&'static [u8]>>>;

const SFX_1: &[u8] = include_bytes!("../../res/sfx/sfx_1.wav");
const SFX_2: &[u8] = include_bytes!("../../res/sfx/sfx_2.wav");

#[dynamic]
static SFX: HashMap<&'static str, SoundBuffer> = {
    let c: &[(&str, &[u8])] = &[
        ("sfx_1", SFX_1),
        ("sfx_2", SFX_2)
    ];

    c
        .into_iter()
        .map(|(x,y)| (*x, Decoder::new(Cursor::new(*y)).unwrap().buffered()))
        .collect()

};

#[test]
fn a() {
    use std::time::Duration;
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    // let (x,y) = rodio::dynamic_mixer::mixer(1, 44100);
    
    for i in 0..3 {

        // std::thread::sleep(Duration::from_millis(500));
        // sink.append(SFX.get("sfx_1").unwrap().clone());
        // sink.sleep_until_end();
        std::thread::sleep(Duration::from_millis(1000));
        // x.add(SFX.get("sfx_2").unwrap().clone());
        sink.append(SFX.get("sfx_1").unwrap().clone());
        sink.sleep_until_end();
        std::thread::sleep(Duration::from_millis(1000));

        sink.append(SFX.get("sfx_2").unwrap().clone());
        sink.sleep_until_end();
        // sink.stop();
    }    
}