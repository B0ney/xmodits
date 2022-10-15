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
            return Err("Not a valid Unreal package".into());
        }
        let export_count = read_u32_le(buf, 0x0014);

        if export_count > 1 {
            return Err("Unreal package contains more than 1 entry.".into());
        }
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