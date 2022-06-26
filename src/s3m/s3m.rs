use crate::utils::prelude::*;
use byteorder::{ByteOrder, BE, LE};

pub struct S3MSample {}

pub struct S3MFile {}


use crate::interface::{TrackerDumper, DumperObject};

impl TrackerDumper for S3MFile {
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path> {
        todo!()
    }

    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        todo!()
    }

    fn number_of_samples(&self) -> usize {
        todo!()
    }

    fn dump(&self) {
        todo!()
    }
}