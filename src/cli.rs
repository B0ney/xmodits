use std::path::Path;
use std::process::exit;

static HELP: &str = "\
--help      -h      Prints help information
--version   -V      Prints version
";

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    None,
    Version,
    Help,
    #[cfg(feature = "built")]
    BuildInfo,
    #[cfg(windows)]
    DragNDrop(Vec<String>),
    Unrecognised(String),
    #[cfg(feature = "manual")]
    Manual,
}

pub fn parse(args: Vec<String>) -> Mode {
    if contains(&args, ["--help", "-h"]) {
        return Mode::Help;
    }

    if contains(&args, ["--version", "-V", "-v"]) {
        return Mode::Version;
    }

    #[cfg(feature = "built")]
    if contains(&args, ["--info", "-i"]) {
        return Mode::BuildInfo;
    }

    #[cfg(feature = "manual")]
    if contains(&args, ["--manual", "-m", "--man"]) {
        return Mode::Manual;
    }

    if let Some(unrecognised) = args
        .iter()
        .find(|f| f.starts_with('-') && !Path::new(f).exists())
    {
        return Mode::Unrecognised(unrecognised.to_owned());
    }

    #[cfg(windows)]
    if !args.is_empty() {
        return Mode::DragNDrop(args);
    }

    Mode::None
}

pub fn print_help() -> ! {
    print!("{}", HELP);

    #[cfg(feature = "built")]
    println!("--info      -i      Prints build information");
    #[cfg(feature = "manual")]
    println!("--manual    -m      Prints application manual");
    exit(0)
}

pub fn print_version() -> ! {
    println!("{}", env!("CARGO_PKG_VERSION"));
    exit(0)
}

#[cfg(feature = "built")]
pub fn print_info() -> ! {
    use crate::screen::build_info::info;

    for (label, value) in info(true) {
        println!("{label}: {value}");
    }

    exit(0)
}

pub fn print_unrecognised(option: String) -> ! {
    eprintln!("Unrecognised option '{option}'");
    print_help()
}

#[cfg(feature = "manual")]
pub fn print_manual() -> ! {
    print!("{}", data::MANUAL);
    exit(0);
}

fn contains<const T: usize>(args: &[String], flags: [&str; T]) -> bool {
    args.iter().any(|f| flags.contains(&f.as_str()))
}
