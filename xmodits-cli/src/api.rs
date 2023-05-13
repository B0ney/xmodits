use crate::Cli;
use std::fs::File;
use std::path::{Path, PathBuf};

use xmodits_lib::{
    common::extract, exporter::AudioFormat, fmt::loader::load_module, interface::ripper::Ripper,
    SampleNamer, SampleNamerTrait,
};

pub fn build_namer(cli: &Cli) -> Box<dyn SampleNamerTrait> {
    SampleNamer {
        index_only: cli.index_only,
        index_padding: cli.index_padding as u8,
        index_raw: cli.index_raw,
        lower: cli.lower_case,
        upper: cli.upper_case,
        prefer_filename: true,
        prefix_source: cli.prefix,
        ..Default::default()
    }
    .into()
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
    let mut file = File::open(module).unwrap();

    let tracker = load_module(&mut file);

    match tracker {
        Ok(m) => {
            println!(
                "Module Name: {}\nFormat: {}\nSamples: {}\nApprox Total Sample Size (KiB): {}",
                m.name(),
                m.format(),
                m.total_samples(),
                m.samples().iter().map(|m| m.length as usize).sum::<usize>() / 1000,
            )
        }
        Err(e) => eprintln!("Error {} <-- {}", e, file_name(module)),
    }
}

pub fn rip(cli: Cli, destination: PathBuf) {
    let ripper = Ripper::new(build_namer(&cli), get_format(&cli.format).into());
    let paths: Vec<PathBuf> = cli.trackers.into_iter().filter(|f| f.is_file()).collect();
    let self_contained = !cli.no_folder;

    if paths.is_empty() {
        return eprintln!("There's nothing to rip!");
    }

    println!("Ripping {} file(s)\n", paths.len());

    let rip_file = move |path: PathBuf| {
        extract(&path, &destination, &ripper, self_contained).unwrap_or_else(|error| {
            eprintln!("Can't rip {}:\n   \"{}\"\n", file_name(&path), error)
        })
    };

    #[cfg(feature = "rayon")]
    {
        // let Some(threads) = cli.threads else {
        let threads = cli.threads as usize;
        if threads == 0 {
            paths.into_iter().for_each(rip_file);
            println!("Done!");
            return;
        };

        use rayon::prelude::*;

        rayon::ThreadPoolBuilder::new()
            .num_threads(threads as usize)
            .build()
            .expect("Building threapool")
            .spawn(move || paths.into_par_iter().for_each(rip_file));
    }

    #[cfg(not(feature = "rayon"))]
    paths.into_iter().for_each(rip_file);

    println!("Done!");
}

fn get_format(format: &str) -> AudioFormat {
    match format {
        "wav" => AudioFormat::WAV,
        "aiff" => AudioFormat::AIFF,
        "8svx" => AudioFormat::IFF,
        "raw" => AudioFormat::RAW,
        _ => AudioFormat::WAV,
    }
}
