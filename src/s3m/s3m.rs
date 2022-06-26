use crate::utils::prelude::*;
use byteorder::{ByteOrder, BE, LE};

pub struct S3MSample {
    smp_name: [char; 28],
    smp_ptr: u32,
    smp_len: u32,
    smp_stereo: bool,
    smp_bytes: u8,
}

pub struct S3MFile {
    title: [char; 28],
    smp_data: Vec<S3MSample>,
}


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