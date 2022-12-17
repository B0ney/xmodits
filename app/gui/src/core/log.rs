use anyhow::Result;
use rand::Rng;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub async fn async_write_error_log(
    log_path: PathBuf,
    errors: Vec<(PathBuf, String)>,
) -> Result<()> {
    tokio::task::spawn_blocking(move || write_error_log(&log_path, errors)).await?
}

pub fn write_error_log<E, P>(log_path: &PathBuf, errors: Vec<(P, E)>) -> Result<()>
where
    E: std::fmt::Display,
    P: AsRef<Path>,
{
    let log_path: PathBuf = log_path.join(format!(
        "xmodits-error-log-{:04X}.txt",
        rand::thread_rng().gen::<u16>()
    ));

    let mut file = File::create(log_path)?;
    errors.iter().for_each(|(path, error)| {
        let _ = file.write(format!("{} <--- {}\n\n", path.as_ref().display(), error).as_bytes());
    });

    Ok(())
}
