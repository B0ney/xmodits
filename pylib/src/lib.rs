mod error;
mod api;
use error::XmError;
use pyo3::prelude::*;

#[pyfunction]
fn dump(
    path: String,                   // Path to tracker module
    destination: String,            // folder to place dump
    
    index_raw: Option<bool>,        // Preserve sample number
    index_padding: Option<usize>,   // Set sample number padding
    index_only: Option<bool>,       // Only name sample by their number
    with_folder: Option<bool>,      // create new folder
) -> PyResult<()> {
    api::rip(path, destination, index_raw, index_padding, index_only, with_folder)
        .map_err(|e| XmError(e).into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn xmodits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    Ok(())
}