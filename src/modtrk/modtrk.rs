use std::path::Path;
use std::fs;
use byteorder::{BE,ByteOrder};
use crate::utils::{Error, wav};

pub struct MODFile {

}
impl MODFile {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let buffer: Vec<u8> = fs::read(path)?;
        Ok(Self{})

    }
}


pub struct MODSample {
    name: [u8; 22],
    length: u16,    // multiply by 2 to get length in bytes
}