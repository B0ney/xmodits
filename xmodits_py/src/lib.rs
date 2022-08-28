mod error;
use error::XmError;

use pyo3::prelude::*;
use xmodits_lib::*;

#[pyfunction]
fn dump(
    // Path to tracker module
    path: String,

    // folder to place dump
    destination: String,

    // Preserve sample number
    preserve_sample_number: Option<bool>,
    
    // 
    no_folder: Option<bool>,

    no_padding: Option<bool>,

    number_only: Option<bool>,

) -> PyResult<()> {
    
    rip(path).map_err(|e| XmError(e).into())
    // Err(Error(xmodits_lib::XmoditsError::EmptyModule).into())
    // Ok(())
}

fn rip(path: String) -> Result<(), Error> {
    let _ = xmodits_lib::tracker_formats::S3MFile::load_module(path)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn xmodits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    Ok(())
}