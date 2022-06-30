mod test;
mod deltadecode;
use crate::utils::prelude::*;

pub struct XMSample {}

pub struct XMFile {}

use crate::interface::{TrackerDumper, TrackerModule};

impl TrackerDumper for XMFile {
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized 
    {
        todo!()
    }

    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        todo!()
    }

    fn number_of_samples(&self) -> usize {
        todo!()
    }

    fn module_name(&self) -> &String {
        todo!()
    }
}

#[test]
fn gen_offset(){
    let offset = [
        0, 17, 37, 38, 58,
        60
    ];
    let offset2 = [
        4, 5,6,10,12,14,16,18,20,0
    ];

    for i in offset {
        println!("0x{:04X} => ", i);
    }
    for i in offset2 {
        println!("0x{:04X} => ", i + 60);
    }
}