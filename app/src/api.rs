use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::app;
use super::Cli;

use xmodits_lib::{SampleNamer, SampleNamerFunc};

use kdam::prelude::*;
use kdam::{Column, RichProgress};

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
    let sample_namer_func: Box<SampleNamerFunc> = build_namer(&cli);

    let items = cli.trackers.iter().filter(|f| f.is_file()).count();

    if items == 0 {
        return println!("{}", "There's nothing to rip!".colorize("green"));
    }

    let mut pb = progress_bar(items);
    let total_size = app::total_size_MB(&cli.trackers);

    if total_size > 512.0 {
        pb.write(&format!("Ripping {:.2} MiB worth of trackers. Please wait...", total_size).colorize("green"));
    } else {
        pb.write("Ripping...".colorize("green"));
    }

    for mod_path in cli.trackers.iter().filter(|f| f.is_file()) {
        if let Err(error) = app::dump_samples_advanced(
            &mod_path, &folder(&destination, &mod_path, !cli.no_folder),
            &sample_namer_func, !cli.no_folder
        ) {
            pb.write(format!("{} {}",
                "Error".colorize("red"),
                &format!("{} <-- \"{}\"", error, mod_path.file_name().unwrap().to_string_lossy())
            ));
        }
        pb.update(1);
    }

    pb.write("Done!".colorize("bold green"));
}

#[cfg(feature="advanced")]
pub fn rip_parallel(cli: Cli, destination: PathBuf) {
    use rayon::prelude::*;

    let sample_namer_func: Box<SampleNamerFunc>= build_namer(&cli);
    // sample_namer_func
    let items = cli.trackers.iter().filter(|f| f.is_file()).count();

    let mut pb = progress_bar(items);
    let total_size = app::total_size_MB(&cli.trackers);

    if total_size > 512.0 {
        pb.write(&format!("Ripping {:.2} MiB worth of trackers in parallel. Please wait...", total_size).colorize("green"));
    } else {
        pb.write("Ripping {:.2} MiB worth of trackers in parallel is no faster when done serially.".colorize("orange"));
    }
    let pb = Arc::new(std::sync::Mutex::new(pb));

    cli.trackers
        .into_par_iter()
        .filter(|f|f.is_file())
        .for_each(|mod_path| {
            let a = pb.clone();
            if let Err(error) = app::dump_samples_advanced(
                &mod_path, &folder(&destination, &mod_path, !cli.no_folder),
                &sample_namer_func, !cli.no_folder
            ) {
                a.lock().unwrap().write(format!("{} {}",
                    "Error".colorize("red"),
                    &format!("{} <-- \"{}\"", error, mod_path.file_name().unwrap().to_string_lossy())
                ));
            }
            a.lock().unwrap().update(1);
        }
    );
    pb.lock().unwrap().write("Done!".colorize("bold green"));
}

fn progress_bar(total: usize) -> RichProgress {
    RichProgress::new(
        tqdm!(
            total = total,
            unit_scale = true,
            unit_divisor = 1024,
            unit = "B"
        ),
        vec![
            Column::Spinner(
                "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
                    .chars()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                80.0,
                1.0,
            ),
            Column::text("[bold blue]?"),
            Column::Bar,
            Column::Percentage(1),
            Column::text("•"),
            Column::CountTotal,
            Column::text("•"),
            Column::Rate,
            Column::text("•"),
            Column::RemainingTime,
        ],
    )
}


#[test]
fn test() {
    let mut pb = progress_bar(200);

    pb.write("download will begin in 5 seconds".colorize("bold red"));

    // while pb.pb.elapsed_time() <= 5.0 {
    //     // pb.refresh();
    // }

    // pb.replace(1, Column::text("[bold blue]docker.exe"));
    pb.write("downloading docker.exe".colorize("bold cyan"));
    pb.write(format!("{} test","downloading docker.exe".colorize("bold cyan")));


    let total_size = 200;
    let mut downloaded = 0;

    while downloaded < total_size {
        let new = std::cmp::min(downloaded + 10, total_size);
        downloaded = new;
        // pb.update_to(new);
        pb.update(10);
        std::thread::sleep(std::time::Duration::from_millis(590));
    }

    pb.write("downloaded docker.exe".colorize("bold green"));
    eprintln!();
}
