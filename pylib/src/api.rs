use std::path::{Path, PathBuf};


use xmodits_lib::SampleNamer;
use xmodits_lib::*;

pub fn rip_multiple(
    paths: Vec<String>,
    destination: String,

    index_raw: Option<bool>,
    index_padding: Option<usize>,
    index_only: Option<bool>,
    with_folder: Option<bool>,
    upper: Option<bool>,
    lower: Option<bool>,
) -> Result<(), Error> {
    let sample_namer_func: Box<SampleNamerFunc> = SampleNamer::build_func(
        index_only.unwrap_or_default(),
        index_padding,
        index_raw.unwrap_or_default(),
        lower.unwrap_or_default(),
        upper.unwrap_or_default(),
    );
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
