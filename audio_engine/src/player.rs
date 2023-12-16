use crate::sample::TrackerSample;

// TODO: Don't panic if there are no audio devices
pub struct SamplePlayer {
    _stream: rodio::OutputStream,
    _handle: rodio::OutputStreamHandle,
    // pub sink: Arc<rodio::Sink>,
}

impl Default for SamplePlayer {
    fn default() -> Self {
        let (_stream, _handle) = rodio::OutputStream::try_default().unwrap();

        Self { _stream, _handle }
    }
}

impl SamplePlayer {
    pub fn create_handle(&self) -> PlayerHandle {
        PlayerHandle {
            sink: rodio::Sink::try_new(&self._handle).unwrap(),
        }
    }
}

pub struct PlayerHandle {
    sink: rodio::Sink,
}

impl PlayerHandle {
    pub fn play(&self, source: TrackerSample) {
        self.sink.append(source);
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn pause(&self) {
        match self.sink.is_paused() {
            true => self.sink.play(),
            false => self.sink.pause(),
        }
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume)
    }
}

mod tests {
    use std::fs::File;
    use std::sync::Arc;

    use xmodits_lib::load_module;

    use crate::sample_pack::SamplePack;

    use super::SamplePlayer;

    #[test]
    fn a() {
        let mut file = File::open("../test/HongKong_Music.umx").unwrap();
        let module = load_module(&mut file).unwrap();
        let player = Arc::new(SamplePlayer::default());
        let sample_pack = Arc::new(SamplePack::build(&*module));

        let handle = player.clone().create_handle();
        let handle2 = player.create_handle();
        let a = sample_pack.clone();
        let b = sample_pack.clone();

        let t1 = std::thread::spawn(move || {
            for (info, sample) in a.samples.iter().filter_map(|s| s.as_ref().ok()) {
                handle.play(sample.clone());
                handle.sink.sleep_until_end();
            }
        });

        let t2 = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(2));
            for (info, sample) in b.clone().samples.iter().filter_map(|s| s.as_ref().ok()) {
                handle2.play(sample.clone());
                handle2.sink.sleep_until_end();
            }
        });

        t1.join();
        t2.join();
    }
}
