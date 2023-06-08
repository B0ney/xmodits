use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use xmodits_lib::dsp::sample::FramesIter;
use xmodits_lib::dsp::{resampler::resample, RawSample, SampleBuffer};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rb::{RbConsumer, RbProducer, RB};

const DEFAULT_RATE: u32 = 44100;
const DEFAULT_BUFFER_SIZE: usize = 256;

pub enum Event {
    RequestAudioDeviceReset,
}

pub trait AudioOutputDevice {
    fn rate(&self) -> u32;
    fn reset(&mut self);
    fn write(&mut self, chunk: &[[f32; 2]]);
}

pub struct Dummy;

impl AudioOutputDevice for Dummy {
    fn rate(&self) -> u32 {
        DEFAULT_RATE
    }

    fn reset(&mut self) {}
    fn write(&mut self, _chunk: &[[f32; 2]]) {}
}

pub struct CpalOutputDevice {
    _device: cpal::Device,
    _stream: cpal::Stream,
    /// sample rate of the output device.
    sample_rate: usize,
    buffer: rb::Producer<f32>,
}

impl AudioOutputDevice for CpalOutputDevice {
    fn rate(&self) -> u32 {
        self.sample_rate as u32
    }

    fn reset(&mut self) {
        *self = Self::init();
    }

    fn write(&mut self, chunk: &[[f32; 2]]) {
        let _ = self.buffer.write_blocking(bytemuck::cast_slice(chunk));
    }
}

impl CpalOutputDevice {
    pub fn init() -> Self {
        let host = cpal::default_host();

        let _device: cpal::Device = host
            .default_output_device()
            .expect("failed to find output device");

        let config = _device.default_output_config().unwrap();
        let buf_ms: usize = DEFAULT_BUFFER_SIZE;
        let channels = config.channels() as usize;
        let sample_rate = config.sample_rate().0 as usize;
        let buffer_size = ((sample_rate * channels) as f32 * (buf_ms as f32 / 1000.0)) as usize;

        dbg!(buffer_size);

        let rb = rb::SpscRb::<f32>::new(buffer_size);
        let (tx, rx) = (rb.producer(), rb.consumer());

        let write_silence = |data: &mut [f32]| data.iter_mut().for_each(|f| *f = 0.0);

        let _stream: cpal::Stream = _device
            .build_output_stream(
                &config.into(),
                // todo: use read_blocking instead?
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                        match rx.read(frame) {
                            // Some(_) => {},
                            // None => {},
                            Ok(written) => {
                                // if written != frame.len() {
                                //     // fill remaining buffer with silence
                                //     write_silence(&mut frame[written..]);
                                // }
                            }

                            // Write silence if buffer is empty
                            Err(_) => (),
                            //  write_silence(frame),
                        }
                    }
                },
                |e| println!("{e}"),
                None,
            )
            .unwrap();

        // start the stream
        _stream.play().unwrap();

        Self {
            _device,
            _stream,
            buffer: tx,
            sample_rate,
        }
    }

}

pub struct SampleInfo {
    name: String,
    filename: Option<String>,
    buffer: Arc<SampleBuffer>,
}

pub struct AudioDevice {
    out: Box<dyn AudioOutputDevice>,
    cache: RwLock<HashMap<String, Weak<SampleBuffer>>>,
}

pub struct SampleList {}
fn a() {
    for frame in FramesIter::new(&SampleBuffer::default()) {

    }
}
