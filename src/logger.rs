pub mod crash_handler;
pub mod global_tracker;
pub mod history;

pub use crash_handler::set_panic_hook;
pub use global_tracker::GLOBAL_TRACKER;

use anyhow::Result;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Initialize application logging
pub fn init_logging() {
    use tracing::subscriber::set_global_default;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    #[cfg(windows)]
    reattach_windows_terminal();

    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
    // will be written to stdout.
    set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}

/// Writes Vec<(Path, Errors)> to a file.
///
/// Returns the path of the log file if successful.
///
/// IDEA: perhaps we should return the errors if there's a failure writing to the file?
pub fn write_error_log<E, P>(log_path: &Path, errors: Vec<(P, E)>) -> Result<PathBuf>
where
    E: std::fmt::Display,
    P: AsRef<Path>,
{
    let log_path: PathBuf = log_path.join(format!(
        "xmodits-error-log-{:04X}.txt",
        rand::thread_rng().gen::<u16>()
    ));

    let mut file = File::create(&log_path)?;
    errors.iter().for_each(|(path, error)| {
        let _ = file.write_all(path.as_ref().display().to_string().as_bytes());
        let _ = file.write_all(b"\n     ");
        let _ = file.write_all(error.to_string().as_bytes());
        let _ = file.write_all(b"\n\n");
        let _ = file.flush();
    });

    Ok(log_path)
}

/// ``WINDOWS ONLY``: Have the application write to the terminal even with
/// ``[windows_subsystem = "windows"]``
///
/// This allows logs to be displayed when launched from the terminal.
pub fn reattach_windows_terminal() {
    #[cfg(windows)]
    unsafe {
        use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
}
