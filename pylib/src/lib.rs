mod api;
mod error;
use error::XmError;
use pyo3::prelude::*;

/// Dump a single tracker
#[pyfunction]
fn dump(
    path: String,        // Path to tracker module
    destination: String, // folder to place dump
    index_raw: Option<bool>,      // Preserve sample number
    index_padding: Option<usize>, // Set sample number padding
    index_only: Option<bool>,     // Only name sample by their number
    with_folder: Option<bool>,    // create new folder
    upper: Option<bool>,
    lower: Option<bool>,
    hint: Option<String>
) -> PyResult<()> {
    dump_multiple(
        vec![path],
        destination,
        index_raw,
        index_padding,
        index_only,
        with_folder,
        upper,
        lower,
        hint
    )
}

/// Dump multiple trackers
#[pyfunction]
fn dump_multiple(
    path: Vec<String>,
    destination: String,
    index_raw: Option<bool>,
    index_padding: Option<usize>,
    index_only: Option<bool>,
    with_folder: Option<bool>,
    upper: Option<bool>,
    lower: Option<bool>,
    hint: Option<String>
) -> PyResult<()> {
    api::rip_multiple(
        path,
        destination,
        index_raw,
        index_padding,
        index_only,
        with_folder,
        upper,
        lower,
        hint
    )
    .map_err(|e| XmError(e).into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn xmodits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(dump_multiple, m)?)?;

    Ok(())
}
