mod test;
mod compression;
use crate::utils::prelude::*;
use self::compression::decompress_sample;

const IT_HEADER_ID: &str    = "IMPM";    // IMPM
// const IT_SAMPLE_ID: u32     = 0x49_4D_50_53;    // IMPS
const IT_HEADER_LEN: usize  = 192;
const IT_SAMPLE_LEN: usize  = 80;
const MASK_SMP_BITS: u8     = 0b0000_0010;      // 16/8bit samples
const MASK_SMP_COMP: u8     = 0b0000_1000;      // Does sample use compression?
const MASK_SMP_STEREO: u8   = 0b0000_0100;      // 0 = mono, 1 = stereo
const IT215: u16            = 0x0215;           // IT215 compression 

#[derive(Debug)]
pub struct ITSample {
    pub filename: String,
    pub name: String,           
    pub smp_len: u32,           // This is NOT in bytes
    pub smp_ptr: u32,           // Sample Pointer
    pub smp_rate: u32,          // Sample rate
    pub smp_flag: u8,           // Sample flag
    pub smp_bits: u8,           // Does sample use 16 or 8 bits
    pub smp_comp: bool,         // Does sample use compression?
    pub smp_stereo: bool,       // Is sample stereo?
}

#[derive(Debug)]
pub struct ITFile {
    title: String,
    buf: Vec<u8>,
    pub version: u16,
    pub compat_ver: u16,
    pub smp_num: u16,
    pub smp_data: Vec<ITSample>,
}

use crate::{TrackerDumper, TrackerModule};

impl TrackerDumper for ITFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized
    {
        if buf.len() < IT_HEADER_LEN
            || read_chars(&buf, 0x0000, 4) != IT_HEADER_ID.as_bytes()
        {
            return Err("File is not a valid Impulse Tracker Module".into());
        };

        let title: String       = string_from_chars(&buf[chars!(0x0004, 26)]);
        let ord_num: u16        = read_u16_le(&buf, 0x0020);
        let ins_num: u16        = read_u16_le(&buf, 0x0022);
        let smp_num: u16        = read_u16_le(&buf, 0x0024);
        let version: u16        = read_u16_le(&buf, 0x0028);
        let compat_ver: u16     = read_u16_le(&buf, 0x002A);
        let smp_ptr_list: u16   = 0x00c0 + ord_num + (ins_num * 4);

        let mut smp_ptrs: Vec<u32> = Vec::new();

        for i in 0..smp_num {
            let index = smp_ptr_list + (i * 4);
            smp_ptrs.push(read_u32_le(&buf, index as usize));
        }

        let smp_data: Vec<ITSample> = build_samples(&buf, smp_ptrs)?;
                
        let smp_num: u16 = smp_data.len() as u16;

        Ok(Box::new(Self {
            title,
            smp_num,
            smp_data,
            version,
            compat_ver,
            buf,
            
        }))
    }
    // Needs to be more readable
    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        if !folder.as_ref().is_dir() {
            return Err("Path is not a folder".into());
        }

        let smp: &ITSample          = &self.smp_data[index];
        let wav_header: [u8; 44]    = wav::build_header(
            smp.smp_rate, smp.smp_bits,
            smp.smp_len, smp.smp_stereo,
        );
        let start_ptr: usize    = smp.smp_ptr as usize;
        let path: PathBuf       = PathBuf::new()
            .join(folder)
            .join(name_sample(index, &smp.filename));
        let mut file: File      = File::create(path)?;
        // Write Wav Header
        file.write_all(&wav_header)?;

        // Write PCM data
        if smp.smp_comp {
            let decomp = decompress_sample(
                &self.buf[start_ptr..], smp.smp_len,
                smp.smp_bits, self.compat_ver == IT215
            )?;
            file.write_all(&decomp)?;

        } else {
            let end_ptr = start_ptr + 
                (smp.smp_len * (smp.smp_bits as u32 / 8)) as usize;
            let mut raw_data = &self.buf[start_ptr..end_ptr];
            let mut b: Vec<u8> = Vec::new();
            
            // convert sample data to "signed" values if it's 8-bit  
            if smp.smp_bits == 8 {
                b = raw_data.to_signed(); 
                raw_data = &b; // make raw data reference b instead
            }

            file.write_all(raw_data)?;
        }

        Ok(())
    }

    fn number_of_samples(&self) -> usize {
        self.smp_num as usize
    }

    fn module_name(&self) -> &String {
        &self.title
    }
}

fn build_samples(it_data: &[u8], smp_ptr: Vec<u32>) -> Result<Vec<ITSample>, Error> {
    let mut smp_meta: Vec<ITSample> = Vec::new();

    for i in smp_ptr {
        let offset: usize       = i as usize;
        let smp_len: u32        = read_u32_le(it_data, 0x0030 + offset);
        
        if smp_len == 0 { continue; }

        let filename: String    = string_from_chars(&it_data[chars!(0x0004 + offset, 12)]);
        let name: String        = string_from_chars(&it_data[chars!(0x0014 + offset, 26)]);

        let smp_ptr: u32        = read_u32_le(it_data, 0x0048 + offset);
        let smp_rate: u32       = read_u32_le(it_data, 0x003C + offset);

        let smp_flag: u8        = it_data[0x012 + offset];
        let smp_bits: u8        = (((smp_flag & MASK_SMP_BITS) >> 1) +  1) * 8;
        let smp_comp: bool      = ((smp_flag & MASK_SMP_COMP) >> 3)     == 1;
        let smp_stereo: bool    = ((smp_flag & MASK_SMP_STEREO) >> 2)   == 1;

        smp_meta.push(ITSample {
            filename,
            name,
            smp_len,
            smp_ptr,
            smp_rate,
            smp_bits,
            smp_comp,
            smp_stereo,
            smp_flag,
        })
    }

    Ok(smp_meta)
}