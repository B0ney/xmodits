use xmodits_lib::XmoditsError as _XmoditsError;
use pyo3::{exceptions::PyTypeError, import_exception, PyErr, PyObject};

/// Wrapper for xmodits error struct.
/// This is done to implement errors.
pub struct XmoditsError(pub _XmoditsError);

impl From<XmoditsError> for PyErr {
    fn from(e: XmoditsError) -> Self {
        use _XmoditsError::*;

        match e.0 {
            SampleExtractionFailure(e) => {
                PyErr::new::<PyTypeError, _>(format!("Failed to rip sample: {}", e))
            },
            UnsupportedFormat(e) => {
                PyErr::new::<PyTypeError, _>(format!("{}", e))
            },
            InvalidModule(e) => {
                PyErr::new::<PyTypeError, _>(format!("{}", e))
            },
            IoError(_) => todo!(),
            FileError(_) => todo!(),

            EmptyModule => {
                PyErr::new::<PyTypeError, _>("Module has no samples")
            },
            GenericError(e) => {
                PyErr::new::<PyTypeError, _>(format!("{}", e))
            },
        }
    }
}