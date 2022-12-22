use std::path::Path;
use xmodits_lib::{SampleNamer, SampleNamerFunc, XmoditsError};
use xmodits_lib::common::{dump_samples_advanced, folder};

pub fn rip_multiple(
    paths: Vec<String>,
    destination: String,
    index_raw: Option<bool>,
    index_padding: Option<usize>,
    index_only: Option<bool>,
    with_folder: Option<bool>,
    // with_loop_points: Option<bool>,
    upper: Option<bool>,
    lower: Option<bool>,
    hint: Option<String>,
) -> Result<(), XmoditsError> {
    let sample_namer_func: Box<SampleNamerFunc> = SampleNamer::build_func(
        index_only.unwrap_or_default(),
        index_padding,
        index_raw.unwrap_or_default(),
        lower.unwrap_or_default(),
        upper.unwrap_or_default(),
    );
    let create_if_absent: bool = with_folder == Some(true);
    // let with_loop_points: bool = with_loop_points == Some(true);
    let with_loop_points: bool = false;

    // Collect errors during dumping
    let mut errors: Vec<XmoditsError> = paths
        .into_iter()
        .filter(|path| Path::new(path).is_file())
        .map(|path| {
            dump_samples_advanced(
                &path,
                folder(&destination, &path, create_if_absent),
                &sample_namer_func,
                create_if_absent,
                &hint,
                with_loop_points,
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
