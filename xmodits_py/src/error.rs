use xmodits_lib::XmoditsError as _XmoditsError;
use pyo3::{exceptions::{PyTypeError, PyIOError}, import_exception, PyErr, PyObject};

macro_rules! batch_create_exceptions {
    ($($EXCEPTION:ident) *) => {
        $(
            pyo3::create_exception!(xmodits, $EXCEPTION, pyo3::exceptions::PyException);
        )*  
    };
}

batch_create_exceptions!(
    SampleExtractionError
    UnsupportedFormatError
    InvalidModuleError
    EmptyModuleError
    Generic
);

pub struct XmError(pub _XmoditsError);

impl From<XmError> for PyErr {
    fn from(e: XmError) -> Self {
        use _XmoditsError::*;

        match e.0 {
            SampleExtractionFailure(e) => {
                PyErr::new::<SampleExtractionError, _>(format!("Failed to rip sample: {}", e))
            },
            UnsupportedFormat(e) => {
                PyErr::new::<UnsupportedFormatError, _>(format!("{}", e))
            },
            InvalidModule(e) => {
                PyErr::new::<InvalidModuleError, _>(format!("{}", e))
            },
            IoError(e) => PyErr::new::<PyIOError, _>(format!("{:?}", e.to_string())),
            FileError(_) => todo!(),

            EmptyModule => {
                PyErr::new::<EmptyModuleError, _>("Module has no samples")
            },
            GenericError(e) => {
                PyErr::new::<Generic, _>(format!("{}", e))
            },
        }
    }
}