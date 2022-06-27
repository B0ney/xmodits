use crate::it::ITFile;
use crate::s3m::S3MFile;
use crate::modtrk::MODFile;
use crate::xm::XMFile;
use crate::utils::prelude::*;

pub struct UMXFile(DumperObject);

use crate::interface::{TrackerDumper, DumperObject};

impl TrackerDumper for UMXFile {
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path> 
    {
        let buf = fs::read(path)?;

        // figure out what kind of tracker module is in the container
        // then we need to strip the header 
        // and let the appropriate module handle it

        match 0 {
            1 => ITFile::load_from_buf(buf),
            2 => S3MFile::load_from_buf(buf),
            3 => MODFile::load_from_buf(buf),
            4 => XMFile::load_from_buf(buf),
            _ => Err("Could not find module in UMX".into()),
        };

        todo!()
    }

    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        self.0.export(path, index)
    }

    fn number_of_samples(&self) -> usize {
        self.0.number_of_samples()
    }

    fn dump(&self) {
        self.0.dump()
    }

    fn load_from_buf(buf: Vec<u8>) -> Result<DumperObject, Error>
        where Self: Sized  
    {
        // temporary
        Err("Call \"load_module\" instead".into())
        // todo!()
    }
}
