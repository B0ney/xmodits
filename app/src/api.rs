use crate::app;

use super::Cli;
use super::cli as application;
use progress_bar::*;
use indicatif::ProgressBar;


pub fn rip(cli: Cli) {
    init_progress_bar(cli.trackers.len());

    let total_size = application::total_size_MB(&cli.trackers);

    if total_size < 512.0 { // change to >
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
            if let Err(error) = app::dump_samples(&mod_path, "~/Downloads/Folders/modarchive_2021_additions/dump/") {
                print_progress_bar_info(
                    "Error", 
                    &format!("{} --> \"{}\"",  error,mod_path.file_name().unwrap().to_string_lossy(),),
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
            a. 1;
        }
    );
    finalize_progress_bar();
}


