use std::{fs::File, io::Write};
use std::path::Path;
use byteorder::{ByteOrder, BE, LE};
use crate::{word, dword, Error};

const RIFF: u32 = 0x5249_4646; // RIFF
const WAVE: u32 = 0x5741_5645; // WAVE
const FMT_: u32 = 0x666D_7420; // "riff "
const DATA: u32 = 0x6461_7461; // data
const HEADER_SIZE: usize = 44;

pub struct Wav {
    smp_rate: u32,  // sample rate
    smp_bits: u8,   // bits per sample
    pcm_len: u32,   // length of byte array
    stereo: bool,   // is pcm stereo)
    header_data: [u8; HEADER_SIZE],
}

impl Wav {
    pub fn header(
        smp_rate: u32,  // sample rate
        smp_bits: u8,   // bits per sample
        pcm_len: u32,   // length of pcm data in BYTES
        stereo: bool    // is pcm stereo)#
    ) -> Self {
        let mut header:[u8; HEADER_SIZE] = [0u8; HEADER_SIZE];     
        let wav_scs:            u32 = 16;                       // sec chunk size
        let wav_type:           u16 = 1;                        // 1 = pcm
        let channels:           u16 = stereo as u16 + 1;        // 0x01 = mono, 0x02 = stereo
        let sample_frequency:   u32 = smp_rate /*/ channels as u32*/;
        let block_align:        u16 = channels * (smp_bits / 8) as u16;
        let bytes_sec:          u32 = smp_rate * block_align as u32;
        let bits_sample:        u16 = smp_bits as u16;
        let file_size:          u32 = HEADER_SIZE as u32 - 8 + pcm_len * channels as u32;
        let size_of_chunk:      u32 = pcm_len * channels  as u32;
    
        BE::write_u32(&mut header[dword!(0x0000)], RIFF);
        LE::write_u32(&mut header[dword!(0x0004)], file_size);
        BE::write_u32(&mut header[dword!(0x0008)], WAVE);
        BE::write_u32(&mut header[dword!(0x000C)], FMT_);
        LE::write_u32(&mut header[dword!(0x0010)], wav_scs);
        LE::write_u16(&mut header[word!(0x0014)], wav_type);
        LE::write_u16(&mut header[word!(0x0016)], channels);
        LE::write_u32(&mut header[dword!(0x0018)], sample_frequency);
        LE::write_u32(&mut header[dword!(0x001C)], bytes_sec);
        LE::write_u16(&mut header[word!(0x0020)], block_align);
        LE::write_u16(&mut header[word!(0x0022)], bits_sample);
        BE::write_u32(&mut header[dword!(0x0024)], DATA);
        LE::write_u32(&mut header[dword!(0x0028)], size_of_chunk);

        Self { smp_rate, smp_bits, pcm_len, stereo, header_data: header }
    }

    pub fn write<P: AsRef<Path>>(&self, path: P, pcm: Vec<u8>) -> Result<(), Error> {
        self.write_ref(path, &pcm)
    }
    pub fn write_ref<P: AsRef<Path>>(&self, path: P, pcm: &[u8]) -> Result<(), Error> { 
        let mut file: File = File::create(path)?;
        file.write_all(&self.header_data)?;

        match self.stereo {
            true    => { write_interleaved(file, pcm, self.smp_bits) },
            false   => { file.write_all(pcm).map_err(|e| e.into()) }
        }
    }
}

/// Is there a way to do this without making the program x100 slower?
/// 
/// s3m already interleaves stereo data, right?
fn write_interleaved(mut _file: File, _pcm: &[u8], _smp_bits: u8) -> Result<(), Error> {
    _file.write_all(_pcm).map_err(|e| e.into())
    // Ok(())
    // return Err("Writing stereo data is not yet supported".into());

    // Slowest thing in the universe
    // Don't use this crap
    
    // let mut switch: bool = true;
    // let smp_bytes: usize = (_smp_bits / 8) as usize;
    // let offset: usize = _pcm.len() / 2;

    // for i in 0..(_pcm.len() / 2) {
    //     if (i * 2) % (2 * smp_bytes) == 0 {
    //         switch = !switch
    //     }
    //     _file.write(&[_pcm[i + (offset * switch as usize)]])?;
    // }
    // Ok(())
}