use crate::Cli;
use std::path::{Path, PathBuf};
use xmodits_lib::common::{dump_samples_advanced, folder, total_size_megabytes};
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

fn file_name(path: &Path) -> std::borrow::Cow<str> {
    path.file_name().unwrap_or_default().to_string_lossy()
}

pub fn info(cli: Cli) {
    let items = cli.trackers.iter().filter(|f| f.is_file()).count();

    if items == 0 {
        return eprintln!("You need to provide a valid module!");
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
        Err(e) => eprintln!("Error {} <-- {}", e, file_name(module)),
    }
}

pub fn rip(cli: Cli, destination: PathBuf) {
    let sample_namer_func: Box<SampleNamerFunc> = build_namer(&cli);

    let items = cli.trackers.iter().filter(|f| f.is_file()).count();

    if items == 0 {
        return eprintln!("There's nothing to rip!");
    }

    let total_size = total_size_megabytes(&cli.trackers);

    if total_size > 512.0 {
        println!(
            "Ripping {:.2} MiB worth of trackers. Please wait...",
            total_size
        );
    } else {
        println!("Ripping...")
    }

    for mod_path in cli.trackers.iter().filter(|f| f.is_file()) {
        if let Err(error) = dump_samples_advanced(
            mod_path,
            &folder(&destination, mod_path, !cli.no_folder),
            &sample_namer_func,
            !cli.no_folder,
            &cli.hint,
            // cli.loop_points,
            false,
        ) {
            eprintln!("Error {} <-- \"{}\"", error, file_name(mod_path))
        }
    }
    println!("Done!");
}
