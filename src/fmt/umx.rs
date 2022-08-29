use crate::{it::ITFile, s3m::S3MFile, amig_mod::MODFile};
use crate::{xm::XMFile, utils::prelude::*};

const UM_MAGIC_NUMBER: u32 = 0x9E2A83C1;
struct DontUseMe;
pub struct UMXFile(DontUseMe);

use crate::interface::{TrackerDumper, TrackerModule};

impl TrackerDumper for UMXFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
    {
        Self::validate(&buf)?;
        // identify header to verify it is a um* package
        

        // if possible determine format from header info

        // otherwise, use tests such as using magic numbers.


        // figure out what kind of module it contains
        // strip umx header from buffer
        // 

        match 2 {
            1 => ITFile::load_from_buf(buf),
            2 => S3MFile::load_from_buf(buf),
            3 => MODFile::load_from_buf(buf),
            4 => XMFile::load_from_buf(buf),
            _ => Err("Could not find module in UMX".into()),
        }
    }  

    fn validate(buf: &[u8]) -> Result<(), Error> {
        if buf.len() < 69 // for now
            || read_u32_le(&buf, 0x0000) != UM_MAGIC_NUMBER 
        {
            return Err("Not a valid Unreal package".into());
        }
        let export_count = read_u32_le(&buf, 0x0014);

        if export_count > 1 {
            return Err("Unreal Package contains more than 1 entry.".into());
        }
        Ok(())
    }

    /*  You should not call these methods from UMX (should be impossible).
        But incase someone somehow manages to do so, panic :) */ 
    fn export(&self, _: &dyn AsRef<Path>, _: usize) -> Result<(), Error> {
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
    fn write_wav(&self, smp: &crate::TrackerSample, file: &PathBuf) -> Result<(), Error> {
        unimplemented!()
    }  
}