mod test;
mod deltadecode;
use byteorder::{ByteOrder, LE};
use crate::utils::prelude::*;

const XM_HEADER_ID: &str    = "Extended Module: ";
const XM_MIN_VER: u16       = 0x0104;
const XM_SMP_BITS: u8       = 0b0000_1000;  // 1 = 16 bit samples
pub struct XMSample {
    smp_len: usize,     // length of sample (in bytes?? )
    smp_name: String,
    smp_flags: u8,      
    smp_bits: u8,       // bits per sample
}

pub struct XMFile {
    buf: Vec<u8>,
    tracker_name: String,   // Name of tracker software that made this module
    module_name: String,    // Name of tracker module
}

use crate::interface::{TrackerDumper, TrackerModule};

impl TrackerDumper for XMFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized 
    {
        if buf.len() < 17 // for now
            || &buf[offset_chars!(0x0000, 17)] != XM_HEADER_ID.as_bytes() 
        {
            return Err("Is not an extended module".into())
        }
        let version: u16 = LE::read_u16(&buf[offset_u16!(0x003A)]);

        if version < XM_MIN_VER {
            return Err("XM version is below 0x0104... Which means it's not supported.".into());
        }

        let tracker_name: String    = string_from_chars(&buf[offset_chars!(0x0026, 20)]);
        let module_name: String     = string_from_chars(&buf[offset_chars!(0x0011, 20)]);


        Ok(Box::new(Self {
            tracker_name,
            module_name,
            buf,
        }))
    }

    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        todo!()
    }

    fn number_of_samples(&self) -> usize {
        todo!()
    }

    fn module_name(&self) -> &String {
        &self.module_name
    }
}

fn build_samples() -> Result<XMSample, Error> {
    todo!()
}

#[test]
fn gen_offset(){
    let offset = [
        0, 17, 37, 38, 58,
        60
    ];
    let offset2 = [
        4, 5,6,10,12,14,16,18,20,0
    ];

    for i in offset {
        println!("0x{:04X} => ", i);
    }
    for i in offset2 {
        println!("0x{:04X} => ", i + 60);
    }
}

#[test]
fn test_2() {
    let xm = XMFile::load_module("samples/xm/SHADOW.XM").unwrap();
    println!("{}", xm.module_name());
    
}