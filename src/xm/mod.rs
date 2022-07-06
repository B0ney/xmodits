mod test;
mod deltadecode;
use byteorder::{ByteOrder, LE};
use crate::utils::prelude::*;

const XM_HEADER_ID: &str    = "Extended Module: ";
const XM_MAGIC_NUM: u8      = 0x1a;
const XM_MIN_VER: u16       = 0x0104;
const XM_SMP_BITS: u8       = 0b0001_0000;  // 1 = 16 bit samples

#[derive(Debug)]
pub struct XMSample {
    smp_len: usize,     // length of sample (in bytes?? )
    smp_name: String,
    smp_flags: u8,      
    smp_bits: u8,       // bits per sample
    smp_ptr: usize,
    smp_rate: i32,
}

pub struct XMFile {
    buf: Vec<u8>,
    tracker_name: String,   // Name of tracker software that made this module
    module_name: String,    // Name of tracker module
    samples: Vec<XMSample>,
}

use crate::interface::{TrackerDumper, TrackerModule};

impl TrackerDumper for XMFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized 
    {
        // Some checks to verify buffer is an XM module
        // 3 checks should be enough, anything more is redundant.
        if buf.len() < 60 
            || &buf[chars!(0x0000, 17)] != XM_HEADER_ID.as_bytes() 
            || buf[0x0025] != XM_MAGIC_NUM 
        {
            return Err("Not a valid XM file".into())
        }

        let version: u16 = LE::read_u16(&buf[word!(0x003A)]);

        if version < XM_MIN_VER {
            return Err("Unsupported XM version! (is below 0104)".into());
        }

        let module_name: String     = string_from_chars(&buf[chars!(0x0011, 20)]);
        let tracker_name: String    = string_from_chars(&buf[chars!(0x0026, 20)]);
        let patnum: u16             = LE::read_u16(&buf[word!(0x0046)]);
        let insnum: u16             = LE::read_u16(&buf[word!(0x0048)]);

        // Skip xm pattern headers so that we can access instrument headers.
        // Pattern headers do not have a fixed size so we need to calculate
        // their total size and add that to 0x0150
        let ins_header_offset: usize = skip_pat_header(&buf, patnum as usize);

        // with the offset given by ins_header_offset,
        // we can obtain infomation about each instrument
        // which may contain some samples
        let samples: Vec<XMSample> = build_samples(
            &buf,
            ins_header_offset,
            insnum as usize
        )?;
        println!("xm ins header: 0x{:04X}", ins_header_offset);
        println!("xm ins number: {}", insnum);
        println!("xm pat number: {}", patnum);

        for s in &samples {
            println!("{:?}", s);
        };

        Ok(Box::new(Self {
            tracker_name,
            module_name,
            buf,
            samples,
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
        pat_header_len  = LE::read_u32(&buf[dword!(0x0000 + offset)]); // should be 9
        pat_data_size   = LE::read_u16(&buf[word!(0x0007 + offset)]) as u32;
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
    let mut samples: Vec<XMSample> = Vec::new();
    let mut offset: usize = ins_header_offset;
    let mut header_size: u32;
    let mut ins_smp_num: u16;
    let mut smp_header_size: u32;

    for _ in 0..insnum {
        header_size = LE::read_u32(&buf[dword!(0x0000 + offset)]);
        ins_smp_num = LE::read_u16(&buf[word!(0x001b + offset)]);

        // If instrument has no samples,
        // move to next instrument header
        if ins_smp_num == 0 {
            offset += header_size as usize;
            continue;
        };
        // Obtain additional infomation from 
        // instrument header
        smp_header_size = LE::read_u32(&buf[dword!(0x001d + offset)]); // should be 40?
        
        offset += header_size as usize; // skip additional header to sample headers ()

        // (length, flag, name, finetune, relative note number)
        let mut smp_info: Vec<(u32, u8, String, i8, i8)> = Vec::new();

        // Sample header follows after additional header
        // When this loop completes, the offset will land at sample data
        for _ in 0..ins_smp_num {
            smp_info.push((
                LE::read_u32(&buf[dword!(0x0000 + offset)]),
                buf[0x000e + offset],
                string_from_chars(&buf[chars!(0x0012 + offset, 22)]),
                buf[0x000d+ offset] as i8,
                buf[0x0010+ offset] as i8,
            ));
            
            offset += smp_header_size as usize
        }
        
        for (
            smp_len,
            smp_flags,
            smp_name,
            finetune,
            notenum,
        ) in smp_info {
            let period: f32     = 7680.0 - ((48.0 + notenum as f32) * 64.0) - (finetune as f32 / 2.0);
            let smp_rate: i32   = (8363.0 * 2.0_f32.powf((4608.0 - period) / 768.0)) as i32;

            samples.push(XMSample{
                smp_bits: (((smp_flags & XM_SMP_BITS) >> 4) + 1) * 8,
                smp_len: smp_len as usize,
                smp_name,
                smp_flags,
                smp_ptr: offset,
                smp_rate
            });

            offset += smp_len as usize;
        }
    }

    Ok(samples)
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
    assert_eq!(b, -25);
    println!("{}", b);
}