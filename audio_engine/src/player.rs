use std::sync::Arc;
use crate::sample::TrackerSample;

pub struct SamplePlayer {
    _stream: rodio::OutputStream,
    _handle: rodio::OutputStreamHandle,
    pub sink: Arc<rodio::Sink>,
}

impl Default for SamplePlayer {
    fn default() -> Self {
        let (_stream, _handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Arc::new(rodio::Sink::try_new(&_handle).unwrap());
        Self {
            _stream,
            _handle,
            sink,
        }
    }
}

impl SamplePlayer {
    pub fn play(&self, source: TrackerSample) {
        self.sink.append(source);
    }
}


mod tests {
    use std::fs::File;

    use xmodits_lib::load_module;

    use crate::sample_pack::SamplePack;

    use super::{SamplePlayer};
    

    #[test]
    fn a() {
        let mut file = File::open("../test/HongKong_Music.umx").unwrap();
        let module = load_module(&mut file).unwrap();
        let player = SamplePlayer::default();
        let sample_pack = SamplePack::build(&*module);

        for (info, sample) in sample_pack.samples.iter().filter_map(|s| s.as_ref().ok()) {
            dbg!(info);
            player.play(sample.clone());
        }
        player.sink.sleep_until_end();

    }
}
