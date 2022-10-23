use super::compression::decompress_sample;
use crate::utils::signed::make_signed_u8_checked;
use crate::{dword, utils::prelude::*, TrackerDumper, TrackerModule, TrackerSample, XmoditsError};

type ITSample = TrackerSample;

const IT_HEADER_ID: &[u8] = b"IMPM";
const ZIRCON: &[u8] = b"ziRCON"; // mmcmp compression
const IT_HEADER_LEN: usize = 192;
const MASK_SMP_BITS: u8 = 0b0000_0010; // 16/8bit samples
const MASK_SMP_STEREO: u8 = 0b0000_0100; // 0 = mono, 1 = stereo
const MASK_SMP_COMP: u8 = 0b0000_1000; // Does sample use compression?
const IT215: u16 = 0x0215; // IT215 compression

pub struct ITFile {
    title: String,
    buf: Vec<u8>,
    pcm_cache: Vec<Vec<u8>>,
    version: u16,
    compat_ver: u16,
    smp_num: u16,
    smp_data: Vec<ITSample>,
}

impl TrackerDumper for ITFile {
    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < IT_HEADER_LEN {
            return Err(XmoditsError::invalid(
                "File is not a valid Impulse Tracker module",
            ));
        }
        if &buf[slice!(0x0000, 6)] == ZIRCON {
            return Err(XmoditsError::unsupported(
                "Unsupported IT: Uses 'ziRCON' sample compression",
            ));
        }
        if &buf[dword!(0x0000)] != IT_HEADER_ID {
            return Err(XmoditsError::invalid(
                "File is not a valid Impulse Tracker module",
            ));
        }

        Ok(())
    }

    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error> {
        Self::validate(&buf)?;

        let title: String = read_string(&buf, 0x0004, 26);
        let ord_num: u16 = read_u16_le(&buf, 0x0020);
        let ins_num: u16 = read_u16_le(&buf, 0x0022);
        let smp_num: u16 = read_u16_le(&buf, 0x0024);
        let version: u16 = read_u16_le(&buf, 0x0028);
        let compat_ver: u16 = read_u16_le(&buf, 0x002A);
        let smp_ptr_list: u16 = 0x00c0 + ord_num + (ins_num * 4);
        let mut smp_ptrs: Vec<u32> = Vec::with_capacity(smp_num as usize);

        for i in 0..smp_num {
            let index = smp_ptr_list + (i * 4);
            smp_ptrs.push(read_u32_le(&buf, index as usize));
        }

        let smp_data: Vec<ITSample> = build_samples(&buf, smp_ptrs);
        let smp_num: u16 = smp_data.len() as u16;

        Ok(Box::new(Self {
            title,
            smp_num,
            smp_data,
            version,
            compat_ver,
            buf,
            pcm_cache: vec![Vec::new(); smp_num as usize],
        }))
    }

    fn pcm(&mut self, index: usize) -> Result<&[u8], Error> {
        let smp = &mut self.smp_data[index];

        if smp.is_compressed && self.pcm_cache[index].is_empty() {
            self.pcm_cache[index] = decompress_sample(
                &self.buf[smp.ptr..],
                smp.len as u32,
                smp.bits,
                self.compat_ver == IT215,
                smp.is_stereo,
            )?;
        };

        Ok(match smp.is_compressed {
            true => &self.pcm_cache[index],

            false => match smp.bits {
                8 => make_signed_u8_checked(&mut self.buf, smp),
                _ => &self.buf[smp.ptr_range()],
            },
        })
    }

    fn number_of_samples(&self) -> usize {
        self.smp_num as usize
    }

    fn module_name(&self) -> &str {
        &self.title
    }

    fn list_sample_data(&self) -> &[TrackerSample] {
        &self.smp_data
    }

    fn format(&self) -> &str {
        "Impulse Tracker"
    }
}

fn build_samples(buf: &[u8], smp_ptrs: Vec<u32>) -> Vec<ITSample> {
    let mut sample_data: Vec<ITSample> = Vec::with_capacity(smp_ptrs.len());

    for (index, i) in smp_ptrs.iter().enumerate() {
        let offset: usize = *i as usize;
        let len: u32 = read_u32_le(buf, 0x0030 + offset);
        if len == 0 {
            continue;
        }

        let ptr: u32 = read_u32_le(buf, 0x0048 + offset);
        let flags: u8 = buf[0x012 + offset];
        let bits: u8 = (((flags & MASK_SMP_BITS) >> 1) + 1) * 8;
        let is_compressed: bool = ((flags & MASK_SMP_COMP) >> 3) == 1;
        // let smp_stereo: bool     = ((smp_flag & MASK_SMP_STEREO) >> 2)   == 1;
        let is_stereo: bool = false;

        // convert to length in bytes
        let len: u32 = len * ((bits / 8) as u32) * (is_stereo as u32 + 1);

        if !is_compressed    // break out of loop if we get a funky offset
            && (ptr + len) as usize > buf.len()
        {
            break;
        }

        let filename: String = read_string(buf, 0x0004 + offset, 12);
        let name: String = read_string(buf, 0x0014 + offset, 26);
        let rate: u32 = read_u32_le(buf, 0x003C + offset);

        sample_data.push(ITSample {
            name,
            filename,
            raw_index: index,
            len: len as usize,
            ptr: ptr as usize,
            flags,
            rate,
            is_stereo,
            is_compressed,
            bits,
            ..Default::default()
        })
    }

    sample_data
}
