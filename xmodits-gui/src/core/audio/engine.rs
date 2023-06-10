use crate::core::audio::sample::TrackerSample;

use super::core::{AudioOutputDevice, Event, Frame, PlayHandle};
use super::device;
use ringbuf;
use xmodits_lib::dsp::{RawSample, SampleBuffer};

const BUFFER_SIZE_LATENCY: usize = 256;

pub struct AudioEngine {
    device: Box<dyn AudioOutputDevice>,
    pub handles: Vec<Box<dyn PlayHandle>>,
    state: State,
    // ring_buffer: ringbuf::HeapRb<Frame>,
    // processing_buffer: Vec<Frame>,
}

impl AudioEngine {
    pub fn init() -> Self {
        let device = Box::new(device::cpal::OutputDevice::init());
        let buffer_size = device.rate() as usize * (BUFFER_SIZE_LATENCY / 1000);
        // let frames_per_tick = ;
        Self {
            device,
            handles: Vec::with_capacity(128),
            state: State::Stop,
            // ring_buffer: ringbuf::HeapRb::new(buffer_size),
        }
    }
    pub fn start() {}
    pub fn tick(&mut self) {
        let mut frame: Frame = [0.0, 0.0];
        let mut to_remove: Vec<usize> = Vec::new();

        for (index, handle) in self.handles.iter_mut().enumerate() {
            let Some(next_frame) = handle.next() else {
                to_remove.push(index);
                continue;
            };

            frame[0] += next_frame[0];
            frame[1] += next_frame[1];
        }

        while let Some(i) = to_remove.pop() {
            self.handles.remove(i);
        }

        self.device.write(&[clamp(frame)]);
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::RequestAudioDeviceReset => self.device.reset(),
            Event::PushPlayHandle(play_handle) => self.handles.push(play_handle),
            Event::PlayEvent(state) => self.state = state,
            Event::Clear => self.handles.clear(),
        }
    }
}

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Stop,
    Playing,
    Pause,
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub frame: u64,
    pub frequency: f32,
    pub duration: usize,
}

impl Oscillator {
    fn next_sample(&mut self) {
        self.frame = (self.frame + 1) % self.sample_rate as u64;
    }

    fn sine(&mut self) -> f32 {
        self.next_sample();
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.frame as f32 * self.frequency * two_pi / self.sample_rate).sin()
    }
}

impl PlayHandle for Oscillator {
    fn next(&mut self) -> Option<[f32; 2]> {
        let s = self.sine();
        Some([s, s])
    }

    fn reset(&mut self) {}

    fn jump(&mut self, _tick: usize) {}
}

pub struct Siren {
    sample_rate: f32,
    high: f32,
    low: f32,
    frame: u64,
    rate: f32,
    switch: bool,
}

impl Siren {
    fn next_sample(&mut self) {
        self.frame = (self.frame + 1) % self.sample_rate as u64;
    }
    fn sine(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.frame as f32 * freq * two_pi / self.sample_rate).sin()
    }

    fn next(&mut self) -> f32 {
        self.next_sample();
        if self.frame % (self.sample_rate / self.rate).floor() as u64 == 0 {
            self.switch = !self.switch;
        }

        match self.switch {
            true => self.sine(self.high),
            false => self.sine(self.low),
        }
    }
}

impl PlayHandle for Siren {
    fn next(&mut self) -> Option<[f32; 2]> {
        // if self.duration > 44100 {
        //     return None;
        // }
        let s = self.next();
        // self.duration += 1;
        // dbg!(s);
        Some([s, s])
    }

    fn reset(&mut self) {}

    fn jump(&mut self, _tick: usize) {}
}

#[test]
fn a() {
    let (tx, rx) = std::sync::mpsc::channel::<Event>();
    const MAX_EVENTS: usize = 32;

    let ha = std::thread::spawn(move || {
        let mut engine = AudioEngine::init();
        loop {
            for event in rx.try_iter().take(MAX_EVENTS) {
                engine.handle_event(event);
            }
            engine.tick();
        }
    });

    let mut file =
        std::io::Cursor::new(include_bytes!("../../../../test/modules/delamour_edit.it"));

    let module = xmodits_lib::fmt::loader::load_module(&mut file).unwrap();
    let smp = &module.samples()[0];
    let pcm = module.pcm(&smp).unwrap();
    let mut sample = SampleBuffer::from(RawSample::from((smp, pcm)).into());
    xmodits_lib::dsp::resampler::resample(&mut sample, 48000);
    let sample = TrackerSample::new(sample);
    // let samples: Vec<TrackerSample> = module
    //     .samples()
    //     .iter()
    //     .map(|sample| {
    //         let pcm = module.pcm(sample).unwrap();
    //         let mut sample = SampleBuffer::from(RawSample::from((sample, pcm)).into());
    //         xmodits_lib::dsp::resampler::resample(&mut sample, 48000);
    //         sample
    //     })
    //     .map(TrackerSample::new)
    //     .collect();

    // tx.send(Event::PushPlayHandle(Box::new(Siren {
    //     sample_rate: 48000.0,
    //     high: 700.0,
    //     low: 400.0,
    //     frame: 0,
    //     rate: 5.0,
    //     switch: true,
    // })));

    // for sample in samples {
    //     tx.send(Event::PushPlayHandle(Box::new(sample)));
    //     std::thread::sleep(std::time::Duration::from_millis(1500));
    // }
    let sample = sample.clone();
    dbg!("{:?}",&sample.buffer.loop_data);
    tx.send(Event::PushPlayHandle(Box::new(sample)));
    std::thread::sleep(std::time::Duration::from_millis(1500));

    println!("Done!");
    // std::thread::sleep(std::time::Duration::from_millis(1500));
    ha.join();
}

fn clamp(mut frame: [f32; 2]) -> [f32; 2] {
    frame
        .iter_mut()
        .for_each(|smp| {
            if *smp < -1.0 {
                *smp = -1.0;
                return;
            }
            if *smp > 1.0 {
                *smp = 1.0;
                return;
            }
        });
    frame
}
