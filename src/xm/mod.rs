mod deltadecode;
use crate::utils::prelude::*;
use deltadecode::{delta_decode_u16, delta_decode_u8};

const XM_HEADER_ID: &[u8; 17]       = b"Extended Module: ";
const MOD_PLUGIN_PACKED: &[u8; 20]  = b"MOD Plugin packed   ";
const XM_MAGIC_NUM: u8              = 0x1a;
const XM_MIN_VER: u16               = 0x0104;
const MASK_SMP_BITS: u8             = 0b0001_0000;  // 1 = 16 bit samples, 0 = 8 bit
const XM_SMP_SIZE: usize            = 40;
const XM_INS_SIZE: u32              = 263;

pub struct XMSample {
    smp_len: usize,     // length of sample in bytes
    smp_name: String,   // sample name
    _smp_flags: u8,     // sample bit flags
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
            || read_slice(&buf, 0x0000, 17) != XM_HEADER_ID 
            || buf[0x0025] != XM_MAGIC_NUM 
        {
            return Err("Not a valid XM file".into())
        }
        if read_slice(&buf, 0x0026, 20) == MOD_PLUGIN_PACKED {
            return Err("Unsupported XM: Uses 'MOD plugin packed'".into());
        }

        let version: u16 = read_u16_le(&buf, 0x003A);

        if version < XM_MIN_VER {
            return Err("Unsupported XM: Version below 0104".into());
        }

        Ok(())
    }

    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
    {
        Self::validate(&buf)?;

        let module_name: String         = read_string(&buf, 0x0011, 20);
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

    fn module_name(&self) -> &str {
        &self.module_name
    }
}

/// Skip xm pattern headers to access instrument headers.
/// Pattern headers do not have a fixed size so they need to be calculated.
fn skip_pat_header(buf: &[u8], patnum: usize, hdr_size: u32) -> Result<usize, Error> {
    let mut offset: usize = 60 + hdr_size as usize; // add 60 to go to pat header
    let mut pat_hdr_len: u32;
    let mut pat_data_size: u32;

    if patnum > 256 {
        return Err("Invalid XM: Contains more than 256 patterns".into())
    }

    for _ in 0..patnum {
        pat_hdr_len     = read_u32_le(buf, offset);
        pat_data_size   = read_u16_le(buf, 0x0007 + offset) as u32; 
        offset += (pat_hdr_len + pat_data_size) as usize;        
    }

    Ok(offset as usize)
}

/* Needs refactoring, it works but looks horrible. */
fn build_samples(buf: &[u8], ins_offset: usize, ins_num: usize) -> Result<Vec<XMSample>, Error> {
    let mut samples: Vec<XMSample>  = Vec::with_capacity(25); 
    let mut offset: usize           = ins_offset;
    let mut ins_hdr_size: u32;
    let mut ins_smp_num: u16;

    if ins_num > 128 {
        return Err("Invalid XM: Contains more than 128 instruments".into());
    }

    for _ in 0..ins_num {
        ins_hdr_size    = read_u32_le(buf, offset);
        ins_smp_num     = read_u16_le(buf, 0x001b + offset);

        if ins_hdr_size == 0 || ins_hdr_size > XM_INS_SIZE 
        {
            ins_hdr_size = XM_INS_SIZE;
        }
       
        offset += ins_hdr_size as usize; // skip to sample headers
        
        // Offset where the sample headers starts
        let start_smp_hdr: usize        = offset;
        let total_smp_hdr_size: usize   = XM_SMP_SIZE * ins_smp_num as usize;

        // Cumulative sum of sample data size
        let mut smp_len_cumulative: usize = 0;

        for _ in 0..ins_smp_num {
            let smp_len: usize = read_u32_le(buf, offset) as usize;

            // restart loop if length is 0
            if smp_len == 0 { offset += XM_SMP_SIZE; continue; }
            
            // break out of loop if offset leads to overflow panic
            if 0x0010 + offset > buf.len()
                || start_smp_hdr
                    + total_smp_hdr_size + smp_len > buf.len()
            {
                break;
            }

            let smp_ptr: usize      = start_smp_hdr + total_smp_hdr_size + smp_len_cumulative;
            let smp_name: String    = read_string(&buf, 0x0012 + offset,22);
            let smp_flags: u8       = buf[0x000e + offset];
            let smp_bits: u8        = (((smp_flags & MASK_SMP_BITS) >> 4) + 1) * 8;
            let finetune: i8        = buf[0x000d + offset] as i8;
            let notenum: i8         = buf[0x0010 + offset] as i8;

            let period: f32         = 7680.0 - ((48.0 + notenum as f32) * 64.0) - (finetune as f32 / 2.0);
            let smp_rate: u32       = (8363.0 * 2.0_f32.powf((4608.0 - period) / 768.0)) as u32;
            
            samples.push(XMSample{
                smp_len, 
                smp_name,
                _smp_flags: smp_flags,
                smp_bits,
                smp_ptr,
                smp_rate
            });

            smp_len_cumulative += smp_len;
            offset += XM_SMP_SIZE;
        }

        offset += smp_len_cumulative;
    }

    Ok(samples)
}