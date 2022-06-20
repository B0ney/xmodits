use byteorder::{LittleEndian, ByteOrder,LE, BE};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::utils::Error;
use crate::wav;
use crate::{offset_u16, offset_u32, offset_chars};

const IT_HEADER_ID: u32 = 0x49_4D_50_4D; // IMPM
const IT_SAMPLE_ID: u32 = 0x49_4D_50_53; // IMPS
const IT_HEADER_LEN: usize = 192;
const IT_SAMPLE_LEN: usize = 80;
const MASK_BITS_SAMPLE: u8 = 0b0000_0011;

#[derive(Debug)]
pub struct ItSample {
    filename: [char; 12],
    name: [char; 26],
    length: u32,
    sample_pointer: u32,
    sample_rate: u32,
    flags: u8,
    bits_sample: u16,
}

#[derive(Debug)]
pub struct ItFile {
    buffer: Vec<u8>,
    pub sample_number: u16,
    samples_meta: Vec<ItSample>,
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
            buffer,
            sample_number,
            samples_meta,
        })
    }

    pub fn export<P: AsRef<Path>>(&self, path: P, index: usize) -> Result<(), Error> {
        let smp = &self.samples_meta[index];
        let wav_header = wav::build_header(
            smp.sample_rate,
            smp.bits_sample,
            smp.length,
        );
        
        let start_ptr = smp.sample_pointer as usize;

        let end_ptr = start_ptr + 
            (smp.length * (smp.bits_sample as u32 / 8)) as usize;

        let raw = &self.buffer[start_ptr..end_ptr];

        let mut file = File::create(path)?;

        file.write_all(&wav_header)?;

        // Write PCM data
        if smp.bits_sample == 8 {
            // normalize to prevent earrape
            let a = raw.iter().map(|e| e.wrapping_sub(128)).collect::<Vec<u8>>();
            file.write_all(&a)?;
        } else {
            file.write_all(&raw)?;
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
        )
        .into());
    }

    for i in 0..num_samples as usize{
        let offset = ins_start_index + (i * IT_SAMPLE_LEN) as usize;
        let mut filename:   [char; 12] = [' '; 12];
        let mut name:       [char; 26] = [' '; 26];

        load_to_array(&mut filename, &it_data[offset_chars!(0x0004 + offset, 12)]);
        load_to_array(&mut name, &it_data[offset_chars!(0x0014 + offset, 26)]);
        
        let bits_sample = match it_data[0x012 + offset] & MASK_BITS_SAMPLE {
            0b11 => 16, // 16 bit samples
            0b01 => 8, // 8- bit samples
            f => {
                println!("warning, got flag {:02b}, defaulting to 8 bits per sample", f);
                16
            },
        };

        smp_meta.push(ItSample {
            filename,
            name,
            length:             LE::read_u32(&it_data[offset_u32!(0x0030 + offset)]),
            sample_pointer:     LE::read_u32(&it_data[offset_u32!(0x0048 + offset)]),
            sample_rate:        LE::read_u32(&it_data[offset_u32!(0x003C + offset)]),
            flags:              it_data[0x012 + offset],
            bits_sample
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
