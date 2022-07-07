use std::path::{Path, PathBuf};
use std::fs;
use crate::utils::Error;

pub type TrackerModule = Box<dyn TrackerDumper>;

pub trait TrackerDumper {

    /// Load tracker module from memory
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized;

    // check if tracker module is valid
    // fn is_valid(buf: &[u8]) -> Result<(), Error>

    // export sample given index
    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error>;

    /// Number of samples a tracker module contains
    fn number_of_samples(&self) -> usize;

    // fn set_module_filename(&mut self, path: &dyn AsRef<Path>);

    /// Name of tracker module
    fn module_name(&self) -> &String;

    /// Load tracker module from given path
    fn load_module<P>(path: P) -> Result<TrackerModule, Error> 
        where Self: Sized, P: AsRef<Path> 
        {
            /*
                Tracker modules are frickin' tiny.
                We can get away with loading it to memory directly
                rather than using Seek.
                
                This allows us to access specific locations with offsets,
                which makes everything easier to read and debug (hopefully).
                
                At any point should we consider optimizing the code,
                using Seek *may* help performance...(At the cost of readability)

                But this performance increase would mostly take effect with large files.

                The largest tracker module iirc is ~21MB. 
                The average tracker module (IT, XM, S3M) is < 2MB.

                For large scale dumping in parallel, using Seek will be considered.
            */
            
            let buf: Vec<u8> = fs::read(&path)?;
            Self::load_from_buf(buf)
        }

    /// Dump all samples
    fn dump(&self, folder: &dyn AsRef<Path>, module_name: &String) -> Result<(), Error> 
    {
        if !&folder.as_ref().is_dir() 
        {
            return Err("folder provided either doesn't exist or is not a directory".into());
        }
        // Create root folder

        // Name of exported folder is the name of the module file
        // Until I can come up with a way to abstract the module name,
        // The api caller must specify folder name
        let root: PathBuf = PathBuf::new()
            .join(folder).join(module_name);
    
        if root.exists() {
            return Err(format!("Folder Already exists: '{}'", root.display()).into());
        }
        std::fs::create_dir(&root)?;
        
        for i in 0..self.number_of_samples() {
            self.export(&root, i)?;
        }

        Ok(())
    }
}