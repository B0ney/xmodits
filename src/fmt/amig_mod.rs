use crate::tables::FINETUNE_TABLE;
use crate::utils::signed::make_signed_u8_checked;
use crate::{
    dword, utils::prelude::*, word, TrackerDumper, TrackerModule, TrackerSample, XmoditsError,
};
use byteorder::{ByteOrder, BE};

const ALT_FINETUNE: [&[u8]; 2] = [b"M&K!", b"FEST"];
const MOD_XPK_MAGIC: &[u8] = b"PP20"; // PP20
const MOD_SMP_START: usize = 0x0014; // offset where title ends & smp data begins
const MOD_SMP_LEN: usize = 0x1e;
const PAT_META: usize = 0x3b8;

type MODSample = TrackerSample;

/// This format has the least checks, use with caution.
pub struct MODFile {
    buf: Vec<u8>,
    title: String,
    smp_num: u8,
    smp_data: Vec<MODSample>,
}

/// I need to work on "MOD Format.md" before I continue working on this.
impl TrackerDumper for MODFile {
    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < 1085 {
            return Err(XmoditsError::invalid("Not a valid MOD file"));
        }
        if &buf[dword!(0x0000)] == MOD_XPK_MAGIC {
            return Err(XmoditsError::unsupported(
                "XPK compressed MOD files are not supported",
            ));
        }
        Ok(())
    }

    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error> {
        Self::validate(&buf)?;

        let title: String = read_string(&buf, 0x0000, 20);
        let alt_finetune: bool = ALT_FINETUNE.contains(&&buf[dword!(0x0438)]);

        // if it contains any non-ascii, it was probably made with ultimate sound tracker
        let smp_num: u8 = {
            // Valid ASCII chars are in between 32-127
            if buf[dword!(0x0438)].iter().any(|b| *b <= 32 || *b >= 126) {
                15
            } else {
                31
            }
        };

        // Fixed panic on modules made with ultimate sound tracker.
        // ^ outdated
        // TODO: Why did I add 1? I fogor
        let offset: usize = if smp_num == 15 { (15 + 1) * 30 } else { 0 };

        let largest_pat = *buf[slice!(PAT_META - offset, 128)].iter().max().unwrap() as usize;

        // TODO: Document mysterious values
        // 0x0438 = 4 byte tag to identify mod type add 4 to skip
        let smp_index: usize = { 4 + (0x0438 - offset) + (largest_pat + 1) * 1024 };

        if smp_index == buf.len() {
            return Err(XmoditsError::EmptyModule);
        }
        if smp_index >= buf.len() {
            return Err(XmoditsError::invalid("Not a valid MOD file"));
        }

        let smp_data: Vec<MODSample> = build_samples(smp_num, &buf, smp_index, alt_finetune)?;

        Ok(Box::new(Self {
            title,
            smp_num: smp_data.len() as u8,
            smp_data,
            buf,
        }))
    }

    fn pcm(&mut self, index: usize) -> Result<&[u8], Error> {
        let smp = &mut self.smp_data[index];
        Ok(make_signed_u8_checked(&mut self.buf, smp))
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
        "Amiga ProTracker"
    }
}

fn build_samples(
    smp_num: u8,
    buf: &[u8],
    smp_start: usize,
    alt_finetune: bool,
) -> Result<Vec<MODSample>, Error> {
    let mut smp_data: Vec<MODSample> = Vec::with_capacity(smp_num as usize);
    let mut smp_pcm_stream_index: usize = smp_start;

    for i in 0..smp_num as usize {
        let offset = MOD_SMP_START + (i * MOD_SMP_LEN);
        let len: u16 = BE::read_u16(&buf[word!(0x0016 + offset)]).wrapping_mul(2);
        if len == 0 {
            continue;
        }

        if len as usize > (128 * 1024) {
            return Err(XmoditsError::invalid("MOD contains sample exceeding 128KB"));
        }

        if len as usize + smp_pcm_stream_index > buf.len() {
            break;
        }

        let finetune: u8 = buf[0x0018 + offset];

        let freq: u16 = match alt_finetune {
            true => alt_frequency(finetune as i8),
            false => FINETUNE_TABLE[((finetune & 0xf) ^ 8) as usize],
        };
        let name = read_string(buf, offset, 22);

        smp_data.push(MODSample {
            // filename: name.clone(),
            name,
            raw_index: i,
            len: len as usize,
            ptr: smp_pcm_stream_index,
            bits: 8,
            rate: freq as u32,
            ..Default::default()
        });

        smp_pcm_stream_index += len as usize;
    }

    Ok(smp_data)
}

fn alt_frequency(finetune: i8) -> u16 {
    (363.0 * 2.0_f32.powf(-finetune.wrapping_shl(3) as f32 / 1536.0)) as u16
}
