use xmodits_lib::interface::Error;
use xmodits_lib::{
    common::extract, fmt::loader::load_module, interface::ripper::Ripper, SampleNamer,
    SampleNamerTrait,
};
use crate::core::cfg::Config;
use crate::core::log::write_error_log;
use crate::dialog::{
    failed_single, show_help_box, success, success_partial, success_partial_no_log,
};
use std::cmp::Ordering;
use std::path::PathBuf;

pub fn rip(paths: Vec<PathBuf>) {
    let paths: Vec<PathBuf> = paths.into_iter().filter(|f| f.is_file()).collect();
    if paths.is_empty() {
        return show_help_box();
    };
    let config = Config::load();
    let log_path = match &config.general.logging_path {
        Some(log) => log,
        None => &config.ripping.destination,
    };
    let config = &config.ripping;
    let namer = config.naming.build_func();
    // let hint = &config.hint.into();
    let mut ripper = Ripper::default();
    ripper.change_namer(namer);


    let mut errors: Vec<(PathBuf, Error)> = paths
        .into_iter()
        .filter_map(|mod_path| {
            match extract(
                &mod_path,
                &config.destination,
                &ripper,
                config.self_contained,
            ) {
                Ok(_) => None,
                Err(error) => Some((mod_path, error)),
            }
        })
        .collect();

    match errors.len().cmp(&1) {
        Ordering::Less => success(&config.destination),
        Ordering::Equal => failed_single(&errors.pop().unwrap().1.to_string()),
        Ordering::Greater => match write_error_log(log_path, errors) {
            Ok(log_path) => success_partial(log_path),
            Err(error) => success_partial_no_log(&error.to_string()),
        },
    }
}
