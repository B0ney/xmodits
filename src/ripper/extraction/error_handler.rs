use std::path::PathBuf;

use rand::Rng;
use tokio::io::{AsyncWriteExt, BufWriter};

use super::error::{Failed, Reason};

const MAX: usize = 100;
const ABSOLUTE_LIMIT: usize = MAX * 10;

/// When the subscription receives errors from the workers, they're stored in this enum.
///
/// They're first stored in memory, but if there's too many of them to be displayed,
/// store them in a file.
/// At this stage, all future errors will be streamed to the file asynchronously.
///
/// However, if we can't create a file for some reason, we keep the errors in memory;
/// to preserve memory at this stage, future errors will be discarded when it's reached its absolute limit.
#[derive(Debug)]
pub enum ErrorHandler {
    Mem {
        errors: Vec<Failed>,
        log_dir: PathBuf,
    },
    File {
        total: u64,
        path: PathBuf,
        file: Box<BufWriter<tokio::fs::File>>,
    },
    FailedFile {
        reason: String,
        errors: Vec<Failed>,
        discarded: u64,
    },
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::Mem {
            // Reserve an extra element so that pushing the last error before they're moved to a file
            // won't allocate an extra MAX elements
            errors: Vec::with_capacity(MAX + 1),
            log_dir: dirs::download_dir().expect("downloads folder"),
        }
    }
}

impl ErrorHandler {
    pub async fn push(&mut self, error: Failed) {
        match self {
            ErrorHandler::Mem { errors, log_dir } => {
                if errors.len() < MAX {
                    errors.push(error);
                    return;
                }

                let mut errors = std::mem::take(errors);
                let mut log_path = std::mem::take(log_dir);

                errors.push(error);
                log_path.push(format!(
                    "xmodits-error-log-{:04X}.txt",
                    rand::thread_rng().gen::<u16>()
                ));

                *self = match tokio::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&log_path)
                    .await
                    .map(BufWriter::new)
                    .map(Box::new)
                {
                    Ok(mut file) => {
                        let total = errors.len() as u64;

                        // Write stored errors to the new file
                        for error in errors {
                            Self::write_error(&mut file, error).await;
                        }

                        Self::File {
                            total,
                            path: log_path,
                            file,
                        }
                    }

                    Err(error) => Self::FailedFile {
                        reason: error.to_string(),
                        errors,
                        discarded: 0,
                    },
                };
            }

            ErrorHandler::File { total, file, .. } => {
                Self::write_error(file, error).await;
                *total += 1;
            }

            ErrorHandler::FailedFile {
                errors, discarded, ..
            } => {
                if errors.len() < ABSOLUTE_LIMIT {
                    errors.push(error);
                    return;
                }
                *discarded += 1;
            }
        }
    }

    /// dump the errors to a file, will overwrite
    pub async fn dump(errors: Vec<Failed>, path: PathBuf) -> Result<(), String> {
        match tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)
            .await
            .map(BufWriter::new)
        {
            Ok(mut file) => {
                for error in errors {
                    Self::write_error(&mut file, error).await;
                }
                Ok(())
            }
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    async fn write_error<W>(file: &mut W, error: Failed)
    where
        W: AsyncWriteExt + std::marker::Unpin,
    {
        let failed_file = error.path.display().to_string();
        let _ = file.write_all(failed_file.as_bytes()).await;
        let _ = file.write_all(b"\n     ").await;

        match error.reason {
            Reason::Single(reason) => {
                let _ = file.write_all(reason.as_bytes()).await;
            }
            Reason::Multiple(reasons) => {
                //todo include raw index
                for (_raw_idx, reason) in reasons {
                    let _ = file.write_all(reason.as_bytes()).await;
                    let _ = file.write_all(b"\n").await;
                }
            }
        }
        let _ = file.write_all(b"\n\n").await;
        let _ = file.flush().await;
    }
}
