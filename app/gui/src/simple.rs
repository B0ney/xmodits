use crate::core::cfg::Config;
use crate::dialog::{
    failed_single, show_help_box, success, success_partial, success_partial_no_log,
};
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use xmodits_common::folder;
use xmodits_lib::XmoditsError;

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
    let hint = &config.hint.convert();

    let mut errors: Vec<(usize, XmoditsError)> = paths
        .iter()
        .map(|mod_path| {
            xmodits_common::dump_samples_advanced(
                mod_path,
                &folder(&config.destination, mod_path, !config.no_folder),
                &namer,
                !config.no_folder,
                hint,
                config.embed_loop_points,
            )
        })
        .enumerate()
        .filter_map(|(idx, result)| match result {
            Ok(_) => None,
            Err(error) => Some((idx, error)),
        })
        .collect();

    match errors.len().cmp(&1) {
        Ordering::Less => success(&config.destination),
        Ordering::Equal => failed_single(&errors.pop().unwrap().1.to_string()),
        Ordering::Greater => {
            let log_path: PathBuf = PathBuf::new().join(log_path).join(format!(
                "xmodits-error-log-{:04X}.txt",
                rand::thread_rng().gen::<u16>()
            ));

            match File::create(&log_path) {
                Ok(mut file) => {
                    errors.iter().for_each(|(idx, error)| {
                        let _ = file.write_all(
                            format!("{} <--- {}\n\n", Path::new(&paths[*idx]).display(), error)
                                .as_bytes(),
                        );
                    });
                    success_partial(log_path)
                }

                Err(error) => success_partial_no_log(&error.to_string()),
            }
        }
    }
}
