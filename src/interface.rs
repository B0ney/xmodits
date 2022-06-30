use std::path::{Path, PathBuf};
use std::fs;
use crate::utils::Error;
pub type TrackerModule = Box<dyn TrackerDumper + Send + Sync>;
pub trait TrackerDumper {
    /// Load tracker module from memory
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized;
    // export sample given index
    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error>;
    /// Number of samples a tracker module contains
    fn number_of_samples(&self) -> usize;
    /// Name of tracker module
    fn module_name(&self) -> &String;
    /// Load tracker module from given path
    fn load_module<P>(path: P) -> Result<TrackerModule, Error> 
        where Self: Sized, P: AsRef<Path> 
        {
            let buf = fs::read(path)?;
            Self::load_from_buf(buf)
        }
    /// Dump all samples
    fn dump(&self, folder: &dyn AsRef<Path>) -> Result<(), Error> 
    {
        if !&folder.as_ref().is_dir() 
        {
            return Err("folder provided either doesn't exist or is not a directory".into());
        }
        // Create root folder
        // TODO: default to module filename if module name is empty
        let root: PathBuf = PathBuf::new()
            .join(folder).join(self.module_name());
    
        if root.exists() {
            return Err(format!("Folder Already exists: '{}'", root.display()).into());
        }
        std::fs::create_dir(&root)?;
        
        for i in 0..Self::number_of_samples(&self) {
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