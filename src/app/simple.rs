use data::config::Config;
use data::xmodits_lib;

use crate::dialog;
use crate::logger::write_error_log;
use crate::ripper::subscription::extraction::strict_loading;

use std::cmp::Ordering;
use std::path::PathBuf;

use xmodits_lib::interface::Error;
use xmodits_lib::{common::extract, interface::ripper::Ripper};

pub fn rip(paths: impl IntoIterator<Item = String>) {
    let mut paths: Vec<PathBuf> = paths
        .into_iter()
        .map(PathBuf::from)
        .filter(|f| f.exists())
        .collect();

    let has_folder = paths.iter().any(|f| f.is_dir());

    if has_folder {
        return dialog::path_contains_folder();
    }

    if paths.is_empty() {
        return dialog::show_help_box();
    };

    let config = Config::load();

    let filter = strict_loading(config.ripping.strict);

    if config.ripping.strict {
        paths.retain(|f| filter(f));

        if paths.is_empty() {
            return dialog::no_valid_modules();
        };
    }

    let use_cwd = config.general.non_gui_use_cwd;

    let destination = match use_cwd {
        true => std::env::current_dir().unwrap_or(".".into()),
        false => config.ripping.destination.clone(),
    };

    let log_path = config.general.logging_path.as_ref().unwrap_or(&destination);

    let namer = config.naming.build_func();
    let self_contained = config.ripping.self_contained;

    let mut ripper = Ripper::default();
    ripper.change_namer(namer);

    let errors: Vec<(PathBuf, Error)> = paths
        .into_iter()
        .filter_map(|mod_path| {
            extract(&mod_path, &destination, &ripper, self_contained)
                .err()
                .map(|error| (mod_path, error))
        })
        .collect();

    let quiet_output = config.general.non_gui_quiet_output;

    match errors.len().cmp(&1) {
        Ordering::Less if !quiet_output => dialog::success(&destination),
        Ordering::Equal if !quiet_output => dialog::failed_single({
            let (_, error) = &errors[0];
            &error.to_string()
        }),
        Ordering::Greater => match write_error_log(log_path, errors) {
            Ok(log_path) => dialog::success_partial(log_path),
            Err(error) => dialog::success_partial_no_log(&error.to_string()),
        },
        _ => (),
    }
}
