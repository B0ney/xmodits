use crate::core::audio::sample::TrackerSample;
use crate::core::audio::sample_pack::SamplePack;

use super::core::{AudioOutputDevice, Event, Frame, FrameModifier, PlayHandle};
use super::device;
use ringbuf;
use xmodits_lib::dsp::SampleBuffer;
use xmodits_lib::interface::Sample;

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
        let mut index: usize = 0;

        while index < self.handles.len() {
            let handle = &mut self.handles[index];

            let Some(next_frame) = handle.next() else {
                self.handles.swap_remove(index);
                continue;
            };

            frame[0] += next_frame[0];
            frame[1] += next_frame[1];

            index += 1;
        }

        self.device.write(&[frame.amplify(0.25).clamp()]);
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

    let mut file = std::io::Cursor::new(include_bytes!("../../../../test/modules/dnber.it"));

    let module = xmodits_lib::fmt::loader::load_module(&mut file).unwrap();
    let pack = SamplePack::from_module(&module);

    let samples: Vec<(Sample, TrackerSample)> =
        pack.samples.into_iter().filter_map(|f| f.ok()).collect();

    for chunk in samples.chunks(1) {
        for (metadata, sample) in chunk {
            dbg!(metadata);
            handle.send(Event::PushPlayHandle(Box::new(sample.clone())));
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    dbg!(&samples[5].0);
    let g = samples[5].1.clone();

    // Cool beat

    handle.send(Event::PushPlayHandle(Box::new(g.clone())));
    std::thread::sleep(std::time::Duration::from_millis(250));

    handle.send(Event::Clear);
    handle.send(Event::PushPlayHandle(Box::new(g.clone())));
    std::thread::sleep(std::time::Duration::from_millis(128));

    handle.send(Event::Clear);
    handle.send(Event::PushPlayHandle(Box::new(g.clone())));
    std::thread::sleep(std::time::Duration::from_millis(128));

    handle.send(Event::Clear);
    handle.send(Event::PushPlayHandle(Box::new(g.clone())));
    std::thread::sleep(std::time::Duration::from_millis(250));

    handle.send(Event::Clear);
    handle.send(Event::PushPlayHandle(Box::new(g.clone())));
    std::thread::sleep(std::time::Duration::from_millis(250));

    handle.send(Event::Clear);
    handle.send(Event::PushPlayHandle(Box::new(g.clone())));
    std::thread::sleep(std::time::Duration::from_millis(20000));
    dbg!(&g.buffer.loop_data);

    println!("Done!");
    // std::thread::sleep(std::time::Duration::from_millis(15000));
    // ha.join();
}
