use super::core::{AudioOutputDevice, DEFAULT_BUFFER_SIZE, DEFAULT_RATE};

pub struct Dummy;

impl AudioOutputDevice for Dummy {
    fn rate(&self) -> u32 {
        DEFAULT_RATE
    }
    fn reset(&mut self) {}
    fn write(&mut self, _chunk: &[[f32; 2]]) {}
}

pub mod cpal {
    use super::{AudioOutputDevice, DEFAULT_BUFFER_SIZE};

    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use rb::{RbConsumer, RbProducer, RB};

    pub struct OutputDevice {
        _device: cpal::Device,
        _stream: cpal::Stream,
        sample_rate: usize,
        buffer: rb::Producer<f32>,
    }

    impl AudioOutputDevice for OutputDevice {
        fn rate(&self) -> u32 {
            self.sample_rate as u32
        }

        fn reset(&mut self) {
            *self = Self::init();
        }

        fn write(&mut self, chunk: &[[f32; 2]]) {
            let _ = self.buffer.write_blocking_timeout(
                bytemuck::cast_slice(chunk),
                std::time::Duration::from_millis(64),
            );
        }
    }

    impl OutputDevice {
        pub fn init() -> Self {
            let host = cpal::default_host();

            let _device: cpal::Device = host
                .default_output_device()
                .expect("failed to find output device");

            let config = _device.default_output_config().unwrap();
            let buf_ms: usize = 128;
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
                            match rx.read_blocking(frame) {
                                Some(written) => {
                                    if written != frame.len() {
                                        // fill remaining buffer with silence
                                        write_silence(&mut frame[written..]);
                                    }
                                }

                                // Write silence if buffer is empty
                                None => write_silence(frame),
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
}
