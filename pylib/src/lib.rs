mod error;
use error::XmError;
use pyo3::prelude::*;
use xmodits_lib::*;

#[pyfunction]
fn dump(
    path: String,                           // Path to tracker module
    destination: String,                    // folder to place dump
    preserve_sample_number: Option<bool>,   // Preserve sample number
    with_folder: Option<bool>,              // create new folder
    no_padding: Option<bool>,
    number_only: Option<bool>
) -> PyResult<()> {
    rip(path, destination)
        .map_err(|e| XmError(e).into())

}

/// A Python module implemented in Rust.
#[pymodule]
fn xmodits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    Ok(())
}

// xmodits.dump("../samples/s3m/arc-cell.s3m", "/ii/")
fn rip(path: String, destination: String) -> Result<(), Error> {
    let modname: String = std::path::Path::new(&path).to_str().unwrap().replace(".", "_");
    let namer = SampleNamer{};

    xmodits_lib::load_module(path)?
        .dump_with_sample_namer(
            &destination,
            &namer.func_builder()
        )
}

struct SampleNamer {

}

impl SampleNamer {
    /// dynamically build a function to format sample names given its internal parameters
    /// Ideas:
    /// Add recursion?
    /// match statements
    fn func_builder(&self) -> Box<SampleNamerFunc> {
        Box::new(|smp, _| format!("{}.wav", smp.raw_index()))
    }
}