use anyhow::Result;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Async wrapper over ```write_error_log()```
pub async fn async_write_error_log(
    log_path: PathBuf,
    errors: Vec<(PathBuf, String)>,
) -> Result<PathBuf> {
    tokio::task::spawn_blocking(move || write_error_log(&log_path, errors)).await?
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
        let _ = file.write(format!("{} <--- {}\n\n", path.as_ref().display(), error).as_bytes());
    });

    Ok(log_path)
}
