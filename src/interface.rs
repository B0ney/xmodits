use std::path::{Path, PathBuf};
use std::fs;
use crate::XmoditsError;
use crate::utils::Error;
use crate::utils::prelude::Wav;
pub type TrackerModule = Box<dyn TrackerDumper>;

/// Function type signature to flexibly format sample names.
#[cfg(not(feature="thread"))]
pub type SampleNamerFunc = dyn Fn(&TrackerSample, usize) -> String;
#[cfg(feature="thread")]
pub type SampleNamerFunc = dyn Fn(&TrackerSample, usize) -> String + Sync + Send;

#[derive(Default, Debug)]
pub struct TrackerSample {
    /// Sample name
    pub name: String,
    /// Sample filename
    pub filename: String,
    /// You should to call ```index()``` instead as this value is zero indexed.
    pub raw_index: usize,          
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
    /// Is the stereo sample data interleaved?
    pub is_interleaved: bool,
    /// Can the sample data be read directly?
    pub is_readable: bool,
}

impl TrackerSample {
    /// Return both Start & End pointers to sample data as a range.
    pub fn ptr_range(&self) -> std::ops::Range<usize> {
        self.ptr..(self.ptr + self.len)
    }
    /// Return Sample's index as if it's listed in a tracker module.
    pub fn raw_index(&self) -> usize {
        self.raw_index + 1
    }
}

pub trait TrackerDumper {
    /// Load tracker module from memory
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
        where Self: Sized;

    /// Check if tracker module is valid
    fn validate(buf: &[u8]) -> Result<(), Error>
        where Self: Sized;

    /// export sample given index
    fn export(&mut self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        self.export_advanced(folder, index, &crate::utils::prelude::name_sample)
    }

    fn export_advanced(
        &mut self,
        folder: &dyn AsRef<Path>,
        index: usize,
        name_sample: &SampleNamerFunc ) -> Result<(), Error>
    {
        let sample: &TrackerSample  = &self.list_sample_data()[index];
        let file: PathBuf           = PathBuf::new()
            .join(folder)
            .join(name_sample(sample, index));

        self.write_wav(&file, index)
    }

    /// Number of samples a tracker module contains
    fn number_of_samples(&self) -> usize;

    /// Name of tracker module
    fn module_name(&self) -> &str;

    /// List tracker sample infomation
    fn list_sample_data(&self) -> &[TrackerSample];

    /// Write sample data to PCM
    fn write_wav(&mut self, file: &Path, index: usize) -> Result<(), Error> {
        let smp =  &self.list_sample_data()[index];

        Wav::header(smp.rate, smp.bits, smp.len as u32, smp.is_stereo, smp.is_interleaved)
            .write_ref(file, self.pcm(index)?)
    }

    /// return reference to readable pcm data
    fn pcm(&mut self, index: usize) -> Result<&[u8], Error>;

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
            return Err(
                XmoditsError::file("File provided is larger than 64MB. No tracker module should ever be close to that")
            );
        }
        
        let buf: Vec<u8> = fs::read(&path)?;
        Self::load_from_buf(buf)
    }

    /// Dump all samples to a folder
    fn dump(&mut self, folder: &dyn AsRef<Path>, create_dir_if_absent: bool) -> Result<(), Error> 
    {
        self.dump_advanced(folder, &crate::utils::prelude::name_sample, create_dir_if_absent)
    }

    /// Dump all samples with the added ability to format sample names to our likinng.
    fn dump_advanced(
        &mut self,
        folder: &dyn AsRef<Path>,
        sample_namer_func: &SampleNamerFunc,
        create_dir_if_absent: bool,
    ) -> Result<(), Error>  
    {
        if self.number_of_samples() == 0 {
            return Err(XmoditsError::EmptyModule);
        }

        if !&folder.as_ref().is_dir() {
            if create_dir_if_absent {
                fs::create_dir(&folder)
                    .map_err(|err| helpful_io_error(err, folder.as_ref()))?;
            } else {
                return Err(
                    XmoditsError::file(
                        &format!("Destination '{}' either doesn't exist or is not a directory", folder.as_ref().display())
                    )
                );
            }
        }
        
        for i in 0..self.number_of_samples() {
            self.export_advanced(&folder, i, sample_namer_func)?;
        }

        Ok(())
    }
}

fn helpful_io_error(err: std::io::Error, folder: &Path) -> XmoditsError {
    XmoditsError::file(
        &format!("Could not create folder '{}'{}", 
            folder.display(),
            
            match err.kind() {
                std::io::ErrorKind::NotFound => format!(".\nMake sure directory '{}' exists.", 
                    match folder.ancestors().nth(1) {
                        Some(p) => format!("{}", p.display()),
                        _ => String::from("")
                    }
                ),
                _ => format!(" {}", err)
            },
        )
    )
}