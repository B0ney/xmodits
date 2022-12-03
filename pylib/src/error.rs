use pyo3::exceptions::PyIOError;
use pyo3::PyErr;
use xmodits_lib::XmoditsError as _XmoditsError;

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

impl From<&str> for XmError {
    fn from(e: &str) -> Self {
        Self(_XmoditsError::GenericError(e.to_owned()))
    }
}

impl From<_XmoditsError> for XmError {
    fn from(e: _XmoditsError) -> Self {
        Self(e)
    }
}
// TODO: add module name / path to error
impl From<XmError> for PyErr {
    fn from(e: XmError) -> Self {
        use _XmoditsError::*;

        match e.0 {
            SampleExtractionFailure(e) => {
                PyErr::new::<SampleExtractionError, _>(format!("Failed to rip sample: {}", e))
            }
            UnsupportedFormat(e) => PyErr::new::<UnsupportedFormatError, _>(e),
            InvalidModule(e) => PyErr::new::<InvalidModuleError, _>(e),
            IoError(e) => PyErr::new::<PyIOError, _>(e.to_string()),
            FileError(e) => PyErr::new::<PyIOError, _>(e),

            EmptyModule => PyErr::new::<EmptyModuleError, _>("Module has no samples"),
            GenericError(e) => PyErr::new::<Generic, _>(e),

            MultipleErrors(errors) => {
                PyErr::new::<Generic, _>(format!("multiple errors: {:#?}", errors))
            }
        }
    }
}
