use crate::{app, Cli};

use std::path::PathBuf;

use xmodits_lib::{SampleNamer, SampleNamerFunc};

pub fn build_namer(cli: &Cli) -> Box<SampleNamerFunc> {
    SampleNamer::build_func(
        cli.index_only,
        Some(cli.index_padding as usize),
        cli.index_raw,
        cli.lower_case,
        cli.upper_case,
    )
}

fn file_name(path: &PathBuf) -> String {
    path.file_name().unwrap().to_string_lossy().to_string()
}

fn folder(destination: &PathBuf, path: &PathBuf, with_folder: bool) -> PathBuf {
    match with_folder {
        true => {
            let modname: String = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".", "_");

            let new_folder: PathBuf = destination.join(modname);

            new_folder
        }
        _ => destination.to_path_buf(),
    }
}

pub fn info(cli: Cli) {
    let items = cli.trackers.iter().filter(|f| f.is_file()).count();

    if items == 0 {
        return println!("{}", "You need to provide a valid module!");
    }

    let module = &cli.trackers[0];

    let tracker = match &cli.hint {
        Some(hint) => xmodits_lib::load_from_ext(module, hint),
        None => xmodits_lib::load_module(module),
    };

    match tracker {
        Ok(m) => {
            println!(
                "Module Name: {}\nFormat: {}\nSamples: {}\nApprox Total Sample Size (KiB): {}",
                m.module_name(),
                m.format(),
                m.number_of_samples(),
                m.list_sample_data().iter().map(|m| m.len).sum::<usize>() / 1000,
            )
        }
        Err(e) => println!("Error {} <-- {}", e, file_name(module)),
    }
}

pub fn rip(cli: Cli, destination: PathBuf) {
    let sample_namer_func: Box<SampleNamerFunc> = build_namer(&cli);

    let items = cli.trackers.iter().filter(|f| f.is_file()).count();

    if items == 0 {
        return println!("{}", "There's nothing to rip!");
    }

    let total_size = app::total_size_MiB(&cli.trackers);

    if total_size > 512.0 {
        println!(
            "Ripping {:.2} MiB worth of trackers. Please wait...",
            total_size
        );
    } else {
        println!("Ripping...")
    }

    for mod_path in cli.trackers.iter().filter(|f| f.is_file()) {
        if let Err(error) = app::dump_samples_advanced(
            &mod_path,
            &folder(&destination, &mod_path, !cli.no_folder),
            &sample_namer_func,
            !cli.no_folder,
            &cli.hint,
        ) {
            eprintln!("Error {} <-- \"{}\"", error, file_name(&mod_path))
        }
    }
    println!("Done!");
}

#[cfg(feature = "advanced")]
pub fn rip_parallel(cli: Cli, destination: PathBuf) {
    use rayon::prelude::*;

    let sample_namer_func: Box<SampleNamerFunc> = build_namer(&cli);

    let items = cli.trackers.iter().filter(|f| f.is_file()).count();

    if items == 0 {
        return println!("{}", "There's nothing to rip!");
    }

    let total_size = app::total_size_MiB(&cli.trackers);

    if total_size > 512.0 {
        println!(
            "Ripping {:.2} MiB worth of trackers in parallel. Please wait...",
            total_size
        );
    } else {
        println!(
            "Ripping {:.2} MiB worth of trackers in parallel is no faster when done serially.",
            total_size
        );
    }

    cli.trackers
        .into_par_iter()
        .filter(|f| f.is_file())
        .for_each(|mod_path| {
            if let Err(error) = app::dump_samples_advanced(
                &mod_path,
                &folder(&destination, &mod_path, !cli.no_folder),
                &sample_namer_func,
                !cli.no_folder,
            ) {
                eprintln!("{} <-- \"{}\"", error, file_name(&mod_path));
            }
        });
    println!("Done!");
}
