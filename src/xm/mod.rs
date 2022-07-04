mod test;
mod deltadecode;
use byteorder::{ByteOrder, LE};
use crate::utils::prelude::*;

const XM_HEADER_ID: &str    = "Extended Module: ";
const XM_MAGIC_NUM: u8      = 0x1a;
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
        // Some checks to verify buffer is an XM module
        // 3 checks should be enough, anything more is redundant.
        if buf.len() < 60 
            || &buf[offset_chars!(0x0000, 17)] != XM_HEADER_ID.as_bytes() 
            || buf[0x0025] != XM_MAGIC_NUM 
        {
            return Err("Not a valid XM file".into())
        }

        let version: u16 = LE::read_u16(&buf[offset_u16!(0x003A)]);

        if version < XM_MIN_VER {
            return Err("Unsupported XM version! (is below 0104)".into());
        }

        let module_name: String     = string_from_chars(&buf[offset_chars!(0x0011, 20)]);
        let tracker_name: String    = string_from_chars(&buf[offset_chars!(0x0026, 20)]);
        let patnum: u16             = LE::read_u16(&buf[offset_u16!(0x0046)]);
        let insnum: u16             = LE::read_u16(&buf[offset_u16!(0x0048)]);

        // Skip xm pattern headers so that we can access instrument headers.
        // Pattern headers do not have a fixed size so we need to calculate
        // their total size and add that to 0x0150
        let ins_header_offset: usize = skip_pat_header(&buf, patnum as usize);

        // with the offset given by ins_header_offset,
        // we can obtain infomation about each instrument
        // which may contain some samples
        let xmsamples: Vec<XMSample> = build_samples(
            &buf,
            ins_header_offset,
            insnum as usize
        )?;


        println!("xm ins header: 0x{:04X}", ins_header_offset);
        println!("xm ins number: {}", insnum);
        println!("xm pat number: {}", patnum);



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
/// Skip pattern data by adding their sizes and 
/// returning the offset where next data starts
/// which is the xm instrument headers.
fn skip_pat_header(buf: &[u8], patnum: usize) -> usize {
    let mut offset: usize = 0x0150;
    let mut pat_header_len: u32;
    let mut pat_data_size: u32;

    for _ in 0..patnum {
        pat_header_len  = LE::read_u32(&buf[offset_u32!(0x0000 + offset)]); // should be 9
        pat_data_size   = LE::read_u16(&buf[offset_u16!(0x0007 + offset)]) as u32;
        assert_eq!(pat_header_len, 9, "header len is not 9?");
        offset += (pat_header_len + pat_data_size) as usize; 
    }

    offset as usize
}

fn build_samples(
    buf: &[u8],
    ins_header_offset: usize,
    insnum: usize
) -> Result<Vec<XMSample>, Error> {
    Err("".into())
    // todo!()
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
fn gen_offset2(){
    let offset = [
        4,96,48,48,
        1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,
        2,2
    ];
    let mut a = 29;

    for i in offset {
        println!("0x{:04X} => ", a);
        a += i;
    }

}
#[test]
fn gen_offset3(){
    let offset = [
        4, 4,4,
        1,1,1,
        1,1,1,
        22,
    ];
    let mut a = 0;

    for i in offset {
        println!("0x{:04X} => ", a);
        a += i;
    }

}
#[test]
fn test_2() {
    let xm = XMFile::load_module("samples/xm/an-path.xm").unwrap();
    println!("{}", xm.module_name());
    
}

#[test]
fn test_3() {
    let a:u8 = 0xE7;
    let b = a as i8;// casting u8 to i8 works as intended
    assert!(b, -25);
    println!("{}", b);
}