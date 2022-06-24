use std::path::Path;

use crate::utils::Error;

pub type DumperObject = Box<dyn TrackerDumper>;

pub trait TrackerDumper {
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path>;
    fn export(&self, path: &dyn AsRef<Path>, index: usize) -> Result<(), Error>;
    fn number_of_samples(&self) -> usize;
    fn dump(&self);
}

// pub trait Sample {
//     fn name();
//     fn file_name();

//     fn sample_rate();
// }