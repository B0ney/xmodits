use std::path::PathBuf;
use crate::utils::prelude::*;
use byteorder::{ByteOrder, BE, LE};

const SMP_MASK_STEREO: u8 = 0b0000_0100;
const SMP_MASK_BITS: u8   = 0b0000_1000;

const INS_HEAD_LENGTH: usize = 13;
#[derive(Debug)]
pub struct S3MSample {
    smp_name: String,
    smp_ptr: u32,
    smp_len: u32,
    smp_stereo: bool,
    smp_rate: u32,
    smp_bits: u8,
}

pub struct S3MFile {
    buf: Vec<u8>,
    title: String,
    smp_data: Vec<S3MSample>,
}

use crate::interface::{TrackerDumper, DumperObject};

impl TrackerDumper for S3MFile {
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path> 
    {
        let buf = fs::read(path)?;
        // TODO: add checks to see if valid
        let title= string_from_chars(&buf[offset_chars!(0x0000, 28)]);

        // use the ins_ptrs stored to locate instument data
        let ord_count: u16 = LE::read_u16(&buf[offset_u16!(0x0020)]);
        let ins_count: u16 = LE::read_u16(&buf[offset_u16!(0x0022)]);
        // use this offset to locate list of instrument ptrs.
        let ins_ptr_list = 0x0060 + ord_count;

        let mut ins_ptrs: Vec<u16> = Vec::new();

        for i in 0..ins_count {
            let index = ins_ptr_list + (i * 2);
            // convert parameter to byte-level offset by << 4
            ins_ptrs.push(LE::read_u16(&buf[offset_u16!(index as usize)]) << 4)
        };

        Ok(Box::new(Self{
            title,
            smp_data: build_samples(&buf, ins_ptrs)?,
            buf,
        }))
    }

    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        let smp = &self.smp_data[index];
        if smp.smp_stereo {
            return Err("Stereo samples are not yet supported, please provide this module".into());
        }
        let start = smp.smp_ptr as usize;
        let end = start + smp.smp_len as usize;
        // TODO: figure out how to include stereo 
        let pcm = &self.buf[start..end]; // no need to convert to signed data
        let wav_header = wav::build_header(
            smp.smp_rate, smp.smp_bits, smp.smp_len, false /*smp.smp_stereo */,
        );
        let pathbuf: PathBuf = PathBuf::new()
            .join(path)
            .join(format!("({}) {}.wav",index,smp.smp_name));

        println!("{}", &pathbuf.display());
        let mut file: File = File::create(pathbuf)?;
        file.write_all(&wav_header)?;
        file.write_all(&pcm)?;

        Ok(())
    }

    fn number_of_samples(&self) -> usize {
        self.smp_data.len()
    }

    fn dump(&self) {
        todo!()
    }
}


fn build_samples(buf: &[u8], ins_ptr: Vec<u16>) -> Result<Vec<S3MSample>, Error> {
    let mut samples: Vec<S3MSample> = Vec::new();

    for i in ins_ptr {
        // if it's not a PCM instrument, skip
        if buf[i as usize] != 0x1 { continue; }
        let index: usize    = i as usize + INS_HEAD_LENGTH; // skip instrument header (13 bytes)
        // The sample pointer is a 24-bit integer
        // remember this is LE
        let hi_ptr: u8      = buf[index];   // smp ptr upper 8 bits 
        let lo_ptr: u16     = LE::read_u16(&buf[offset_u16!(0x0001 + index)]); // smp ptr lower 16 bits 
        let smp_ptr: u32    = (hi_ptr as u32) >> 16 | (lo_ptr as u32) << 4; 
        
        let smp_len: u32        = LE::read_u32(&buf[offset_u32!(0x0003 + index)]) as u32 & 0xffff;
        let smp_rate: u32       = LE::read_u32(&buf[offset_u32!(0x0013 + index)]) as u32;
        let smp_flag: u8        = buf[0x0012 + index];
        let smp_stereo: bool    = (smp_flag & SMP_MASK_STEREO) >> 2 == 1;
        let smp_bits: u8        = if (smp_flag & SMP_MASK_BITS) >> 3 == 1 { 16 } else { 8 };

        let smp_name= string_from_chars(&buf[offset_chars!(0x0023 + index, 28)]);

        samples.push(
            S3MSample{
                smp_name,
                smp_ptr,
                smp_len,
                smp_stereo,
                smp_rate,
                smp_bits,
            }
        )
    }

    Ok(samples)
}

#[test]
fn test1() {
    let a = S3MFile::load_module("samples/s3m/underwater_world_part_ii.s3m").unwrap();
    println!("{}", a.number_of_samples());
    for i in 0..a.number_of_samples() {
        if let Err(e) = a.export(&format!("test/s3m/"), i) {
            println!("{:?}", e);
        }
    }
    
}