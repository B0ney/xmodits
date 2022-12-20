// use once_cell::sync::Lazy;
// use rodio::{source::Buffered, Decoder, Source};
// use std::collections::HashMap;
// use std::io::Cursor;

// type SoundBuffer = Buffered<Decoder<Cursor<&'static [u8]>>>;

// const SFX_1: &[u8] = include_bytes!("../../res/sfx/sfx_1.wav");
// const SFX_2: &[u8] = include_bytes!("../../res/sfx/sfx_2.wav");

// pub static SFX: Lazy<HashMap<&'static str, SoundBuffer>> = Lazy::new(|| {
//     let sfx: &[(&str, &[u8])] = &[
//         ("sfx_1", SFX_1),
//         ("sfx_2", SFX_2),
//         // ("sfx_3", SFX_3),
//         // ("sfx_4", SFX_4),
//     ];

//     sfx.iter()
//         .map(|(x, y)| (*x, Decoder::new(Cursor::new(*y)).unwrap().buffered()))
//         .collect()
// });

// pub struct Audio {
//     _stream: rodio::OutputStream,
//     handle: rodio::OutputStreamHandle,
//     sink: Option<rodio::Sink>,
// }

// impl Default for Audio {
//     fn default() -> Self {
//         Self::init()
//     }
// }

// impl Audio {
//     pub fn init() -> Self {
//         let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
//         let mut a = Self {
//             _stream,
//             handle,
//             sink: None,
//         };
//         a.sink = Some(rodio::Sink::try_new(&a.handle).unwrap());
//         a
//     }

//     pub fn play(&self, sfx: &str) {
//         if let Some(sound) = SFX.get(sfx) {
//             self.sink.as_ref().unwrap().append(sound.clone());
//         }
//     }
// }
