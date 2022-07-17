mod test;
mod deltadecode;
use crate::utils::prelude::*;
use deltadecode::{delta_decode_u16, delta_decode_u8};

const XM_HEADER_ID: &str    = "Extended Module: ";
const XM_MAGIC_NUM: u8      = 0x1a;
const XM_MIN_VER: u16       = 0x0104;
const XM_SMP_BITS: u8       = 0b0001_0000;  // 1 = 16 bit samples, 0 = 8 bit
const XM_SMP_SIZE: usize    = 40;
const XM_FLG_FRQ_TABLE: u16 = 0x1;  // 0th bit
const XM_INS_SIZE: u32 = 263;

pub struct XMSample {
    smp_len: usize,     // length of sample in bytes??
    smp_name: String,   // sample name
    _smp_flags: u8,      // sample bit flags
    smp_bits: u8,       // bits per sample
    smp_ptr: usize,     // offset to sample data
    smp_rate: u32,      // sample sampling rate
}

pub struct XMFile {
    buf: Vec<u8>,
    module_name: String,    // Name of tracker module
    smp_data: Vec<XMSample>,
    smp_num: usize,
}

use crate::interface::{TrackerDumper, TrackerModule};

impl TrackerDumper for XMFile {
    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < 60 
            || read_chars(&buf, 0x0000, 17) != XM_HEADER_ID.as_bytes() 
            || buf[0x0025] != XM_MAGIC_NUM 
        {   // 3 checks should be enough, anything more is redundant.
            return Err("Not a valid XM file".into())
        }
        let version: u16 = read_u16_le(&buf, 0x003A);
        if version < XM_MIN_VER {
            return Err("Unsupported XM version! (is below 0104)".into());
        }
        let uses_amiga_table: bool = (read_u16_le(&buf, 0x004a) & XM_FLG_FRQ_TABLE) == 0;
        if uses_amiga_table  {
            /*  If we ignore this and treat AMIGA FREQUENCY as LINEAR FREQUENCY: 
                * The sampling frquency will be correct, but the waveform wouldn't.
                * Its waveform appears to have its amplitudes constrained to 0.5,-0.5
                rather than 1,-1.
                
                * The waveform when looked at in detail, will have slight differences 
                compared to a waveform dumped by schism (ignoring the first buggy sample points).
            
                refer to https://github.com/Artefact2/libxm/blob/master/src/play.c
            */
            
            return Err("Unsupported XM file. It use the 'AMIGA FREQUENCY TABLE' for its sample data.".into()); 
        }
        Ok(())
    }

    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized 
    {
        Self::validate(&buf)?;

        let module_name: String         = string_from_chars(&buf[chars!(0x0011, 20)]);
        let header_size: u32            = read_u32_le(&buf, 0x003c);
        let patnum: u16                 = read_u16_le(&buf, 0x0046);
        let insnum: u16                 = read_u16_le(&buf, 0x0048);
        let ins_offset: usize           = skip_pat_header(&buf, patnum as usize, header_size)?;
        let samples: Vec<XMSample>      = build_samples(
            &buf, ins_offset, insnum as usize
        )?;
        let smp_num: usize = samples.len();

        Ok(Box::new(Self {
            module_name,
            buf,
            smp_data: samples,
            smp_num,
        }))
    }

    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        if !folder.as_ref().is_dir() {
            return Err("Path is not a folder".into());
        }

        let smp: &XMSample          = &self.smp_data[index];
        let start: usize            = smp.smp_ptr;
        let end: usize              = start + smp.smp_len as usize;
        let path: PathBuf           = PathBuf::new()
            .join(folder)
            .join(name_sample(index, &smp.smp_name));

        WAV::header(smp.smp_rate, smp.smp_bits, smp.smp_len as u32, false)
            .write(
                path,
                match smp.smp_bits {
                    8   => { delta_decode_u8(&self.buf[start..end]).to_signed() }
                    _   => { delta_decode_u16(&self.buf[start..end]) }
                }
            )
    }

    fn number_of_samples(&self) -> usize {
        self.smp_num
    }

    fn module_name(&self) -> &String {
        &self.module_name
    }
}

/// Skip xm pattern headers so that we can access instrument headers.
/// Pattern headers do not have a fixed size so we need to calculate them.
fn skip_pat_header(buf: &[u8], patnum: usize, header_size: u32) -> Result<usize, Error> {
    let mut offset: usize = 60 + header_size as usize; // add 60 to go to pat header
    let mut pat_header_len: u32;
    let mut pat_data_size: u32;
    let mut pat_pak_type: u8;

    for _ in 0..patnum {
        pat_pak_type = buf[0x0004 + offset];
        pat_header_len  = read_u32_le(buf, offset);
        pat_data_size   = read_u16_le(buf, 0x0007 + offset) as u32;
        
        offset += (pat_header_len + pat_data_size) as usize;        
    }

    Ok(offset as usize)
}

/* Needs refactoring, it works but looks horrible. */
fn build_samples(buf: &[u8], ins_offset: usize, ins_num: usize) -> Result<Vec<XMSample>, Error> {
    let mut samples: Vec<XMSample>  = Vec::with_capacity(25); 
    let mut offset: usize           = ins_offset;
    let mut ins_header_size: u32;
    let mut ins_smp_num: u16;

    for _ in 0..ins_num {
        ins_header_size = read_u32_le(buf, offset);
        ins_smp_num     = read_u16_le(buf, 0x001b + offset);

        if ins_header_size == 0 
            || ins_header_size > XM_INS_SIZE 
        {
            ins_header_size = XM_INS_SIZE;
        }
       
        offset += ins_header_size as usize; // skip to sample headers
        
        // (length, flag, name, finetune, relative note number)
        let mut smp_info: Vec<(usize, u8, String, i8, i8)> = Vec::new();

        // Sample header follows after additional header
        // When this loop completes, the offset will land at sample data
        for _ in 0..ins_smp_num {
            smp_info.push((
                read_u32_le(buf, offset) as usize,
                buf[0x000e + offset],
                string_from_chars(&buf[chars!(0x0012 + offset, 22)]),
                buf[0x000d + offset] as i8,
                buf[0x0010 + offset] as i8,
            ));
            
            offset += XM_SMP_SIZE;
        }

        for (smp_len, smp_flags,
            smp_name, finetune, notenum) in smp_info 
        {
            if smp_len == 0 { continue; }
            if (offset + smp_len) > buf.len() { break; }

            let period: f32     = 7680.0 - ((48.0 + notenum as f32) * 64.0) - (finetune as f32 / 2.0);
            let smp_rate: u32   = (8363.0 * 2.0_f32.powf((4608.0 - period) / 768.0)) as u32;
            let smp_bits: u8    = (((smp_flags & XM_SMP_BITS) >> 4) + 1) * 8;

            samples.push(XMSample{
                smp_bits, 
                smp_len,
                smp_name,
                _smp_flags: smp_flags,
                smp_ptr: offset,
                smp_rate
            });

            offset += smp_len as usize;
        }
    }

    Ok(samples)
}

// pub fn peroid_linear(note: f32) -> f32 {
//     6.0
// }

// pub fn peroid_amiga(note: f32) -> f32 {
//     6.0
// }

// pub fn frequency_linear(period: f32) -> f32 {

// }

// pub fn frequency_amiga(period: f32) -> f32 {
    
// }