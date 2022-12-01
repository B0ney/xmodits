/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::utils::prelude::*;
use crate::{TrackerDumper, TrackerModule, TrackerSample, XmoditsError};

use super::deltadecode::{delta_decode_u16_checked, delta_decode_u8_checked};

const XM_HEADER_ID: &[u8; 17] = b"Extended Module: ";
const MOD_PLUGIN_PACKED: &[u8; 20] = b"MOD Plugin packed   ";
const XM_MAGIC_NUM: u8 = 0x1a;
const XM_MIN_VER: u16 = 0x0104;
const MASK_SMP_BITS: u8 = 0b0001_0000; // 1 = 16 bit samples, 0 = 8 bit
const XM_SMP_SIZE: usize = 40;
const XM_INS_SIZE: u32 = 263;

type XMSample = TrackerSample;

pub struct XMFile {
    buf: Vec<u8>,
    module_name: String,
    samples: Vec<XMSample>,
    smp_num: usize,
}

impl TrackerDumper for XMFile {
    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < 60
            || read_slice(buf, 0x0000, 17) != XM_HEADER_ID
            || buf[0x0025] != XM_MAGIC_NUM
        {
            return Err(XmoditsError::invalid("Not a valid Extended Module"));
        }
        if read_slice(buf, 0x0026, 20) == MOD_PLUGIN_PACKED {
            return Err(XmoditsError::unsupported(
                "Unsupported XM: Uses 'MOD plugin packed'",
            ));
        }

        let version: u16 = read_u16_le(buf, 0x003A);

        if version < XM_MIN_VER {
            return Err(XmoditsError::unsupported(
                "Unsupported XM: Version below 0104",
            ));
        }

        Ok(())
    }

    fn load_from_buf_unchecked(buf: Vec<u8>) -> Result<TrackerModule, Error> {
        let module_name: String = read_string(&buf, 0x0011, 20);
        let header_size: u32 = read_u32_le(&buf, 0x003c);
        let patnum: u16 = read_u16_le(&buf, 0x0046);
        let insnum: u16 = read_u16_le(&buf, 0x0048);
        let ins_offset: usize = skip_pat_header(&buf, patnum as usize, header_size)?;
        let samples: Vec<XMSample> = build_samples(&buf, ins_offset, insnum as usize)?
            .into_iter()
            .filter(|x| x.len != 0)
            .collect();
        let smp_num: usize = samples.len();

        Ok(Box::new(Self {
            module_name,
            buf,
            samples,
            smp_num,
        }))
    }

    fn pcm(&mut self, index: usize) -> Result<&[u8], Error> {
        let smp = &mut self.samples[index];

        Ok(match smp.bits {
            8 => delta_decode_u8_checked(&mut self.buf, smp),
            _ => delta_decode_u16_checked(&mut self.buf, smp),
        })
    }

    fn number_of_samples(&self) -> usize {
        self.smp_num
    }

    fn module_name(&self) -> &str {
        &self.module_name
    }

    fn list_sample_data(&self) -> &[crate::TrackerSample] {
        &self.samples
    }

    fn format(&self) -> &str {
        "Extended Module"
    }
}

/// Skip xm pattern headers to access instrument headers.
/// Pattern headers do not have a fixed size so they need to be calculated.
fn skip_pat_header(buf: &[u8], patnum: usize, hdr_size: u32) -> Result<usize, Error> {
    let mut offset: usize = 60 + hdr_size as usize; // add 60 to go to pat header
    let mut pat_hdr_len: u32;
    let mut pat_data_size: u32;

    if patnum > 256 {
        return Err(XmoditsError::invalid(
            "Invalid XM: Contains more than 256 patterns",
        ));
    }

    for _ in 0..patnum {
        pat_hdr_len = read_u32_le(buf, offset);
        pat_data_size = read_u16_le(buf, 0x0007 + offset) as u32;
        offset += (pat_hdr_len + pat_data_size) as usize;
    }

    Ok(offset as usize)
}

/* Needs refactoring, it works but looks horrible. */
fn build_samples(buf: &[u8], ins_offset: usize, ins_num: usize) -> Result<Vec<XMSample>, Error> {
    let mut samples: Vec<XMSample> = Vec::with_capacity(25);
    let mut offset: usize = ins_offset;
    let mut ins_hdr_size: u32;
    let mut ins_smp_num: u16;

    if ins_num > 128 {
        return Err(XmoditsError::invalid(
            "Invalid XM: Contains more than 128 instruments",
        ));
    }

    for _ in 0..ins_num {
        ins_hdr_size = read_u32_le(buf, offset);
        ins_smp_num = read_u16_le(buf, 0x001b + offset);

        if ins_hdr_size == 0 || ins_hdr_size > XM_INS_SIZE {
            ins_hdr_size = XM_INS_SIZE;
        }

        offset += ins_hdr_size as usize; // skip to sample headers

        // Offset where the sample headers starts
        let start_smp_hdr: usize = offset;
        let total_smp_hdr_size: usize = XM_SMP_SIZE * ins_smp_num as usize;

        // Cumulative sum of sample data size
        let mut smp_len_cumulative: usize = 0;

        for _ in 0..ins_smp_num {
            let len: usize = read_u32_le(buf, offset) as usize;

            // break out of loop if offset leads to overflow panic
            if 0x0010 + offset > buf.len() || start_smp_hdr + total_smp_hdr_size + len > buf.len() {
                break;
            }

            let ptr: usize = start_smp_hdr + total_smp_hdr_size + smp_len_cumulative;
            let name: String = read_string(buf, 0x0012 + offset, 22);
            let flags: u8 = buf[0x000e + offset];
            let bits: u8 = (((flags & MASK_SMP_BITS) >> 4) + 1) * 8;
            let finetune: i8 = buf[0x000d + offset] as i8;
            let notenum: i8 = buf[0x0010 + offset] as i8;

            let period: f32 = 7680.0 - ((48.0 + notenum as f32) * 64.0) - (finetune as f32 / 2.0);
            let rate: u32 = (8363.0 * 2.0_f32.powf((4608.0 - period) / 768.0)) as u32;
            let loop_start: u32 = read_u32_le(buf, 0x0004 + offset);
            let loop_end: u32 = loop_start + read_u32_le(buf, 0x0008 + offset);
    
            samples.push(XMSample {
                filename: name.clone(),
                name,
                raw_index: samples.len(),
                len,
                ptr,
                flags,
                bits,
                rate,
                loop_start,
                loop_end,
                ..Default::default()
            });
            smp_len_cumulative += len;
            offset += XM_SMP_SIZE;
        }

        offset += smp_len_cumulative;
    }

    Ok(samples)
}
