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

pub struct MODFile {
    buf: Vec<u8>,
    title: [char; 20],
    smp_num: u8,
    smp_data: Vec<MODSample>
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

        Ok(Self{
            title,
            smp_data: build_samples(smp_num, &buf), 
            smp_num,
            buf
            
        })
    }
    pub fn export<P: AsRef<Path>>(&self, path: P, index: usize) -> Result<(), Error> {

        Ok(())
    }
}

fn build_samples(smp_num: u8, buf: &[u8]) -> Vec<MODSample> {
    let mut smp_data: Vec<MODSample> = Vec::new();
    let smp_start_index: usize = MOD_SMP_START;

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
}

#[test]
fn test1() {
    let protk = MODFile::load("samples/mod/omnibus.mod").unwrap();
    println!("no. allocated samples: {}\n\n", protk.smp_num);

    for i in protk.smp_data {
        println!("{}", i.length);
    }
}