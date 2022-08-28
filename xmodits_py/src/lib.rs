mod error;
use error::XmoditsError;

use pyo3::prelude::*;
use xmodits_lib;

#[pyfunction]
fn dump(
    path: String,
    arg: Option<bool>
) -> PyResult<()> {

    if let Some(true) = arg {
        println!("nice");
    } 

    Err(XmoditsError(xmodits_lib::XmoditsError::EmptyModule).into())
    // Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn xmodits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    Ok(())
}