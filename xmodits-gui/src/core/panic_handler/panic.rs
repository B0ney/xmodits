use crate::core::{dialog, track::GLOBAL_TRACKER};
use std::path::PathBuf;

pub enum Source {
    XmoditsLIB,
    XmoditsGUI,
    Other(String),
}

pub struct Crash {
    source: Source,
    bad_module: Option<BadModule>,
    location: Location,
}

enum Location {
    Unknown,
    Known { location: String, line: u32 },
}

pub enum BadModule {
    Exact(PathBuf),
    Suspects {
        traversed_entries: PathBuf,
        offset: u64,
        window: u64,
    },
}

impl BadModule {}

fn a() {
    let a = &GLOBAL_TRACKER;
    let batch_offset: u64 = a.get_batch_size() as u64 * a.get_batch_number();
    let sub_batch_size = a.get_sub_batch_size() as u64;
    let sub_batch_offset: u64 =  sub_batch_size * a.get_sub_batch_number();

    let offset: u64 = batch_offset + sub_batch_offset;
    let window = sub_batch_size;
    
}

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let backtrace = std::backtrace::Backtrace::force_capture();
        let info = match panic_info.location() {
            Some(location) => format!(
                "Panic occurred in file '{}' at line {}",
                location.file(),
                location.line()
            ),
            None => String::from("Panic occurred but can't get location information..."),
        };

        let message: String = match panic_info.payload().downcast_ref::<String>() {
            Some(e) => e.into(),
            None => match panic_info.payload().downcast_ref::<&str>() {
                Some(err) => err.to_string(),
                None => "Panic occured".into(),
            },
        };

        dialog::critical_error(&format!("{}\n{:?}", info, message));

        std::process::exit(1)
    }));
}
