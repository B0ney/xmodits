mod error;
use std::path::{Path, PathBuf};
use error::XmError;
use pyo3::prelude::*;
use xmodits_lib::*;

#[pyfunction]
fn dump(
    path: String,                   // Path to tracker module
    destination: String,            // folder to place dump
    raw_index: Option<bool>,        // Preserve sample number
    with_folder: Option<bool>,      // create new folder
    padding: Option<bool>,
    number_only: Option<bool>
) -> PyResult<()> {
    rip(path, destination, raw_index, with_folder, padding, number_only)
        .map_err(|e| XmError(e).into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn xmodits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    Ok(())
}

fn rip(
    path: String,
    destination: String,
    preserve_sample_number: Option<bool>,
    with_folder: Option<bool>,
    padding: Option<bool>,
    number_only: Option<bool>
) -> Result<(), Error> {  
    let namer: SampleNamer = SampleNamer{
        number_only,
        padding,
        raw_index: preserve_sample_number,
        with_folder,
    };

    let destination: PathBuf = match namer.with_folder {
        Some(true) => {
            let modname: String = Path::new(&path).file_name().unwrap().to_str().unwrap().replace(".", "_");
            let new_folder: PathBuf = PathBuf::new().join(&destination).join(modname);

            new_folder
        },
        _ => PathBuf::new().join(&destination),
    };

    let create_if_absent: bool = namer.with_folder.is_some();

    xmodits_lib::load_module(path)?
        .dump_with_sample_namer(
            &destination,
            &namer.to_func(),
            create_if_absent
        )
}

#[derive(Default)]
struct SampleNamer {
    with_folder: Option<bool>,
    number_only: Option<bool>,
    padding: Option<bool>,
    raw_index: Option<bool>,
}

impl SampleNamer {
    /// Dynamically build a function to format sample name given its internal parameters
    /// 
    // Kinda ugly, but it's beautiful in its own right
    fn to_func(self) -> Box<SampleNamerFunc> {
        Box::new({
            move |smp: &TrackerSample, idx: usize| {
                format!(
                    "{}{}.wav",
                    {
                        let index = match self.raw_index {
                            Some(true) => smp.raw_index(),
                            _ => idx,
                        };
                        match self.padding {
                            Some(false) => format!("{}", index),
                            _ => format!("{:02}", index),
                        }
                    },
                    match self.number_only {
                        Some(true) => "".to_string(),
                        _ => match smp.filename.trim() {
                            x if x.is_empty() => "".to_string(),
                            x => format!(
                                " - {}", 
                                x.replace(".wav", "").replace(".", "_")
                            ),
                        }
                    }
                )
            }
        })
    }
}