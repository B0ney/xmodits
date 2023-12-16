use crate::sample::TrackerSample;

// TODO: Don't panic if there are no audio devices
pub struct SamplePlayer {
    _stream: rodio::OutputStream,
    _handle: rodio::OutputStreamHandle,
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

    pub fn is_playing(&self) -> bool {
        !self.sink.empty() && !self.sink.is_paused()
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume)
    }
}
