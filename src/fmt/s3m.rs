/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{
    dword,
    utils::{prelude::*, signed::make_signed_u16_checked},
    TrackerDumper, TrackerModule, TrackerSample, XmoditsError,
};

const S3M_HEADER_ID: &[u8] = b"SCRM";
const S3M_MAGIC_NUMBER: u8 = 0x10;
// const SMP_MASK_STEREO: u8 = 0b0000_0010;
const SMP_MASK_BITS: u8 = 0b0000_0100;
const INS_HEAD_LENGTH: usize = 13;

type S3MSample = TrackerSample;

pub struct S3MFile {
    buf: Vec<u8>,
    title: String,
    smp_data: Vec<S3MSample>,
}

impl TrackerDumper for S3MFile {
    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < 0x0060
            || buf[0x001d] != S3M_MAGIC_NUMBER
            || &buf[dword!(0x002c)] != S3M_HEADER_ID
        {
            return Err(XmoditsError::invalid(
                "File is not a valid Scream Tracker Module",
            ));
        }
        Ok(())
    }

    fn load_from_buf_unchecked(buf: Vec<u8>) -> Result<TrackerModule, Error> {
        let title: String = read_string(&buf, 0x0000, 28);
        let ord_count: u16 = read_u16_le(&buf, 0x0020);
        let ins_count: u16 = read_u16_le(&buf, 0x0022);
        let ins_ptr_list: u16 = 0x0060 + ord_count;

        let mut ins_ptrs: Vec<usize> = Vec::with_capacity(ins_count as usize);

        for i in 0..ins_count {
            let index: u16 = ins_ptr_list + (i * 2);
            // convert parameter to byte-level offset by << 4
            // cast to usize to avoid potential overflow
            ins_ptrs.push((read_u16_le(&buf, index as usize) as usize) << 4)
        }

        let smp_data: Vec<S3MSample> = build_samples(&buf, ins_ptrs);

        Ok(Box::new(Self {
            title,
            smp_data,
            buf,
        }))
    }

    fn pcm(&mut self, index: usize) -> Result<&[u8], Error> {
        let smp = &mut self.smp_data[index];

        Ok(match smp.bits {
            8 => &self.buf[smp.ptr_range()],
            _ => make_signed_u16_checked(&mut self.buf, smp),
        })
    }

    fn number_of_samples(&self) -> usize {
        self.smp_data.len()
    }

    fn module_name(&self) -> &str {
        &self.title
    }

    fn list_sample_data(&self) -> &[crate::TrackerSample] {
        &self.smp_data
    }

    fn format(&self) -> &str {
        "Scream Tracker"
    }
}

fn build_samples(buf: &[u8], ins_ptr: Vec<usize>) -> Vec<S3MSample> {
    let mut samples: Vec<S3MSample> = Vec::with_capacity(ins_ptr.len());

    for (index, i) in ins_ptr.iter().enumerate() {
        if buf[*i] != 0x1 {
            continue;
        } // if it's not a PCM instrument, skip
        let offset: usize = *i + INS_HEAD_LENGTH; // skip instrument header (13 bytes)
        let len: u32 = read_u32_le(buf, 0x0003 + offset) & 0xffff;

        if len == 0 {
            continue;
        }

        let hi_ptr: u8 = buf[offset];
        let lo_ptr: u16 = read_u16_le(buf, 0x0001 + offset);
        let ptr: u32 = (hi_ptr as u32) >> 16 | (lo_ptr as u32) << 4;
        let flags: u8 = buf[0x0012 + offset];
        // let is_stereo: bool = (flags & SMP_MASK_STEREO) >> 1 == 1;
        let is_stereo: bool = false;
        let bits: u8 = if (flags & SMP_MASK_BITS) >> 2 == 1 {
            16
        } else {
            8
        };
        let len: u32 = len * (is_stereo as u32 + 1) * (bits / 8) as u32;

        if (ptr + len) > buf.len() as u32 {
            break;
        } // break out of loop if we get a funky offset

        let name: String = read_string(buf, 0x0023 + offset, 28);
        let rate: u32 = read_u32_le(buf, 0x0013 + offset);
        let loop_start: u32 = read_u32_le(buf, 0x0007 + offset);
        let loop_end: u32 = read_u32_le(buf, 0x000b + offset);

        samples.push(S3MSample {
            filename: name.clone(),
            name,
            raw_index: index,
            len: len as usize,
            ptr: ptr as usize,
            bits,
            rate,
            loop_start,
            loop_end,
            // is_stereo: false,
            // is_interleaved: false,
            ..Default::default()
        })
    }

    samples
}
