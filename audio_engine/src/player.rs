use std::time::Instant;

use crate::sample::{FramesIter, TrackerSample};

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
        self.sink.append(FramesIter {
            sample: source,
            timer: Instant::now(),
            callback: None,
        });
    }

    pub fn play_with_callback<F>(&self, source: TrackerSample, callback: F)
    where
        F: Fn(&TrackerSample, &mut Instant) + Send + 'static,
    {
        self.sink.append(FramesIter {
            sample: source,
            timer: Instant::now(),
            callback: Some(Box::new(callback)),
        });
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
