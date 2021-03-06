use crate::utils::prelude::*;
use crate::{dword, word};
mod tables;
use tables::FINETUNE_TABLE;
use byteorder::{BE, ByteOrder};

const MOD_XPK_MAGIC: u32        = 0x50503230; // PP20
const MOD_SMP_START: usize      = 0x0014; // offset where title ends & smp data begins
const MOD_SMP_LEN: usize        = 0x1e;
const PAT_META: usize           = 0x3b8;
const ALT_FINETUNE: [&[u8]; 2]  = [b"M&K!", b"FEST"];

pub struct MODSample {
    name: String,
    length: u16,
    index: usize,
    freq: u16,
}

pub struct MODFile {
    buf: Vec<u8>,
    title: String,
    smp_num: u8,
    smp_data: Vec<MODSample>,
}

use crate::{TrackerDumper, TrackerModule};

/// I need to work on "MOD Format.md" before I continue working on this. 
impl TrackerDumper for MODFile {
    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < 1085 {
            return Err("Not a valid MOD file".into());
        }
        if BE::read_u32(&buf[dword!(0x0000)]) == MOD_XPK_MAGIC {
            return Err("XPK compressed MOD files are not supported".into());
        }
        Ok(())
    }
    
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error> 
    {
        Self::validate(&buf)?;

        let title: String       = string_from_chars(&buf[chars!(0x0000, 20)]);
        let alt_finetune: bool  = ALT_FINETUNE.contains(&&buf[dword!(0x0438)]);

        // if it contains any non-ascii, it was probably made with ultimate sound tracker
        let smp_num: u8 = { 
            // Valid ASCII chars are in between 32-127        
            if buf[dword!(0x0438)].iter()
                .any(|b| *b <=32 || *b >= 126) 
            { 15 } else { 31 }
        };     
          
        // Fixed panic on modules made with ultimate sound tracker.
        // ^ outdated
        // TODO: Why did I add 1? I fogor
        let offset: usize = if smp_num == 15 { (15+1) * 30 } else { 0 };

        let largest_pat = *buf[chars!(PAT_META - offset, 128)]
            .iter()
            .max()
            .unwrap() as usize;

        // TODO: Document mysterious values
        // 0x0428 = 4 byte tag to identify mod type add 4 to skip
        let smp_index: usize = {
            4 + (0x0438 - offset) + (largest_pat + 1) * 1024 
        }; 

        if smp_index == buf.len() {
            return Err("MOD has no samples".into())
        }
        if smp_index >= buf.len() {
            return Err("Invalid MOD".into())
        }
        
        let smp_data: Vec<MODSample> = build_samples(smp_num, &buf, smp_index, alt_finetune)?;

        Ok(Box::new(Self {
            title,
            smp_num: smp_data.len() as u8,
            smp_data, 
            buf
        }))
    }
    
    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        if !folder.as_ref().is_dir() {
            return Err("Path is not a folder".into());
        }
        let smp: &MODSample         = &self.smp_data[index];
        let start: usize            = smp.index;
        let end: usize              = start + smp.length as usize;
        let path: PathBuf           = PathBuf::new()
            .join(folder)
            .join(name_sample(index, &smp.name));

        WAV::header(smp.freq as u32, 8, smp.length as u32, false)
            .write(path, (&self.buf[start..end]).to_signed())
    }

    fn number_of_samples(&self) -> usize {
        self.smp_num as usize
    }

    fn module_name(&self) -> &str {
        &self.title
    }
}

fn build_samples(smp_num: u8, buf: &[u8], smp_start: usize, alt_finetune: bool) -> Result<Vec<MODSample>, Error> {
    let mut smp_data: Vec<MODSample> = Vec::with_capacity(smp_num as usize);
    let mut smp_pcm_stream_index: usize = smp_start;

    for i in 0..smp_num as usize {
        let offset = MOD_SMP_START + (i * MOD_SMP_LEN); 
        let len: u16 = BE::read_u16(&buf[word!(0x0016 + offset)]).wrapping_mul(2); 
        if len == 0 { continue; }

        if len as usize > (128 * 1024) {
            return Err("MOD contains sample exceeding 128KB".into()); 
        }

        if len as usize + smp_pcm_stream_index > buf.len() { break; }

        let finetune: u8 = buf[0x0018 + offset];

        let freq: u16 = match alt_finetune {
            true => {
                (// untested
                    8363.0 * 2.0_f32.powf((-(finetune as i8).wrapping_shl(3)) as f32 / 1536.0)
                ) as u16
            },
            false => FINETUNE_TABLE[((finetune & 0xf) ^ 8) as usize]
        };

        smp_data.push(MODSample {
            name: string_from_chars(&buf[chars!(offset, 22)]),
            index: smp_pcm_stream_index,
            length: len, 
            freq
        });
        
        smp_pcm_stream_index += len as usize;
    }
    
    Ok(smp_data)
}