use crate::XmoditsError;
use crate::tracker_formats::*;
use crate::{xm::XMFile, utils::prelude::*};
// https://github.com/Konstanty/libmodplug/blob/d1b97ed0020bc620a059d3675d1854b40bd2608d/src/load_umx.cpp#L196

const UM_MAGIC_NUMBER: u32 = 0x9E2A83C1;
struct DontUseMe;
pub struct UMXFile(DontUseMe);

use crate::interface::{TrackerDumper, TrackerModule};

type ModValidatorFunc = fn(&[u8]) -> Result<(), XmoditsError>;
type ModLoaderFunc = fn(Vec<u8>) -> Result<TrackerModule, XmoditsError>;

use once_cell::sync::Lazy;

static VALIDATE_LOADER: Lazy<[(ModValidatorFunc, ModLoaderFunc);3]> = Lazy::new(|| {
    [
        (|p| ITFile::validate(p), ITFile::load_from_buf),
        (|p| XMFile::validate(p), XMFile::load_from_buf),
        (|p| S3MFile::validate(p), S3MFile::load_from_buf),
        // (|p| MODFile::validate(&p), |p| MODFile::load_from_buf(p))
    ]
});

impl TrackerDumper for UMXFile {
    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < 69 // for now
            || read_u32_le(buf, 0x0000) != UM_MAGIC_NUMBER 
        {
            return Err(XmoditsError::invalid("Not a valid Unreal package"));
        }

        let version = read_u32_le(buf, 0x0004);

        if version < 61 {
            return Err(XmoditsError::unsupported("UMX versions below 61 are unsupported."))
        }

        let export_count = read_u32_le(buf, 0x0014);

        if export_count > 1 {
            return Err(XmoditsError::unsupported("Unreal package contains more than 1 entry."));
        }

        let name_count: usize   = read_u32_le(buf, 0x000C) as usize;
        let name_offset: usize  = read_u32_le(buf, 0x0010) as usize;

        let mut name_table: Vec<String> = Vec::with_capacity(name_count);

        let mut offset = name_offset;

        for _ in 0..name_count {
            let length: usize   = buf[offset] as usize;
            let name: String    = read_string(buf, offset, length);
            dbg!(&name);
            name_table.push(name);
            offset += length + 1; // Add 1 to skip \00
            offset += 4;
        }

        if !name_table.contains(&String::from("Music")) {
            return Err(XmoditsError::invalid("Unreal Package does not contain any music"));
        }

        // let import_count = read_u32_le(buf, 0x001C);
        // let import_offset = read_u32_le(buf, 0x0020) & 0x00ff;

        // dbg!(version);
        // dbg!(name_count);
        // dbg!(name_offset);
        // dbg!(export_count);
        // dbg!(import_offset);

        // obtain names
    
        // let chunk_size = read_compact_index(&buf, offset).1;

        // dbg!(chunk_size);
        dbg!(offset);
        // let mut offset: usize = 0;
        Ok(())
    }

    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
    {
        Self::validate(&buf)?;

        let export_offset: usize = read_u32_le(&buf, 0x0018) as usize;
        let mut offset: usize = export_offset;

        // The first item of the name table could be used to identify what module it contains?
        offset += read_compact_index(&buf, offset).1; // class index
        offset += read_compact_index(&buf, offset).1; // super index
        offset += 4;
        offset += read_compact_index(&buf, offset).1; // obj name
        offset += 4; // obj flags

        offset += read_compact_index(&buf, offset).1;   // serial size skip


        let (serial_offset, inc) = read_compact_index(&buf, offset);
        offset += inc;
        offset += 2;

        // if version > 61 {
        //     offset += 2;
        // }

        // identify header to verify it is a um* package
        
        // if possible determine format from header info

        // otherwise, use tests such as using magic numbers.

        // figure out what kind of module it contains
        // strip umx header from buffer

        // Technically validates the buffer twice... But who cares
        for (validator, loader) in VALIDATE_LOADER.iter() {
            if validator(&buf).is_ok() {
                return loader(buf)
            }
        };

        Err(XmoditsError::unsupported("UMX doesn't contain a supported format"))
    }

    /*  You should not call these methods from UMX (should be impossible).
        But incase someone somehow manages to do so, panic :) */ 
    fn export(&mut self, _: &dyn AsRef<Path>, _: usize) -> Result<(), Error> {
        unimplemented!()
    }
    fn number_of_samples(&self) -> usize {
        unimplemented!()
    }
    fn module_name(&self) -> &str {
        unimplemented!()
    }
    fn list_sample_data(&self) -> &[crate::TrackerSample] {
        unimplemented!()
    }
    fn write_wav(&mut self, _: &Path, _: usize) -> Result<(), Error> {
        unimplemented!()
    }
    fn pcm(&mut self, _: usize) -> Result<&[u8], Error> {
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
        offset += 1;
        let x = buf[offset] as i32;

        if i == 0 {
            if x & 0x80 > 0 {
                signed == true;
            }

            output |= x & 0x3F;

            if x & 0x40 == 0 {
                break
            }
        }
        if i == 4 {
            output |= (x & 0x1F) << (6 + (3 * 7));
        } else {
            output |= (x & 0x7F) << (6 + ((i - 1) * 7));
            if x & 0x80 == 0 {
                break;
            }
        }
        size += 1
    }

    if signed { output *= -1; }
    
    (output, size)
}

#[test]
fn test1() {
    // let a = UMXFile::load_module("./test/umx/UNATCO_Music.umx");
    let a: _ = UMXFile::load_module("./test/umx/MJ12_Music.umx");

}