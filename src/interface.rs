use std::path::{Path, PathBuf};
use std::fs;
use crate::utils::Error;

pub type TrackerModule = Box<dyn TrackerDumper>;

#[derive(Default, Debug)]
pub struct TrackerSample {
    /// Sample name
    pub name: String,
    /// Sample filename
    pub filename: String,
    /// You should to call ```index()``` instead as this value is zero indexed.
    pub index: usize,          
    /// Sample length in BYTES
    pub len: usize,             
    /// Sample pointer
    pub ptr: usize,             
    /// Sample flags
    pub flags: u8,             
    /// Bits per sample
    pub bits: u8,               
    /// Sample rate
    pub rate: u32,              
    /// Is sample stereo?
    pub is_stereo: bool,        
    /// Is sample compressed?
    pub is_compressed: bool,    
}

impl TrackerSample {
    /// Return both Start & End pointers to sample data as a range.
    pub fn ptr_range(&self) -> std::ops::Range<usize> {
        self.ptr..(self.ptr + self.len)
    }
    /// Return Sample's index as if its listed in a tracker module.
    pub fn index(&self) -> usize {
        self.index + 1
    }
}

pub trait TrackerDumper {
    /// Load tracker module from memory
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized;

    // check if tracker module is valid
    fn validate(buf: &[u8]) -> Result<(), Error>
        where Self: Sized;

    // export sample given index
    fn export(&self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error>;

    /// Number of samples a tracker module contains
    fn number_of_samples(&self) -> usize;

    /// Name of tracker module
    fn module_name(&self) -> &str;

    /// List tracker sample infomation
    fn list_sample_data(&self) -> &[TrackerSample];

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
            if std::fs::metadata(&path)?.len() > 1024 * 1024 * 64 {
                return Err("File provided is larger than 64MB. No tracker module should ever be close to that".into());
            }
            
            let buf: Vec<u8> = fs::read(&path)?;
            Self::load_from_buf(buf)
        }

    /// Dump all samples
    fn dump(&self, folder: &dyn AsRef<Path>, module_name: &str) -> Result<(), Error> 
    {
        if self.number_of_samples() == 0 {
            return Err("Module has no samples".into());
        }

        if !&folder.as_ref().is_dir() {
            return Err("folder provided either doesn't exist or is not a directory".into());
        }

        // Create root folder
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