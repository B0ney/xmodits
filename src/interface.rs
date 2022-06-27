use std::path::Path;
use std::fs;
use crate::utils::Error;
pub type DumperObject = Box<dyn TrackerDumper>;

pub trait TrackerDumper {
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path> 
        {
            let buf = fs::read(path)?;
            Self::load_from_buf(buf)
        }
    fn load_from_buf(buf: Vec<u8>) -> Result<DumperObject, Error>
        where Self: Sized;
    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error>;
    fn number_of_samples(&self) -> usize;
    fn dump(&self);
}

// pub trait Sample {
//     fn name();
//     fn file_name();

//     fn sample_rate();
// }