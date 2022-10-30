use std::collections::HashMap;
use std::io::Cursor;

use rodio::{source::Buffered, Decoder, Source};
use once_cell::sync::Lazy;

type SoundBuffer = Buffered<Decoder<Cursor<&'static [u8]>>>;

const SFX_1: &[u8] = include_bytes!("../../res/sfx/sfx_1.wav");
const SFX_2: &[u8] = include_bytes!("../../res/sfx/sfx_2.wav");
// const SFX_3: &[u8] = include_bytes!("../../res/sfx/riff.wav");
// const SFX_4: &[u8] = include_bytes!("../../res/sfx/aauugghh.wav");

pub static SFX: Lazy<HashMap<&'static str, SoundBuffer>> = Lazy::new(|| {
    let sfx: &[(&str, &[u8])] = &[
        ("sfx_1", SFX_1),
        ("sfx_2", SFX_2),
        // ("sfx_3", SFX_3),
        // ("sfx_4", SFX_4),
    ];

    sfx
        .into_iter()
        .map(|(x,y)| (*x, Decoder::new(Cursor::new(*y)).unwrap().buffered()))
        .collect()
});

pub struct Audio {
    _stream: rodio::OutputStream,
    handle: rodio::OutputStreamHandle,
    sink: Option<rodio::Sink>
}

impl Default for Audio {
    fn default() -> Self {
        Self::init()
    }
}

impl Audio {
    pub fn init() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let mut a = Self {
            _stream, handle, sink: None
        };
        a.sink = Some(rodio::Sink::try_new(&a.handle).unwrap());
        a
    }

    pub fn play(&self, sfx: &str) {
        if let Some(sound) = SFX.get(sfx) {
            self.sink.as_ref().unwrap().append(sound.clone());
        }
    }
}

#[test]
fn a() {
    use std::time::Duration;
    // let (x,y) = rodio::dynamic_mixer::mixer(1, 44100);
    let player = Audio::default();
    player.play("sfx_1");
    
    for i in 0..3 {
        std::thread::sleep(Duration::from_millis(1000));
        player.play("sfx_2");
        std::thread::sleep(Duration::from_millis(1000));

        player.play("sfx_1");
    }    
}
