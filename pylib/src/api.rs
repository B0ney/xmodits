use std::path::{Path, PathBuf};

use xmodits_lib::*;

pub fn rip_multiple(
    paths: Vec<String>,
    destination: String,

    index_raw: Option<bool>,
    index_padding: Option<usize>,
    index_only: Option<bool>,
    with_folder: Option<bool>,
) -> Result<(), Error> {
    let sample_namer_func: Box<SampleNamerFunc> =
        SampleNamer::build_func(index_only, index_padding, index_raw);
    let create_if_absent: bool = with_folder.is_some();

    // Collect errors during dumping
    let mut errors: Vec<XmoditsError> = paths
        .into_iter()
        .filter(|path| Path::new(path).is_file())
        .map(|path| {
            xmodits_lib::load_module(&path)?.dump_advanced(
                &folder(&destination, &path, with_folder),
                &sample_namer_func,
                create_if_absent,
            )
        })
        .filter_map(|result| result.err())
        .collect();

    use std::cmp::Ordering;
    // Compare size of errors
    // return Ok(()) if errors.len() = 0
    // Extract a single error & return it if errors.len() = 1
    // Construct "MultipleErrors" to contain errors and return it if errors.len() > 1

    match errors.len().cmp(&1) {
        Ordering::Less => Ok(()),
        Ordering::Equal => Err(errors.pop().unwrap()),
        Ordering::Greater => Err(XmoditsError::MultipleErrors(errors)),
    }
}

fn folder(destination: &String, path: &String, with_folder: Option<bool>) -> PathBuf {
    match with_folder {
        Some(true) => {
            let modname: String = Path::new(&path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".", "_");

            let new_folder: PathBuf = PathBuf::new().join(&destination).join(modname);

            new_folder
        }
        _ => PathBuf::new().join(&destination),
    }
}

#[derive(Default)]
struct SampleNamer {
    index_only: Option<bool>,
    index_padding: Option<usize>,
    index_raw: Option<bool>,
}

impl SampleNamer {
    /// Dynamically build a function to format sample names given its internal parameters
    fn to_func(self) -> Box<SampleNamerFunc> {
        const DEFAULT_PADDING: usize = 2;

        Box::new(move |smp: &TrackerSample, idx: usize| -> String {
            format!(
                "{}{}.wav",
                // Index component
                {
                    let index = match self.index_raw {
                        Some(true) => smp.raw_index(),
                        _ => idx + 1,
                    };
                    match self.index_padding {
                        Some(padding) => format!("{:0padding$}", index),
                        None => format!("{:0DEFAULT_PADDING$}", index),
                    }
                },
                // Name component
                match self.index_only {
                    Some(true) => "".to_string(),
                    _ => match smp.filename.trim() {
                        name if name.is_empty() => "".to_string(),
                        name => format!(" - {}", name.replace(".wav", "").replace(".", "_")),
                    },
                }
            )
        })
    }

    fn build_func(
        index_only: Option<bool>,
        index_padding: Option<usize>,
        index_raw: Option<bool>,
    ) -> Box<SampleNamerFunc> {
        SampleNamer {
            index_only,
            index_padding,
            index_raw,
        }
        .to_func()
    }
}
