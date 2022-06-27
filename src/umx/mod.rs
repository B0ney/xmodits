use crate::it::ITFile;
use crate::s3m::S3MFile;
use crate::modtrk::MODFile;
use crate::xm::XMFile;
use crate::utils::prelude::*;

pub struct UMXFile;

use crate::interface::{TrackerDumper, DumperObject};

impl TrackerDumper for UMXFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<DumperObject, Error>
        where Self: Sized  
    {
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

    /*  It should be **impossible** to call these methods from UMX.
        But incase someone somehow manages to do so, panic :) */ 
    fn export(&self, _: &dyn AsRef<Path>, _: usize) -> Result<(), Error> {
        unimplemented!()
    }
    fn number_of_samples(&self) -> usize {
        unimplemented!()
    }
    fn dump(&self) {
        unimplemented!()
    }    
}