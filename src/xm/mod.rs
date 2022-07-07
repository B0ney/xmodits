mod test;
mod deltadecode;
use crate::utils::prelude::*;
use deltadecode::{delta_decode_u16, delta_decode_u8};

const XM_HEADER_ID: &str    = "Extended Module: ";
const XM_MAGIC_NUM: u8      = 0x1a;
const XM_MIN_VER: u16       = 0x0104;
const XM_SMP_BITS: u8       = 0b0001_0000;  // 1 = 16 bit samples

#[derive(Debug)]
pub struct XMSample {
    smp_len: u32,     // length of sample (in bytes?? )
    smp_name: String,
    smp_flags: u8,      
    smp_bits: u8,       // bits per sample
    smp_ptr: usize,
    smp_rate: u32,
}

pub struct XMFile {
    buf: Vec<u8>,
    tracker_name: String,   // Name of tracker software that made this module
    module_name: String,    // Name of tracker module
    samples: Vec<XMSample>,
    smp_num: usize,
}

use crate::interface::{TrackerDumper, TrackerModule};
use crate::utils::reader::read_u32_le;

impl TrackerDumper for XMFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized 
    {
        // Some checks to verify buffer is an XM module
        // 3 checks should be enough, anything more is redundant.
        if buf.len() < 60 
            || read_chars(&buf, 0x0000, 17) != XM_HEADER_ID.as_bytes() 
            || buf[0x0025] != XM_MAGIC_NUM 
        {
            return Err("Not a valid XM file".into())
        }

        let version: u16 = read_u16_le(&buf, 0x003A);

        if version < XM_MIN_VER {
            return Err("Unsupported XM version! (is below 0104)".into());
        }

        let module_name: String     = string_from_chars(&buf[chars!(0x0011, 20)]);
        let tracker_name: String    = string_from_chars(&buf[chars!(0x0026, 20)]);

        let patnum: u16             = read_u16_le(&buf, 0x0046);
        let insnum: u16             = read_u16_le(&buf, 0x0048);

        // Skip xm pattern headers so that we can access instrument headers.
        // Pattern headers do not have a fixed size so we need to calculate them.
        let ins_header_offset: usize = skip_pat_header(&buf, patnum as usize);

        // given by ins_header_offset, obtain infomation about each instrument
        // which may contain some samples
        let samples: Vec<XMSample> = build_samples(
            &buf, ins_header_offset, insnum as usize)?;

        let smp_num: usize = samples.len();

        Ok(Box::new(Self {
            tracker_name,
            module_name,
            buf,
            samples,
            smp_num,
        }))
    }

    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        if !folder.as_ref().is_dir() {
            return Err("Path is not a folder".into());
        }
        let smp: &XMSample          = &self.samples[index];
        let wav_header: [u8; 44]    = wav::build_header(
            smp.smp_rate, smp.smp_bits,
            smp.smp_len, false,
        );
        let start_ptr: usize    = smp.smp_ptr as usize;
        let end_ptr: usize      = start_ptr + smp.smp_len as usize;
        let path: PathBuf       = PathBuf::new()
            .join(folder)
            .join(name_sample(index, &smp.smp_name));
        let mut file: File      = File::create(path)?;

        file.write_all(&wav_header)?;
        // We need to delta decode them, but it's fine for now
        match smp.smp_bits {
            16 => { 
                let deltad: Vec<u8> = delta_decode_u16(&self.buf[start_ptr..end_ptr]);
                file.write_all(&deltad)?; 
            }
            8 => {  
                let deltad: Vec<u8> = delta_decode_u8(&self.buf[start_ptr..end_ptr]).to_signed();
                file.write_all(&deltad)?;
            }
            e => return Err(format!("Why is it {} bits per sample?",e).into())
        }
        
        Ok(())
    }

    fn number_of_samples(&self) -> usize {
        self.smp_num
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
        pat_header_len  = read_u32_le(buf, offset); // should be 9
        pat_data_size   = read_u16_le(buf, 0x0007 + offset) as u32;
        offset += (pat_header_len + pat_data_size) as usize; 
    }
    offset as usize
}

// Needs refactoring, it works..
// but looks horrible
fn build_samples(
    buf: &[u8],
    ins_header_offset: usize,
    insnum: usize
) -> Result<Vec<XMSample>, Error> {
    let mut samples: Vec<XMSample> = Vec::new();
    let mut offset: usize = ins_header_offset;
    let mut ins_header_size: u32;
    let mut ins_smp_num: u16;
    let mut smp_header_size: u32;

    for _ in 0..insnum {
        ins_header_size = read_u32_le(buf, offset);
        ins_smp_num     = read_u16_le(buf, 0x001b + offset);
        
        // If instrument has no samples,
        // move to next instrument header
        if ins_smp_num == 0 {
            offset += ins_header_size as usize;
            continue;
        };
        // Obtain additional infomation from 
        // instrument header
        smp_header_size = read_u32_le(buf, 0x001d + offset); // should be 40?
        
        offset += ins_header_size as usize; // skip to sample headers

        // (length, flag, name, finetune, relative note number)
        let mut smp_info: Vec<(u32, u8, String, i8, i8)> = Vec::new();

        // Sample header follows after additional header
        // When this loop completes, the offset will land at sample data
        for _ in 0..ins_smp_num {
            smp_info.push((
                read_u32_le(buf, offset),
                buf[0x000e + offset],

                string_from_chars(
                    &buf[chars!(0x0012 + offset, 22)]
                ),
                buf[0x000d + offset] as i8,
                buf[0x0010 + offset] as i8,
            ));

            offset += smp_header_size as usize
        }
        // TODO: ignore if instrument uses AMIGA frequency table
        
        for (
            smp_len,
            smp_flags,
            smp_name,
            finetune,
            notenum,
        ) in smp_info {
            if smp_len == 0 { continue; }

            let period: f32     = 7680.0 - ((48.0 + notenum as f32) * 64.0) - (finetune as f32 / 2.0);
            let smp_rate: u32   = (8363.0 * 2.0_f32.powf((4608.0 - period) / 768.0)) as u32;

            samples.push(XMSample{
                smp_bits: (((smp_flags & XM_SMP_BITS) >> 4) + 1) * 8,
                smp_len,
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
    let xm = XMFile::load_module("samples/xm/xo-sat.xm").unwrap();
    println!("{}", xm.module_name());
    println!("{}", xm.number_of_samples());
    

    
}

#[test]
fn test_3() {
    let a:u8 = 0xE7;
    let b = a as i8;// casting u8 to i8 works as intended
    assert_eq!(b, -25);
    println!("{}", b);
}