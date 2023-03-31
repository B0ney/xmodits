/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::interface::{TrackerDumper, TrackerModule};
use crate::utils::prelude::*;
use crate::utils::reader::read_str;
use crate::XmoditsError;
use crate::LOADERS;
use std::borrow::Cow;

const UPKG_MAGIC: u32 = 0x9E2A83C1;
const UPKG_HEADER_SIZE: usize = 64;

struct DontUseMe;

/// "Abandon all hope ye who try to parse this file format." - Tim Sweeney, Unreal Packages
pub struct UMXFile(DontUseMe);

impl TrackerDumper for UMXFile {
    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < UPKG_HEADER_SIZE || read_u32_le(buf, 0x0000)? != UPKG_MAGIC {
            return Err(XmoditsError::invalid("Not a valid Unreal package"));
        }

        let version = read_u32_le(buf, 0x0004)?;

        if version < 61 {
            return Err(XmoditsError::unsupported(
                "UMX versions below 61 are unsupported.",
            ));
        }
        // // Is this check even useful?
        // let export_count = read_u32_le(buf, 0x0014);
        //
        // if export_count > 1 {
        //     return Err(XmoditsError::unsupported(
        //         "Unreal package contains more than 1 entry.",
        //     ));
        // }

        let name_count: usize = read_u32_le(buf, 0x000C)? as usize;
        let name_offset: usize = read_u32_le(buf, 0x0010)? as usize;

        let mut name_table: Vec<Cow<str>> = Vec::with_capacity(name_count);

        let mut offset = name_offset;

        for _ in 0..name_count {
            name_table.push(if version < 64 {
                let mut length: usize = 0;

                while buf[length + offset] != 0 && length + offset < buf.len() {
                    length += 1;
                }

                let name = read_str(buf, offset, length)?;
                offset += length;
                name
            } else {
                let length: usize = buf[offset] as usize - 1; // length of string inc \0, -1 to remove null
                offset += 1; // skip size field

                let name = read_str(buf, offset, length)?;
                offset += length;
                name
            });

            offset += 1; // Add 1 to skip '\0'
            offset += 4;
        }

        if !name_table.contains(&Cow::from("Music")) {
            return Err(XmoditsError::invalid(
                "Unreal Package does not contain any music",
            ));
        }

        Ok(())
    }

    fn load_from_buf_unchecked(mut buf: Vec<u8>) -> Result<TrackerModule, Error> {
        let version = read_u32_le(&buf, 0x0004)?;
        let export_offset: usize = read_u32_le(&buf, 0x0018)? as usize;
        let mut offset: usize = export_offset;

        // Export table
        offset += read_compact_index(&buf, offset).1; // class index
        offset += read_compact_index(&buf, offset).1; // super index
        offset += 4; // group
        offset += read_compact_index(&buf, offset).1; // obj name
        offset += 4; // obj flags

        let (serial_size, inc) = read_compact_index(&buf, offset);

        if serial_size == 0 {
            return Err(XmoditsError::invalid("UMX doesn't contain anything"));
        }

        offset += inc; // serial size skip

        let serial_offset = read_compact_index(&buf, offset).0 as usize;

        // jump to object
        offset = serial_offset;
        offset += read_compact_index(&buf, offset).1; // skip name index

        if version > 61 {
            offset += 4;
        }

        offset += read_compact_index(&buf, offset).1; // skip obj_size field
        let (size, inc) = read_compact_index(&buf, offset);
        offset += inc; // skip size of object data

        _ = buf.drain(..offset); // Strip UMX header
        _ = buf.drain(size as usize..); // Strip UMX tables

        // The first item in the name table can be used as a "hint", but this is unreliable.
        // This approach iterates through an array of tuples containing two functions:
        // one validates the buffer, the other loads it.
        for (_, (validator, loader)) in LOADERS.entries().filter(|(ext, _)| **ext != "umx") {
            if validator(&buf).is_ok() {
                return loader(buf);
            }
        }

        Err(XmoditsError::unsupported(
            "UMX doesn't contain a supported format",
        ))
    }
    /*  You should not call these methods from UMX (should be impossible).
    But incase someone somehow manages to do so, panic :) */
    fn number_of_samples(&self) -> usize {
        unimplemented!()
    }
    fn module_name(&self) -> &str {
        unimplemented!()
    }
    fn list_sample_data(&self) -> &[crate::TrackerSample] {
        unimplemented!()
    }
    fn pcm(&mut self, _: usize) -> Result<Cow<[u8]>, XmoditsError> {
        unimplemented!()
    }
    fn format(&self) -> &str {
        unimplemented!()
    }
}

fn read_compact_index(buf: &[u8], offset: usize) -> (i32, usize) {
    let mut output: i32 = 0;
    let mut signed: bool = false;
    let mut offset: usize = offset;
    let mut size: usize = 0;

    for i in 0..5 {
        let x = buf[offset] as i32;
        offset += 1;
        size += 1;

        if i == 0 {
            if (x & 0x80) > 0 {
                signed = true;
            }

            output |= x & 0x3F;

            if x & 0x40 == 0 {
                break;
            }
        } else if i == 4 {
            output |= (x & 0x1F) << (6 + (3 * 7));
        } else {
            output |= (x & 0x7F) << (6 + ((i - 1) * 7));

            if x & 0x80 == 0 {
                break;
            }
        }
    }

    if signed {
        output *= -1;
    }

    (output, size)
}

#[ignore = "not part of unit test"]
#[test]
fn test1() {
    // let b = read_compact_index(&[0x74,0x07], 0);
    // dbg!(b.0);
    // let a: _ = UMXFile::load_module("./test/umx/UNATCO_Music.umx");
    // let a: _ = UMXFile::load_module("./test/umx/desu/MJ12_Music.umx");
    let a: _ = crate::loader::load_module("./test/umx/ut/Mech8.umx");
    //
    dbg!(a.unwrap().module_name());
}

// Test read compact index works
#[test]
fn test_compact_index() {
    let tests: Vec<(i32, &[u8])> = vec![
        (1, &[0x01]),
        (500, &[0x74, 0x07]),
        (1000, &[0x68, 0x0f]),
        (10, &[0x0a]),
        (100, &[0x64, 0x01]),
        (10_000_000, &[0x40, 0xDA, 0xC4, 0x09]),
        (1_000_000_000, &[0x40, 0xA8, 0xD6, 0xB9, 0x07]),
    ];

    for (number, compact) in tests {
        let a = read_compact_index(compact, 0);
        dbg!(a.1);
        assert_eq!(a.0, number);
    }
}