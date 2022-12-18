/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
TODO
https://github.com/a740g/QB64-MOD-Player/blob/main/docs/MOD.txt
https://github.com/OpenMPT/openmpt/blob/master/soundlib/Load_mod.cpp
https://github.com/milkytracker/MilkyTracker/blob/master/src/milkyplay/LoaderMOD.cpp // milkyplay is BSD licensed
*/

use std::borrow::Cow;

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

    fn load_from_buf_unchecked(buf: Vec<u8>) -> Result<TrackerModule, Error> {
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

        // let last = smp_data.last().unwrap();
        // let end = last.ptr + last.len;

        // if end != buf.len() {
        //     let v1 = buf.len() as f32;
        //     let v2 = end as f32;
        //     let percent_delta = ((v1 - v2).abs() / ((v1 + v2) / 2.0)) * 100.0;
        //     println!("{}", percent_delta);
        // }

        // ^
        // This approach is kinda simiar to openmpt's method of detecting invalid mod files
        // by calculating a threshold.
        // However this must be rewritten
        // https://github.com/OpenMPT/openmpt/blob/master/soundlib/Load_mod.cpp#L313

        Ok(Box::new(Self {
            title,
            smp_num: smp_data.len() as u8,
            smp_data,
            buf,
        }))
    }

    fn pcm(&mut self, index: usize) -> Result<Cow<[u8]>, XmoditsError> {
        let smp = &mut self.smp_data[index];
        Ok(Cow::Borrowed(make_signed_u8_checked(&mut self.buf, smp)))
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

        // let finetune: u8 = buf[0x0018 + offset];

        let name = read_string(buf, offset, 22);
        let loop_start: u32 = read_u16_le(buf, 0x001A + offset) as u32 * 16;
        // loop length in words, 1 word = 2 bytes
        // if 1, looping is disabled
        let loop_end: u32 = match read_u16_le(buf, 0x001C + offset) {
            1 => 0, // TODO
            length => loop_start + (length as u32 * 16) as u32,
        };

        smp_data.push(MODSample {
            // filename: name.clone(),
            name,
            raw_index: i,
            len: len as usize,
            ptr: smp_pcm_stream_index,
            bits: 8,
            rate: 8363,
            loop_start,
            loop_end,
            ..Default::default()
        });

        smp_pcm_stream_index += len as usize;
    }

    Ok(smp_data)
}
