use crate::{dword, word, Error};
// use byteorder::{ByteOrder, BE, LE};
use std::path::Path;
use std::{fs::File, io::Write};

const RIFF: [u8; 4] = [0x52, 0x49, 0x46, 0x46]; // RIFF
const WAVE: [u8; 4] = [0x57, 0x41, 0x56, 0x45]; // WAVE
const FMT_: [u8; 4] = [0x66, 0x6D, 0x74, 0x20]; // "riff "
const DATA: [u8; 4] = [0x64, 0x61, 0x74, 0x61]; // data
const HEADER_SIZE: u32 = 44;

pub struct Wav {
    smp_rate: u32, // sample rate
    smp_bits: u8,  // bits per sample
    pcm_len: u32,  // length of byte array
    stereo: bool,  // is pcm stereo)
    is_interleaved: bool,
    header: WavHeader,
}

pub struct WavHeader {
    file_size: [u8;4],
    wav_scs: [u8;4],
    wav_type: [u8;2],
    channels: [u8;2],
    sample_frequency: [u8;4],
    bytes_sec: [u8;4],
    block_align: [u8;2],
    bits_sample: [u8;2],
    size_of_chunk: [u8;4],
}

impl Wav {
    pub fn header(
        smp_rate: u32, // sample rate
        smp_bits: u8,  // bits per sample
        pcm_len: u32,  // length of pcm data in BYTES
        stereo: bool,  // is pcm stereo)#
        is_interleaved: bool,
    ) -> Self {
        let channels: u16 = (stereo as u16 + 1); // 0x01 = mono, 0x02 = stereo
        let block_align: u16 = (channels * (smp_bits / 8) as u16);
        Self {
            smp_rate,
            smp_bits,
            pcm_len,
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
                size_of_chunk: (pcm_len * channels as u32).to_le_bytes()
            },
        }
    }

    pub fn write<P: AsRef<Path>>(&self, path: P, pcm: Vec<u8>) -> Result<(), Error> {
        self.write_ref(path, &pcm)
    }

    pub fn write_ref<P: AsRef<Path>>(&self, path: P, pcm: &[u8]) -> Result<(), Error> {
        let mut file: File = File::create(path)?;
        let hdr = &self.header;
        let header: [&[u8]; 13] = [
            &RIFF, &hdr.file_size, &WAVE, &FMT_, &hdr.wav_scs, &hdr.wav_type, 
            &hdr.channels, &hdr.sample_frequency,&hdr.bytes_sec, 
            &hdr.block_align, &hdr.bits_sample, &DATA, &hdr.size_of_chunk
        ];
        for i in header {
            file.write_all(i)?;
        }

        match (self.stereo, self.is_interleaved) {
            (true, false) => write_interleaved(file, pcm, self.smp_bits),
            (_, _) => file.write_all(pcm).map_err(|e| e.into()),
        }
    }
}

/// Is there a way to do this without making the program x100 slower?
///
/// s3m already interleaves stereo data, right?
fn write_interleaved(mut _file: File, _pcm: &[u8], _smp_bits: u8) -> Result<(), Error> {
    _file.write_all(_pcm).map_err(|e| e.into())
}
