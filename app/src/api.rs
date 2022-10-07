use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

use crate::app;

use super::Cli;
use super::cli as application;
use indicatif::ProgressIterator;
use progress_bar::*;
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
    // init_progress_bar(cli.trackers.iter().filter(|f| f.is_file()).count());
    // let pb = ProgressBar::new(cli.trackers.len() as u64);
    let sample_namer_func: Box<SampleNamerFunc> = build_namer(&cli);

    let total_size = application::total_size_MB(&cli.trackers);

    if total_size > 512.0 {
        print_progress_bar_info(
            "Info:",
            &format!("Ripping {:.2} MiB worth of trackers. Please wait...", total_size),
            Color::Green, Style::Normal);
    }
    let pb = ProgressBar::new(cli.trackers.iter().filter(|f| f.is_file()).count() as u64);
    
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed}] [{bar:40.cyan/blue}] ({pos}/{len})",
        )
        .unwrap(),
    );
    
    // set_progress_bar_action("Ripping", Color::Blue, Style::Bold);
    pb.println("\x1B[32mRipping...");

    cli.trackers
        .into_iter()
        .filter(|f| f.is_file())
        .for_each(|mod_path| {
            if let Err(error) = app::dump_samples_advanced(
                &mod_path, &folder(&destination, &mod_path, !cli.no_folder),
                &sample_namer_func, !cli.no_folder
            ) {
                pb.println(format!("{} {}",
                    "\x1B[31mError",
                    &format!("{} <-- \"{}\"", error, mod_path.file_name().unwrap().to_string_lossy())
                ));

                // print_progress_bar_info(
                //     "Error ", 
                //     &format!("{} <-- \"{}\"", error, mod_path.file_name().unwrap().to_string_lossy(),),
                //     Color::Red, Style::Normal
                // );
            }
            // inc_progress_bar();
            pb.inc(1);
            // pb.inc(1);
        }
    );
    // finalize_progress_bar();
    pb.finish_with_message("done");

}

#[cfg(feature="advanced")]
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

#[test]
fn test() {
    use indicatif::{ProgressBar, ProgressStyle};

    // // Default styling, attempt to use Iterator::size_hint to count input size
    // for _ in (0..1000).progress() {
    //     // ...
    //     thread::sleep(Duration::from_millis(5));
    // }

    // // Provide explicit number of elements in iterator
    // for _ in (0..1000).progress_count(1000) {
    //     // ...
    //     thread::sleep(Duration::from_millis(5));
    // }

    // Provide a custom bar style
    let pb = ProgressBar::new(10);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len})",
        )
        .unwrap(),
    );
    // for _ in (0..10).progress_with(pb) {
    //     // ...
    //     println!("ygj");
    //     thread::sleep(Duration::from_millis(500));
    // }
    // let pb = ProgressBar::new(100);
    for i in 0..10 {
        thread::sleep(Duration::from_millis(500));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
