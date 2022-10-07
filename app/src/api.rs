use std::path::Path;
use std::path::PathBuf;

use crate::app;

use super::Cli;
use super::cli as application;
use progress_bar::*;
use indicatif::ProgressBar;
use xmodits_lib::{SampleNamer, SampleNamerFunc};

pub fn build_namer(cli: &Cli) -> Box<SampleNamerFunc> {
    SampleNamer::build_func(
        cli.index_only, Some(cli.index_padding as usize), cli.index_raw, cli.lower_case, cli.upper_case
    )
}

fn folder(destination: &PathBuf, path: &PathBuf, with_folder: bool) -> PathBuf {
    match with_folder {
        true => {
            let modname: String = path
                .file_name().unwrap()
                .to_str().unwrap()
                .replace(".", "_");
            
            let new_folder: PathBuf = destination.join(modname);

            new_folder
        },
        _ => destination.to_path_buf(),
    }
}

pub fn rip(cli: Cli, destination: PathBuf) {
    init_progress_bar(cli.trackers.len());

    let sample_namer_func: Box<SampleNamerFunc> = build_namer(&cli);

    let total_size = application::total_size_MB(&cli.trackers);

    if total_size > 512.0 { // change to >
        print_progress_bar_info(
            "Info:",
            &format!("Ripping {:.2} MiB worth of trackers. Please wait...", total_size),
            Color::Green, Style::Normal);
    }

    set_progress_bar_action("Ripping", Color::Blue, Style::Bold);

    cli.trackers
        .into_iter()
        .filter(|f| f.exists() && f.is_file())
        .for_each(|mod_path| {
            // println!("{}", );
            if let Err(error) = app::dump_samples_advanced(
                &mod_path, &folder(&destination, &mod_path, !cli.no_folder),
                &sample_namer_func, !cli.no_folder
            ) {
                print_progress_bar_info(
                    "Error", 
                    &format!("{} <-- \"{}\"",  error, mod_path.file_name().unwrap().to_string_lossy(),),
                    Color::Red, Style::Normal
                );
            }
            inc_progress_bar();
        }
    );
    finalize_progress_bar();
}

// #[cfg(feature="advanced")]
pub fn rip_parallel(cli: Cli) {
    use rayon::prelude::*;

    init_progress_bar(cli.trackers.len());

    let total_size = application::total_size_MB(&cli.trackers);

    if total_size < 512.0 {
        print_progress_bar_info(
            "Warning:",
            &format!("Ripping {:.2} MiB worth of trackers in parallel is no faster when done serially.", total_size),
            Color::Yellow, Style::Normal);
    } else {
        print_progress_bar_info(
            "Info:",
            &format!("Ripping {:.2} MiB worth of trackers. Please wait...", total_size),
            Color::Green, Style::Normal);
    }

    set_progress_bar_action("Ripping", Color::Blue, Style::Bold);

    cli.trackers
        .into_par_iter()
        .filter(|f| f.exists() && f.is_file())
        .for_each(|mod_path| {
            if let Err(error) = app::dump_samples(&mod_path, "/home/boney/Downloads/Folders/modarchive_2021_additions/dump/") {
                print_progress_bar_info(
                    "Error", 
                    &format!("{} --> \"{}\"",  error,mod_path.file_name().unwrap().to_string_lossy(),),
                    Color::Red, Style::Normal
                );
            }
            // inc_progress_bar();
            // a. 1;
        }
    );
    finalize_progress_bar();
}



