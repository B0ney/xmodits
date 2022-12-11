/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use crate::loader::load_to_buf;
use crate::utils::prelude::Wav;
use crate::utils::Error;
use crate::XmoditsError;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(feature = "thread")]
pub type TrackerModule = Box<dyn TrackerDumper + Sync + Send>;
#[cfg(not(feature = "thread"))]
pub type TrackerModule = Box<dyn TrackerDumper>;

/// Function type signature to flexibly format sample names.
#[cfg(not(feature = "thread"))]
pub type SampleNamerFunc = dyn Fn(&TrackerSample, usize) -> String;
#[cfg(feature = "thread")]
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
    /// sample loop start
    pub loop_start: u32,
    /// sample loop end
    pub loop_end: u32,
    /// Is sample stereo?
    pub is_stereo: bool,
    /// Is sample compressed?
    pub is_compressed: bool,
    /// Is the stereo sample data interleaved?
    pub is_interleaved: bool,
    /// Can the sample data be read directly?
    pub is_readable: bool,
    /// What type of looping does this sample use?
    pub loop_type: LoopType,
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

#[derive(Default, Debug)]
pub enum LoopType {
    #[default]
    Off = -1,
    Forward = 0,
    PingPong = 1,
    Reverse = 3,
}

pub trait TrackerDumper {
    /// Load tracker module from memory
    /// Validates headers.
    fn load_from_buf(buf: Vec<u8>) -> Result<TrackerModule, Error>
    where
        Self: Sized,
    {
        Self::validate(&buf)?;
        Self::load_from_buf_unchecked(buf)
    }

    /// Load tracker module from memory.
    ///
    /// Can panic if used without any form of external validation
    fn load_from_buf_unchecked(buf: Vec<u8>) -> Result<TrackerModule, Error>
    where
        Self: Sized;

    /// Check if a tracker module is valid
    fn validate(buf: &[u8]) -> Result<(), Error>
    where
        Self: Sized;

    /// export sample given index
    fn export(&mut self, folder: &dyn AsRef<Path>, index: usize) -> Result<(), Error> {
        self.export_advanced(folder, index, &crate::utils::prelude::name_sample, false)
    }

    fn export_advanced(
        &mut self,
        folder: &dyn AsRef<Path>,
        index: usize,
        name_sample: &SampleNamerFunc,
        with_loop_points: bool,
    ) -> Result<(), Error> {
        let sample: &TrackerSample = &self.list_sample_data()[index];
        let file: PathBuf = PathBuf::new().join(folder).join(name_sample(sample, index));

        self.write_wav(&file, index, with_loop_points)
    }

    /// Number of samples a tracker module contains
    fn number_of_samples(&self) -> usize;

    /// Name of tracker module
    fn module_name(&self) -> &str;

    fn format(&self) -> &str;

    /// List tracker sample infomation
    fn list_sample_data(&self) -> &[TrackerSample];

    /// Write sample data to PCM
    fn write_wav(
        &mut self,
        file: &Path,
        index: usize,
        with_loop_points: bool,
    ) -> Result<(), Error> {
        let smp = &self.list_sample_data()[index];

        Ok(Wav::from_tracker_sample(smp).write_ref(file, self.pcm(index)?, with_loop_points)?)
    }

    /// return reference to readable pcm data
    fn pcm(&mut self, index: usize) -> Result<&[u8], Error>;

    // Load tracker module from given path
    fn load_module<P>(path: P) -> Result<TrackerModule, Error>
    where
        Self: Sized,
        P: AsRef<Path>,
    {
        Self::load_from_buf(load_to_buf(path)?)
    }

    /// Dump all samples to a folder
    fn dump(&mut self, folder: &dyn AsRef<Path>, create_dir_if_absent: bool) -> Result<(), Error> {
        self.dump_advanced(
            folder,
            &crate::utils::prelude::name_sample,
            create_dir_if_absent,
            false,
        )
    }

    /// Dump all samples with the added ability to format sample names to our likinng.
    fn dump_advanced(
        &mut self,
        folder: &dyn AsRef<Path>,
        sample_namer_func: &SampleNamerFunc,
        create_dir_if_absent: bool,
        with_loop_points: bool,
    ) -> Result<(), Error> {
        if self.number_of_samples() == 0 {
            return Err(XmoditsError::EmptyModule);
        }

        if !&folder.as_ref().is_dir() {
            if create_dir_if_absent {
                fs::create_dir(folder).map_err(|err| helpful_io_error(err, folder.as_ref()))?;
            } else {
                return Err(XmoditsError::file(&format!(
                    "Destination '{}' either doesn't exist or is not a directory",
                    folder.as_ref().display()
                )));
            }
        }

        for i in 0..self.number_of_samples() {
            self.export_advanced(&folder, i, sample_namer_func, with_loop_points)?;
        }

        Ok(())
    }
}

fn helpful_io_error(err: std::io::Error, folder: &Path) -> XmoditsError {
    XmoditsError::file(&format!(
        "Could not create folder '{}'{}",
        folder.display(),
        match err.kind() {
            std::io::ErrorKind::NotFound => format!(
                ".\nMake sure directory '{}' exists.",
                match folder.ancestors().nth(1) {
                    Some(p) => format!("{}", p.display()),
                    _ => String::from(""),
                }
            ),
            _ => format!(" {}", err),
        },
    ))
}
