/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::path::Path;
use std::{fs::File, io::Write};

use crate::TrackerSample;

const RIFF: [u8; 4] = [0x52, 0x49, 0x46, 0x46]; // RIFF
const WAVE: [u8; 4] = [0x57, 0x41, 0x56, 0x45]; // WAVE
const FMT_: [u8; 4] = [0x66, 0x6D, 0x74, 0x20]; // "riff "
const DATA: [u8; 4] = [0x64, 0x61, 0x74, 0x61]; // data
const SMPL: [u8; 4] = [0x73, 0x6D, 0x70, 0x6C]; // smpl
const HEADER_SIZE: u32 = 44;
pub struct Wav {
    stereo: bool, // is pcm stereo)
    is_interleaved: bool,
    header: WavHeader,
    // chunks: Vec
}

pub struct WavHeader {
    file_size: [u8; 4],
    wav_scs: [u8; 4],
    wav_type: [u8; 2],
    channels: [u8; 2],
    sample_frequency: [u8; 4],
    bytes_sec: [u8; 4],
    block_align: [u8; 2],
    bits_sample: [u8; 2],
    size_of_chunk: [u8; 4],

}

impl Wav {
    pub fn from_tracker_sample(smp: &TrackerSample) -> Self {
        Self::header(smp.rate, smp.bits, smp.len as u32, smp.is_stereo, false)

        // todo!()
    }

    pub fn header(
        smp_rate: u32, // sample rate
        smp_bits: u8,  // bits per sample
        pcm_len: u32,  // length of pcm data in BYTES
        stereo: bool,  // is pcm stereo)#
        is_interleaved: bool,
    ) -> Self {
        let channels: u16 = stereo as u16 + 1; // 0x01 = mono, 0x02 = stereo
        let block_align: u16 = channels * (smp_bits / 8) as u16;
        Self {
            stereo,
            is_interleaved,
            header: WavHeader {
                file_size: (HEADER_SIZE - 8 + pcm_len * channels as u32).to_le_bytes(),
                wav_scs: 16_u32.to_le_bytes(),
                wav_type: 1_u16.to_le_bytes(),
                channels: (stereo as u16 + 1).to_le_bytes(),
                sample_frequency: smp_rate.to_le_bytes(),
                bytes_sec: (smp_rate * block_align as u32).to_le_bytes(),
                block_align: block_align.to_le_bytes(),
                bits_sample: (smp_bits as u16).to_le_bytes(),
                size_of_chunk: (pcm_len * channels as u32).to_le_bytes(),
            },
        }
    }

    pub fn write_ref<P: AsRef<Path>>(&self, path: P, pcm: &[u8], with_loop_points: bool) -> std::io::Result<()> {
        let mut file: File = File::create(path)?;
        let hdr = &self.header;
        let header: [&[u8]; 13] = [
            &RIFF,
            &hdr.file_size,
            &WAVE,
            &FMT_,
            &hdr.wav_scs,
            &hdr.wav_type,
            &hdr.channels,
            &hdr.sample_frequency,
            &hdr.bytes_sec,
            &hdr.block_align,
            &hdr.bits_sample,
            &DATA,
            &hdr.size_of_chunk,
        ];
        for i in header {
            file.write_all(i)?;
        }

        match (self.stereo, self.is_interleaved) {
            // (true, false) => write_interleaved(file, pcm, self.smp_bits),
            (_, _) => file.write_all(pcm)?,
        };

        if with_loop_points {
            
        };

        Ok(())
    }
}

struct SampleChunk {
    sample_loops: Vec<Loop>
}

struct Loop {
    id: u32,
    kind: LoopType,
    start: u32,
    end: u32,
    fraction: u32,
    repeats: u32,
}
enum LoopType {
    Forward = 0,
    PingPong = 1,
    Reverse = 2,
}

// https://www.recordingblogs.com/wiki/sample-chunk-of-a-wave-file
impl SampleChunk {
    fn write(&self, wave: &mut File) {
        let mut data: Vec<u8> = Vec::new();
        
        wave.write_all(&SMPL);
        
    }
}

#[test]
fn a() {
    let a = LoopType::Forward as u32;
    dbg!(a);
}