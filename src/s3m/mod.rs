use std::path::PathBuf;
use crate::utils::prelude::*;

const S3M_HEADER_ID: &str       = "SCRM";
const S3M_MAGIC_NUMBER: u8      = 0x10;
const SMP_MASK_STEREO: u8       = 0b0000_0100;
const SMP_MASK_BITS: u8         = 0b0000_1000;
const INS_HEAD_LENGTH: usize    = 13;

pub struct S3MSample {
    smp_name: String,
    smp_ptr: u32,       // Sample pointer
    smp_len: u32,       // Length of sample 
    smp_stereo: bool,   // Is sample stereo
    smp_rate: u32,      // Sample rate
    smp_bits: u8,       // Bits per sample
}

pub struct S3MFile {
    buf: Vec<u8>,
    title: String,
    smp_data: Vec<S3MSample>,
}

use crate::interface::{TrackerDumper, TrackerModule};

impl TrackerDumper for S3MFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error> 
        where Self: Sized
    {
        if buf.len() < 0x0060
            || buf[0x001d] != S3M_MAGIC_NUMBER
            || read_chars(&buf, 0x002c, 4) != S3M_HEADER_ID.as_bytes()
        {
            return Err("File is not a valid Scream Tracker Module".into());
        }

        let title: String       = string_from_chars(&buf[chars!(0x0000, 28)]);
        let ord_count: u16      = read_u16_le(&buf, 0x0020);
        let ins_count: u16      = read_u16_le(&buf, 0x0022);
        let ins_ptr_list: u16   = 0x0060 + ord_count;

        let mut ins_ptrs: Vec<usize> = Vec::with_capacity(ins_count as usize);

        for i in 0..ins_count {
            let index: u16 = ins_ptr_list + (i * 2);
            // convert parameter to byte-level offset by << 4
            // cast to usize to avoid potential overflow
            ins_ptrs.push((read_u16_le(&buf, index as usize) as usize) << 4)
        };

        let smp_data: Vec<S3MSample> = build_samples(&buf, ins_ptrs);

        Ok(Box::new(Self{
            title,
            smp_data,
            buf,
        }))
    }

    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        if !folder.as_ref().is_dir() {
            return Err("Path is not a folder".into());
        }
        let smp: &S3MSample         = &self.smp_data[index];
        let start: usize            = smp.smp_ptr as usize;
        let end: usize              = start + (smp.smp_len * (smp.smp_stereo as u32 + 1)) as usize;
        let path: PathBuf           = PathBuf::new()
            .join(folder)
            .join(name_sample(index, &smp.smp_name));

        WAV::header(smp.smp_rate, smp.smp_bits, smp.smp_len, smp.smp_stereo)
            .write_ref(path, &self.buf[start..end])
    }

    fn number_of_samples(&self) -> usize {
        self.smp_data.len()
    }
    
    fn module_name(&self) -> &String {
        &self.title
    }
}

fn build_samples(buf: &[u8], ins_ptr: Vec<usize>) -> Vec<S3MSample> {
    let mut samples: Vec<S3MSample> = Vec::with_capacity(ins_ptr.len());

    for i in ins_ptr {
        if buf[i as usize] != 0x1 { continue; }             // if it's not a PCM instrument, skip
        let index: usize        = i + INS_HEAD_LENGTH;      // skip instrument header (13 bytes)
        let smp_len: u32        = read_u32_le(buf, 0x0003 + index) & 0xffff;

        if smp_len == 0 { continue; }

        let hi_ptr: u8          = buf[index];
        let lo_ptr: u16         = read_u16_le(buf, 0x0001 + index);
        let smp_ptr: u32        = (hi_ptr as u32) >> 16 | (lo_ptr as u32) << 4;
        let smp_flag: u8        = buf[0x0012 + index];
        // let smp_stereo: bool    = (smp_flag & SMP_MASK_STEREO) >> 2 == 1;
        let smp_stereo: bool    = false;

        if (smp_ptr + smp_len) > buf.len() as u32 { break; } // break out of loop if we get a funky offset

        let smp_name: String    = string_from_chars(&buf[chars!(0x0023 + index, 28)]);
        let smp_rate: u32       = read_u32_le(buf, 0x0013 + index);
        let smp_bits: u8        = if (smp_flag & SMP_MASK_BITS) >> 3 == 1 { 16 } else { 8 };

        samples.push(S3MSample {
            smp_name,
            smp_len,
            smp_rate,
            smp_bits,
            smp_stereo,   
            smp_ptr
        })
    }

    samples
}