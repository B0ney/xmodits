use crate::utils::prelude::*;
use byteorder::{ByteOrder, BE, LE};

pub struct S3MSample {
    smp_name: [char; 28],
    smp_ptr: u32,
    smp_len: u32,
    smp_stereo: bool,
    smp_bytes: u8,
}

pub struct S3MFile {
    buf: Vec<u8>,
    title: [char; 28],
    smp_data: Vec<S3MSample>,
}

use crate::interface::{TrackerDumper, DumperObject};

impl TrackerDumper for S3MFile {
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path> 
    {
        let buf = fs::read(path)?;
        // TODO: add checks to see if valid
        let mut title: [char; 28] = [' '; 28];
        load_to_array(&mut title, &buf[offset_chars!(0x0000, 28)]);
        
        /// use the ins_ptrs stored to locate instument data
        let ord_count: u16 = LE::read_u16(&buf[offset_u16!(0x0020)]);
        let ins_count: u16 = LE::read_u16(&buf[offset_u16!(0x0022)]);

        // use this offset to locate list of instrument ptrs.
        let ins_ptr_list = 0x0060 + ord_count;
        // println!("0x{:04X}\n", ins_ptr_list);
        
        let mut ins_ptrs: Vec<u16> = Vec::new();

        for i in 0..ins_count {
            let index = ins_ptr_list + (i * 2);
            // convert parameter to byte-level offset by << 4
            ins_ptrs.push(LE::read_u16(&buf[offset_u16!(index as usize)]) << 4)
        };

        for p in ins_ptrs {
            println!("0x{:04X}", p);
        }

        // Ok(Box::new(Self{
        //     title,
        //     smp_data: build_samples(&buf, ins_ptrs)?,
        //     buf,
        // }))
        Err("dont".into())
    }

    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        todo!()
    }

    fn number_of_samples(&self) -> usize {
        todo!()
    }

    fn dump(&self) {
        todo!()
    }
}


fn build_samples(buf: &[u8], ins_ptr: Vec<u16>) -> Result<Vec<S3MSample>, Error> {
    // let mut title: [char; 28] = [' '; 28];
    for i in ins_ptr {
        let i = usize;
        let smp_ptr: u32;

        // if it's not a PCM instrument, skip
        if &buf[i] != 0x1 { 
            continue;
        }
        
    }

    todo!()
}

#[test]
fn test1() {
    let a = S3MFile::load_module("samples/s3m/murallas.s3m");
}