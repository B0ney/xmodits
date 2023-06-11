use crate::core::audio::sample::TrackerSample;

use super::core::{AudioOutputDevice, Event, Frame, PlayHandle, FrameModifier};
use super::device;
use ringbuf;
use xmodits_lib::dsp::{RawSample, SampleBuffer};

const BUFFER_SIZE_LATENCY: usize = 256;
const MAX_EVENTS_PER_TICK: usize = 32;

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Stop,
    Playing,
    Pause,
}

pub struct AudioEngine {
    device: Box<dyn AudioOutputDevice>,
    pub handles: Vec<Box<dyn PlayHandle>>,
    state: State,
    
    // ring_buffer: ringbuf::HeapRb<Frame>,
    // processing_buffer: Vec<Frame>,
}

impl AudioEngine {
    fn init(handle: AudioEngineHandle) -> Self {
        let device = Box::new(device::cpal::OutputDevice::start(handle.clone()));
        let buffer_size = device.rate() as usize * (BUFFER_SIZE_LATENCY / 1000);
        // let frames_per_tick = ;
        Self {
            device,
            handles: Vec::with_capacity(128),
            state: State::Stop,
            // ring_buffer: ringbuf::HeapRb::new(buffer_size),
        }
    }
    pub fn start() -> AudioEngineHandle {
        let (tx, rx) = std::sync::mpsc::channel::<Event>();
        let handle = AudioEngineHandle { tx };
        let audio_handle = handle.clone();

        std::thread::spawn(move || {
            let mut engine = AudioEngine::init(audio_handle);
            loop {
                for event in rx.try_iter().take(MAX_EVENTS_PER_TICK) {
                    engine.handle_event(event);
                }
                engine.tick();
            }
        });

        handle
    }

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

        self.device.write(&[frame.amplify(0.25).clamp()]);

        while let Some(i) = to_remove.pop() {
            self.handles.remove(i);
        }
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

#[derive(Clone)]
pub struct AudioEngineHandle {
    tx: std::sync::mpsc::Sender<Event>,
}

impl AudioEngineHandle {
    pub fn send(&self, event: Event) {
        self.tx.send(event);
    }
}


#[test]
fn a() {
    /*
        cargo test --release --package xmodits-gui --bin xmodits-gui -- core::audio::engine::a --exact --nocapture
     */

    let handle: AudioEngineHandle = AudioEngine::start();

    let mut file =
        std::io::Cursor::new(include_bytes!("../../../../test/modules/enigma.mod"));

    let module = xmodits_lib::fmt::loader::load_module(&mut file).unwrap();

    let samples: Vec<TrackerSample> = module
        .samples()
        .iter()
        .map(|sample| {
            let pcm = module.pcm(sample).unwrap();
            let mut sample_buffer = SampleBuffer::from(RawSample::from((sample, pcm)).into());
            xmodits_lib::dsp::resampler::resample(&mut sample_buffer, 48000);
            sample_buffer
        })
        .map(TrackerSample::new)
        .collect();

    // handle.send(Event::PushPlayHandle(Box::new(crate::core::audio::signal::Siren {
    //     sample_rate: 48000.0,
    //     high: 700.0,
    //     low: 400.0,
    //     frame: 0,
    //     rate: 5.0,
    //     switch: true,
    // })));

    // let mut a = samples[1].clone();
    // tx.send(Event::PushPlayHandle(Box::new(samples[1].clone())));

    for sample in samples {
        // for sample in samples.iter().cloned() {
            handle.send(Event::PushPlayHandle(Box::new(sample)));
        // }
        
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
    // let sample = sample.clone();
    // dbg!("{:?}",&sample.buffer.loop_data);
    // tx.send(Event::PushPlayHandle(Box::new(sample)));
    // std::thread::sleep(std::time::Duration::from_millis(1500));

    println!("Done!");
    // std::thread::sleep(std::time::Duration::from_millis(15000));
    // ha.join();
}
