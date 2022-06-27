mod test;
use crate::utils::prelude::*;
use byteorder::{BE, ByteOrder};

const MOD_SMP_START: usize = 0x0014;
const MOD_SMP_LEN: usize = 0x1e;
const PAT_META: usize = 0x3b8;

pub struct MODSample {
    name: String,
    length: u16,
    index: usize
}

pub struct MODFile {
    buf: Vec<u8>,
    title: String,
    smp_num: u8,
    smp_data: Vec<MODSample>,
}

use crate::{TrackerDumper, DumperObject};

impl TrackerDumper for MODFile {
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path> 
    {
        let buf: Vec<u8> = fs::read(path)?;
        let title: String = string_from_chars(&buf[offset_chars!(0x0000, 20)]);
        
        // keep in mind that sample field remains same size.
        let smp_num: u8 = { 
            // Valid ASCII chars are in between 32-127        
            if buf[offset_u32!(0x0438)].iter()
                .any(|b| *b >=32 && *b <= 126) 
            { 31 } else { 15 }
        };

        let largest_pat = *buf[offset_chars!(PAT_META, 128)]
                .iter()
                .max()
                .unwrap() as usize;

        let smp_index: usize = {
            0x0438 + (largest_pat + 1) * 1024 
        };

        Ok(Box::new(Self {
            title,
            smp_data: build_samples(smp_num, &buf, smp_index), 
            smp_num,
            buf
        }))
    }

    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        let smp: &MODSample     = &self.smp_data[index];
        let start: usize        = smp.index;
        let end: usize          = start + smp.length as usize;
        let pcm: Vec<u8>        = (&self.buf[start..end]).to_signed();
        let wav_header = wav::build_header(
            8363, 8, smp.length as u32, false,
        );
        let mut file: File      = File::create(path)?;

        file.write_all(&wav_header)?;
        file.write_all(&pcm)?;
        Ok(())
    }

    fn number_of_samples(&self) -> usize {
        todo!()
    }

    fn dump(&self) {
        todo!()
    }
}

fn build_samples(smp_num: u8, buf: &[u8], smp_start: usize) -> Vec<MODSample> {
    let mut smp_data: Vec<MODSample> = Vec::new();
    let mut smp_pcm_stream_index = smp_start;

    for i in 0..smp_num as usize {
        let offset = MOD_SMP_START + (i * MOD_SMP_LEN); 
        // Double to get size in bytes
        let len = BE::read_u16(&buf[offset_u16!(0x0016 + offset)]) * 2; 

        smp_data.push(MODSample {
            index: smp_pcm_stream_index,
            name: string_from_chars(&buf[offset_chars!(offset, 22)]),
            length: len, 
        });

        smp_pcm_stream_index += len as usize;
    }
    
    smp_data
}