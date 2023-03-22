use crate::Cli;
use std::fs::File;
use std::path::{Path, PathBuf};

use xmodits_lib::{
    common::extract, fmt::loader::load_module, interface::ripper::Ripper, SampleNamer,
    SampleNamerTrait,
};

pub fn build_namer(cli: &Cli) -> Box<dyn SampleNamerTrait> {
    SampleNamer {
        index_only: cli.index_only,
        index_padding: cli.index_padding as u8,
        index_raw: cli.index_raw,
        lower: cli.lower_case,
        upper: cli.upper_case,
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
                "Module Name: {}\nSamples: {}\nApprox Total Sample Size (KiB): {}",
                m.name(),
                // m.format(),
                m.total_samples(),
                m.samples().iter().map(|m| m.length as usize).sum::<usize>() / 1000,
            )
        }
        Err(e) => eprintln!("Error {} <-- {}", e, file_name(module)),
    }
}

pub fn rip(cli: Cli, destination: PathBuf) {
    let mut ripper = Ripper::default();
    ripper.change_namer(build_namer(&cli));
    // let sample_namer_func: Box<SampleNamerFunc> = ;

    let items = cli.trackers.iter().filter(|f| f.is_file()).count();

    if items == 0 {
        return eprintln!("There's nothing to rip!");
    }

    // let total_size = total_size_megabytes(&cli.trackers);

    // if total_size > 512.0 {
    //     println!(
    //         "Ripping {:.2} MiB worth of trackers. Please wait...",
    //         total_size
    //     );
    // } else {
    //     println!("Ripping...")
    // }

    for mod_path in cli.trackers.iter().filter(|f| f.is_file()) {
        if let Err(error) = extract(mod_path, &destination, &ripper, !cli.no_folder)
        // dump_samples_advanced(
        //
        //     &folder(, mod_path, !cli.no_folder),
        //     &sample_namer_func,
        //
        //     &cli.hint,
        //     // cli.loop_points,
        //     false,
        // )
        {
            eprintln!("Error {} <-- \"{}\"", error, file_name(mod_path))
        }
    }
    println!("Done!");
}
