mod test;
mod deltadecode;
use crate::utils::prelude::*;

pub struct XMSample {}

pub struct XMFile {}

use crate::interface::{TrackerDumper, DumperObject};

impl TrackerDumper for XMFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<DumperObject, Error>
        where Self: Sized 
    {
        todo!()
    }

    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        todo!()
    }

    fn number_of_samples(&self) -> usize {
        todo!()
    }

    fn module_name(&self) -> &String {
        todo!()
    }
}