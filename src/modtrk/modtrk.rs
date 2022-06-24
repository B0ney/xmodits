use std::path::Path;
use std::fs::{self, File};
use std::io::Write;

use byteorder::{BE,ByteOrder};
use crate::{offset_chars, offset_u32, offset_u16};
use crate::utils::{
    Error,
    wav,
    array::load_to_array,signed::SignedByte
};

const MOD_SMP_START: usize = 0x0014;
const MOD_SMP_LEN: usize = 0x1e;        // Sample data is 30 bytes in size
const PAT_META: usize = 0x3b8;
pub struct MODFile {
    buf: Vec<u8>,
    title: [char; 20],
    smp_num: u8,
    smp_data: Vec<MODSample>,

}

impl MODFile {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
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
        // 20 + 30*31 + 1 + 1 + 128

        /// to find the number of patterns, 
        /// we select 128 bytes before offset 0x0438,
        /// find the highest value, add 1.
        /// 
        /// Use this (0x0438 + value * 1024) to skip pattern data
        /// which will lead us to the sample data
        /// 
        /// sample data is placed sequentially.
        let smp_index: usize = {
            0x0438 +
            (*buf[offset_chars!(PAT_META, 128)]
                .iter()
                .max()
                .unwrap() as usize + 1) * 1024 
        };


        Ok(Self {
            title,
            smp_data: build_samples(smp_num, &buf, smp_index), 
            smp_num,
            // pat_num: &buf[0x03b6],
            buf
        })
    }
    pub fn export<P: AsRef<Path>>(&self, path: P, index: usize) -> Result<(), Error> {
        // Now that we have enough infomation, we need to have a way to jump to every sample 
        // perhaps we should generate the indexes?
        let smp     = &self.smp_data[index];
        let wav_header = wav::build_header(
            8363,
            8,
            smp.length as u32,
            false,
        );

        let mut file    = File::create(path)?;
        file.write_all(&wav_header)?;
        let start = smp.index;
        let end: usize = start + smp.length as usize;
        println!("start: {}\nend: {}\n\n", start, end);

        let pcm = (&self.buf[start..end]).to_signed();
        file.write_all(&pcm)?;

        Ok(())
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

pub struct MODSample {
    name: [char; 22],
    length: u16,    // multiply by 2 to get length in bytes
    index: usize
}

#[test]
fn test1() {
    let protk = MODFile::load("samples/mod/wicked_time.mod").unwrap();
    println!("no. allocated samples: {}\n\n", protk.buf.len());

    for (index, i) in protk.smp_data.iter().enumerate().filter(|(_,e)| e.length != 0) {
        if let Err(e) = protk.export(format!("test/mod/{}.wav", index), index) {
            println!("{:?}", e);
        }
        // println!("index: {} size: {}", i.index, i.length);
    }
}