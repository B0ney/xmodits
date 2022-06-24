use std::path::Path;
use std::fs::{self, File};
use std::io::Write;
use byteorder::{BE, ByteOrder};
use crate::utils::prelude::*;

const MOD_SMP_START: usize = 0x0014;
const MOD_SMP_LEN: usize = 0x1e;        // Sample data is 30 bytes in size
const PAT_META: usize = 0x3b8;

pub struct MODSample {
    name: [char; 22],
    length: u16,    // multiply by 2 to get length in bytes
    index: usize
}
pub struct MODFile {
    buf: Vec<u8>,
    title: [char; 20],
    smp_num: u8,
    smp_data: Vec<MODSample>,
}

use crate::{TrackerDumper, DumperObject};

impl TrackerDumper for MODFile {
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path> 
    {
        let buf: Vec<u8> = fs::read(path)?;
        let mut title: [char; 20] = [' '; 20];
        load_to_array(&mut title, &buf[offset_chars!(0x0000, 20)]);
        
        // keep in mind that sample field remains same size.
        let smp_num: u8 = { 
            // Valid ASCII chars are in between 32-127        
            if buf[offset_u32!(0x0438)].iter()
                .any(|b| *b >=32 && *b <= 126) 
            { 31 } else { 15 }
        };

        let smp_index: usize = {
            0x0438 +
            (*buf[offset_chars!(PAT_META, 128)]
                .iter()
                .max()
                .unwrap() as usize + 1) * 1024 
        };

        Ok(Box::new(Self {
            title,
            smp_data: build_samples(smp_num, &buf, smp_index), 
            smp_num,
            buf
        }))
    }

    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        let mut file = File::create(path)?;
        let smp     = &self.smp_data[index];
        let start = smp.index;
        let end: usize = start + smp.length as usize;
        let pcm = (&self.buf[start..end]).to_signed();
        let wav_header = wav::build_header(
            8363, 8, smp.length as u32, false,
        );

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
    let smp_start_index: usize = MOD_SMP_START;
    let mut smp_pcm_stream_index = smp_start;
    // generate sample index here

    for i in 0..smp_num as usize {
        let index = smp_start_index + (i * MOD_SMP_LEN); 
        let len = BE::read_u16(&buf[offset_u16!(0x0016 + index)]) * 2; // Double to get size in bytes
        let mut name: [char; 22] = [' '; 22];

        load_to_array(&mut name, &buf[offset_chars!(index, 22)]);
        smp_data.push(MODSample {
            index: smp_pcm_stream_index,
            name,
            length: len, 
        });

        smp_pcm_stream_index += len as usize;
    }
    
    smp_data
}
