use crate::core::cfg::Config;
use crate::core::dialog::{
    failed_single, show_help_box, success, success_partial, success_partial_no_log,
};
use crate::core::log::write_error_log;
use std::cmp::Ordering;
use std::path::PathBuf;
use xmodits_lib::interface::Error;
use xmodits_lib::{common::extract, interface::ripper::Ripper};

pub fn rip(paths: Vec<PathBuf>) {
    let paths: Vec<PathBuf> = paths.into_iter().filter(|f| f.is_file()).collect();

    if paths.is_empty() {
        return show_help_box();
    };

    let config = Config::load();

    let quiet_output = config.general.non_gui_quiet_output;
    let use_destination = config.general.non_gui_use_cwd;

    let destination = match use_destination {
        false => config.ripping.destination.clone(),
        true => std::env::current_dir().unwrap_or(".".into()),
    };

    let log_path = match &config.general.logging_path {
        Some(log) => log,
        None => &destination,
    };

    let config = &config.ripping;
    let namer = config.naming.build_func();

    let mut ripper = Ripper::default();
    ripper.change_namer(namer);

    let mut errors: Vec<(PathBuf, Error)> = paths
        .into_iter()
        .filter_map(|mod_path| {
            match extract(&mod_path, &destination, &ripper, config.self_contained) {
                Ok(_) => None,
                Err(error) => Some((mod_path, error)),
            }
        })
        .collect();

    // todo: quiet output
    match errors.len().cmp(&1) {
        Ordering::Less => {
            if !quiet_output {
                success(&destination)
            }
        }
        Ordering::Equal => {
            if !quiet_output {
                failed_single(&errors.pop().unwrap().1.to_string())
            }
        }
        Ordering::Greater => {
            let result = write_error_log(log_path, errors);

            if quiet_output {
                return;
            }

            match result {
                Ok(log_path) => success_partial(log_path),
                Err(error) => success_partial_no_log(&error.to_string()),
            }
        }
    }
}
