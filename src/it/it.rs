use byteorder::{ByteOrder, LE, BE};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::utils::{Error, SignedByte};

use crate::wav;
use crate::{offset_u16, offset_u32, offset_chars};

use super::compression::{self, decompress_sample};

const IT_HEADER_ID: u32 = 0x49_4D_50_4D; // IMPM
const IT_SAMPLE_ID: u32 = 0x49_4D_50_53; // IMPS
const IT_HEADER_LEN: usize = 192;
const IT_SAMPLE_LEN: usize = 80;
const MASK_SMP_BITS: u8 = 0b0000_0010; // 16/8bit samples
const MASK_SMP_COMP: u8 = 0b0000_1000; // Does sample use compression?

#[derive(Debug)]
pub struct ItSample {
    pub filename: [char; 12],
    pub name: [char; 26],
    pub smp_len: u32,        // This is NOT in bytes
    pub smp_ptr: u32,
    pub smp_rate: u32,
    pub smp_flag: u8,
    pub smp_bits: u8,       // does sample use 16 or 8 bits
    pub smp_comp: bool, // Does sample use compression
}

#[derive(Debug)]
pub struct ItFile {
    buffer: Vec<u8>,
    pub sample_number: u16,
    pub samples_meta: Vec<ItSample>,
}

impl ItFile {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let buffer: Vec<u8> = fs::read(path)?;

        if buffer.len() < IT_HEADER_LEN
            || BE::read_u32(&buffer[offset_u32!(0x0000)]) != IT_HEADER_ID
        {
            return Err("File is not a valid Impulse Tracker Module".into());
        };

        let sample_number = LE::read_u16(&buffer[offset_u16!(0x0024)]);
        let samples_meta = build_samples(&buffer, sample_number)?;

        Ok(Self {
            sample_number,
            samples_meta,
            buffer,
        })
    }

    pub fn export<P: AsRef<Path>>(&self, path: P, index: usize) -> Result<(), Error> {
        let smp     = &self.samples_meta[index];
        let start_ptr   = smp.smp_ptr as usize;
        let end_ptr     = start_ptr + 
            (smp.smp_len * (smp.smp_bits as u32 / 8)) as usize;
        let mut file    = File::create(path)?;
        let wav_header = wav::build_header(
            smp.smp_rate,
            smp.smp_bits,
            smp.smp_len,
        );
        
        // Write Wav Header
        file.write_all(&wav_header)?;

        // Write PCM data
        if smp.smp_comp {
            let decomp = decompress_sample(
                &self.buffer[start_ptr..], smp.smp_len, smp.smp_bits, false
            )?;
            file.write_all(&decomp)?;

        } else {
            let mut raw_data = &self.buffer[start_ptr..end_ptr];
            let mut b: Vec<u8> = Vec::new();

            let end_ptr = start_ptr + 
                (smp.smp_len * (smp.smp_bits as u32 / 8)) as usize;
            
            // convert sample data to "signed" values if it's 8-bit  
            if smp.smp_bits == 8 {
                b = raw_data.to_signed(); 
                raw_data = &b; // make raw data reference b instead
            }

            file.write_all(&raw_data)?;
        }

        Ok(())
    }
}

fn build_samples(it_data: &Vec<u8>, num_samples: u16) -> Result<Vec<ItSample>, Error> {
    let mut ins_start_index: usize = 0;
    let mut smp_meta: Vec<ItSample> = Vec::new();

    if num_samples == 0 {
        return Err("IT module doesn't contain any samples.".into());
    }

    for i in 0..(it_data.len() - 4) { // 4 is the amount of bytes a u32 takes up. Prevents panic.
        if BE::read_u32(&it_data[offset_u32!(i)]) == IT_SAMPLE_ID {
            ins_start_index = i;
            break;
        }
    }

    if ins_start_index == 0 {
        return Err(format!(
            "IT module doesn't contain any samples. Despite showing that it has \"{}\" samples",
            num_samples
        ).into());
    }

    for i in 0..num_samples as usize {
        let offset = ins_start_index + (i * IT_SAMPLE_LEN) as usize;
        let smp_flag: u8 = it_data[0x012 + offset];
        let mut filename:   [char; 12] = [' '; 12];
        let mut name:       [char; 26] = [' '; 26];      
        
        load_to_array(&mut filename,    &it_data[offset_chars!(0x0004 + offset, 12)]);
        load_to_array(&mut name,        &it_data[offset_chars!(0x0014 + offset, 26)]);
        
        /*  Isolate flag bit to LSB, by ANDing with MASK then Shift Right N. (LSB = Least Significant Bit)
            
            for "bit_samples", Add 1 and multiply by 8 so that
            if bit = 0, 0 becomes 8.  If bit = 1, 1 becomes 16 
        */
        let bits_sample: u8         = (((smp_flag & MASK_SMP_BITS) >> 1) +  1) * 8;
        let is_compressed: bool     =  ((smp_flag & MASK_SMP_COMP) >> 3) == 1;

        smp_meta.push(ItSample {
            filename,
            name,
            smp_len:    LE::read_u32(&it_data[offset_u32!(0x0030 + offset)]),
            smp_ptr:    LE::read_u32(&it_data[offset_u32!(0x0048 + offset)]),
            smp_rate:   LE::read_u32(&it_data[offset_u32!(0x003C + offset)]),
            smp_flag,
            smp_bits: bits_sample,
            smp_comp: is_compressed,
        })
    }

    Ok(smp_meta)
}

// maybe use generics?
// move to utils
fn load_to_array(array: &mut [char], data: &[u8]) {
    assert!(array.len() <= data.len());

    for i in 0..array.len() {
        array[i] = data[i] as char;
    }
}
