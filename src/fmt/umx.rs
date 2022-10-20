use crate::XmoditsError;
use crate::tracker_formats::*;
use crate::{xm::XMFile, utils::prelude::*};


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
            return Err(XmoditsError::unsupported("UMX versions below 61 are unsupported"))
        }

        let export_count = read_u32_le(buf, 0x0014);
        if export_count > 1 {
            return Err(XmoditsError::unsupported("Unreal package contains more than 1 entry."));
        }

        let name_count = read_u32_le(buf, 0x000C) as usize;
        let name_offset = read_u32_le(buf, 0x0010) as usize;
        
        let export_offset = read_u32_le(buf, 0x0018);
        let import_count = read_u32_le(buf, 0x001C);
        let import_offset = read_u32_le(buf, 0x0020) & 0x00ff;

        dbg!(version);
        dbg!(name_count);
        dbg!(name_offset);
        dbg!(export_count);
        dbg!(export_offset);
        dbg!(import_count);
        dbg!(import_offset);

        // obtain names
        let mut name_table: Vec<String> = Vec::with_capacity(name_count);
        let mut offset = name_offset;

        let length: usize = buf[offset + 1] as usize;
        dbg!(length);
        dbg!(read_string(buf, offset, length - 2));
        // for i in 0..name_count  {

            
        //     name_table.push()
        // }

        let mut offset: usize = 0;

        
        
        

        Ok(())
    }

    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
    {
        Self::validate(&buf)?;
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

#[test]
fn test1() {
    let a = UMXFile::load_module("./test/umx/UNATCO_Music.umx");
}