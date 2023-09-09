use super::global_tracker::{GlobalTracker, GLOBAL_TRACKER};
use crate::dialog;
use std::{panic::PanicInfo, path::PathBuf};

#[derive(Default, Debug)]
struct Dump {
    pub current_path: Option<PathBuf>,
    pub batch_size: usize,
    pub batch_number: u64,
    pub sub_batch_size: usize,
    pub sub_batch_number: u64,
    pub location: Option<Location>,
    pub message: Option<String>,
}

#[derive(Debug)]
struct Location {
    pub line: u32,
    pub file: String,
}

impl Dump {
    fn from_panic(panic_info: &PanicInfo) -> Self {
        let global_tracker: &GlobalTracker = &GLOBAL_TRACKER;

        let location = panic_info.location().map(|file| Location {
            line: file.line(),
            file: file.file().to_owned(),
        });

        let message: Option<String> = match panic_info.payload().downcast_ref::<String>() {
            Some(e) => Some(e.to_string()),
            None => panic_info
                .payload()
                .downcast_ref::<&str>()
                .map(|err| err.to_string()),
        };

        Self {
            location,
            message,
            ..Self::from(global_tracker)
        }
    }
}

impl From<&GlobalTracker> for Dump {
    fn from(value: &GlobalTracker) -> Self {
        Self {
            current_path: value.current_path(),
            batch_size: value.batch_size(),
            batch_number: value.batch_number(),
            sub_batch_size: value.sub_batch_size(),
            sub_batch_number: value.sub_batch_number(),
            ..Default::default()
        }
    }
}

/// Provide human friendly crash reporting
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let dump = Dump::from_panic(panic_info);

        // let backtrace = std::backtrace::Backtrace::force_capture();
        let info = match &dump.location {
            Some(location) => format!(
                "Panic occurred in file '{}' at line {}",
                location.file, location.line,
            ),
            None => String::from("Panic occurred but can't get location information..."),
        };

        let message: String = match &dump.message {
            Some(e) => e.into(),
            None => "Panic occured".into(),
        };

        dialog::critical_error(&format!("{}\n{:?}", info, message));
        dbg!("{:?}", &dump);

        std::process::exit(1)
    }));
}
