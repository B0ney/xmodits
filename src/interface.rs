use std::path::{Path, PathBuf};
use std::fs;
use crate::utils::Error;
pub type DumperObject = Box<dyn TrackerDumper>;

pub trait TrackerDumper {
    /// Load tracker module from given path
    /// 
    /// This is implemented automatically
    fn load_module<P>(path: P) -> Result<DumperObject, Error> 
        where Self: Sized, P: AsRef<Path> 
        {
            
            let buf = fs::read(path)?;
            Self::load_from_buf(buf)
        }
    /// Load tracker module from memory
    fn load_from_buf(buf: Vec<u8>) -> Result<DumperObject, Error>
        where Self: Sized;
    // export sample given index
    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error>;
    /// Number of samples a tracker module contains
    fn number_of_samples(&self) -> usize;
    /// Name of tracker module
    fn module_name(&self) -> &String;
    /// Dump all samples
    /// 
    /// Automatically implemented
    fn dump(&self, folder: &dyn AsRef<Path>) -> Result<(), Error> 
    {
        if !&folder.as_ref().is_dir() 
        {
            return Err("folder provided either doesn't exist or is not a directory".into());
        }
        // Create root folder
        let root: PathBuf = PathBuf::new()
            .join(folder).join(self.module_name());
            
        // println!("{}",&self.module_name());
        if root.exists() {
            return Err(format!("Folder Already exists: '{}'", root.display()).into());
        }
        std::fs::create_dir(&root)?;
        
        for i in 0..Self::number_of_samples(&self){
            self.export(&root, i)?;
        }

        Ok(())
    }
}

// pub trait Sample {
//     fn name();
//     fn file_name();

//     fn sample_rate();
// }