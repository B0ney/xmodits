use std::path::Path;
use std::fs;
use byteorder::{BE,ByteOrder};
use crate::{offset_chars, offset_u32, offset_u16};
use crate::utils::{
    Error,
    wav,
    array::load_to_array
};

const MOD_SMP_START: usize = 0x0014;
const MOD_SMP_LEN: usize = 0x1e;        // Sample data is 30 bytes in size
// const MOD_SMP_PCM: uszie = ; // index where pcm data starts

pub struct MODFile {
    buf: Vec<u8>,
    title: [char; 20],
    smp_num: u8,
    smp_data: Vec<MODSample>,
    // pat_num: u8,
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
        /// find the higest value, add 1.
        /// 
        /// Use this (0x0438 + value * 1024) to skip pattern data
        /// which will lead us to the sample data
        /// 
        /// sample data is placed sequentially.

        Ok(Self {
            title,
            smp_data: build_samples(smp_num, &buf), 
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
            smp.smp_len,
            false,
        );

        let mut file    = File::create(path)?;
        file.write_all(&wav_header)?;


        Ok(())
    }
}

fn build_samples(smp_num: u8, buf: &[u8]) -> Vec<MODSample> {
    let mut smp_data: Vec<MODSample> = Vec::new();
    let smp_start_index: usize = MOD_SMP_START;
    // let mut smp_pcm_stream_index = ;

    for i in 0..smp_num as usize {
        let index = smp_start_index + (i * MOD_SMP_LEN); 
        let mut name: [char; 22] = [' '; 22];

        load_to_array(&mut name, &buf[offset_chars!(index, 22)]);
        smp_data.push(MODSample {
            name,
            length: BE::read_u16(&buf[offset_u16!(0x0016 + index)]) * 2, // Double to get size in bytes
        })
    }
    
    smp_data
}

pub struct MODSample {
    name: [char; 22],
    length: u16,    // multiply by 2 to get length in bytes
    // start_index: usize
}

#[test]
fn test1() {
    let protk = MODFile::load("samples/mod/omnibus.mod").unwrap();
    println!("no. allocated samples: {}\n\n", protk.smp_num);

    for i in protk.smp_data {
        println!("{}", i.length);
    }
}