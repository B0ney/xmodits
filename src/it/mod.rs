
mod test;
mod compression;
use crate::utils::prelude::*;
use byteorder::{ByteOrder, LE, BE};
use self::compression::decompress_sample;

const IT_HEADER_ID: u32 = 0x49_4D_50_4D;    // IMPM
const IT_SAMPLE_ID: u32 = 0x49_4D_50_53;    // IMPS
const IT_HEADER_LEN: usize = 192;
const IT_SAMPLE_LEN: usize = 80;
const MASK_SMP_BITS: u8 = 0b0000_0010;      // 16/8bit samples
const MASK_SMP_COMP: u8 = 0b0000_1000;      // Does sample use compression?
const MASK_SMP_STEREO: u8 = 0b0000_0100;    // 0 = mono, 1 = stereo
const IT214: u16 = 0x0214;                  // IT214 compression 

#[derive(Debug)]
pub struct ITSample {
    pub filename: String,
    pub name: String,
    pub smp_len: u32,           // This is NOT in bytes
    pub smp_ptr: u32,           //
    pub smp_rate: u32,          //
    pub smp_flag: u8,           //
    pub smp_bits: u8,           // Does sample use 16 or 8 bits
    pub smp_comp: bool,         // Does sample use compression
    pub smp_stereo: bool,       // Is the sample stereo
}

#[derive(Debug)]
pub struct ITFile {
    title: String,
    buffer: Vec<u8>,
    pub version: u16,
    pub compat_version: u16,
    pub sample_number: u16,
    pub samples_meta: Vec<ITSample>,
}

use crate::{TrackerDumper, TrackerModule};

impl TrackerDumper for ITFile {
    fn load_from_buf(buffer: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized
    {
        if buffer.len() < IT_HEADER_LEN
            || BE::read_u32(&buffer[offset_u32!(0x0000)]) != IT_HEADER_ID
        {
            return Err("File is not a valid Impulse Tracker Module".into());
        };

        let samples_allocated = LE::read_u16(&buffer[offset_u16!(0x0024)]);
        let samples_meta = build_samples(&buffer, samples_allocated)?;
        // Some samples are empty, so set the sample number
        // according to the length of samples meta. 
        let sample_number = samples_meta.len() as u16;

        Ok(Box::new(Self {
            title:          string_from_chars(&buffer[offset_chars!(0x0004, 26)]),
            sample_number,
            samples_meta,
            version:        LE::read_u16(&buffer[offset_u16!(0x0028)]),
            compat_version: LE::read_u16(&buffer[offset_u16!(0x002A)]),
            buffer,
            
        }))
    }
    // TODO: export with sample name
    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        if !folder.as_ref().is_dir() {
            return Err("Path is not a folder".into());
        }
        let smp     = &self.samples_meta[index];
        let start_ptr   = smp.smp_ptr as usize;
        let wav_header  = wav::build_header(
            smp.smp_rate, smp.smp_bits,
            smp.smp_len, smp.smp_stereo,
        );
        let path: PathBuf = PathBuf::new()
            .join(folder)
            .join(name_sample(index, &smp.filename));
        let mut file    = File::create(path)?;
        // Write Wav Header
        file.write_all(&wav_header)?;

        // Write PCM data
        if smp.smp_comp {
            let decomp = decompress_sample(
                &self.buffer[start_ptr..], smp.smp_len,
                smp.smp_bits, self.compat_version != IT214 // Needs testing
            )?;
            file.write_all(&decomp)?;

        } else {
            let end_ptr = start_ptr + 
                (smp.smp_len * (smp.smp_bits as u32 / 8)) as usize;
            let mut raw_data = &self.buffer[start_ptr..end_ptr];
            let mut b: Vec<u8> = Vec::new();
            
            // convert sample data to "signed" values if it's 8-bit  
            if smp.smp_bits == 8 {
                b = raw_data.to_signed(); 
                raw_data = &b; // make raw data reference b instead
            }

            file.write_all(&raw_data)?;
        }

        Ok(())
    }

    fn number_of_samples(&self) -> usize {
        self.sample_number as usize
    }

    fn module_name(&self) -> &String {
        &self.title
    }
}

fn build_samples(it_data: &[u8], num_samples: u16) -> Result<Vec<ITSample>, Error> {
    let mut ins_start_index: usize  = 0;
    let mut smp_meta: Vec<ITSample> = Vec::new();

    if num_samples == 0 {
        return Err("IT module doesn't contain any samples.".into());
    }

    for i in 0..(it_data.len() - 4) { // 4 is the amount of bytes a u32 takes up. Prevents panic.
        if BE::read_u32(&it_data[offset_u32!(i)]) == IT_SAMPLE_ID {
            ins_start_index = i;
            break;
        }
    }

    for i in 0..num_samples as usize {
        let offset: usize       = ins_start_index + (i * IT_SAMPLE_LEN) as usize;
        let smp_flag: u8        = it_data[0x012 + offset];
        let smp_len: u32        = LE::read_u32(&it_data[offset_u32!(0x0030 + offset)]);

        if smp_len == 0 { continue; }

        smp_meta.push(ITSample {
            filename:   string_from_chars(&it_data[offset_chars!(0x0004 + offset, 12)]),
            name:       string_from_chars(&it_data[offset_chars!(0x0014 + offset, 26)]),
            smp_len,
            smp_ptr:    LE::read_u32(&it_data[offset_u32!(0x0048 + offset)]),
            smp_rate:   LE::read_u32(&it_data[offset_u32!(0x003C + offset)]),
            smp_bits:   (((smp_flag & MASK_SMP_BITS) >> 1) +  1) * 8,
            smp_comp:    ((smp_flag & MASK_SMP_COMP) >> 3)      == 1,
            smp_stereo:  ((smp_flag & MASK_SMP_STEREO) >> 2)    == 1,
            smp_flag,
        })
    }

    Ok(smp_meta)
}