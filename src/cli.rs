use std::path::Path;

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

    if let Some(unrecognised) = args
        .iter()
        .find(|f| f.starts_with('-') && !Path::new(f).exists())
    {
        return Mode::Unrecognised(unrecognised.to_owned());
    }

    #[cfg(windows)]
    if args.len() > 0 {
        return Mode::DragNDrop(args);
    }

    Mode::None
}

pub fn print_help() {
    print!("{}", HELP);

    #[cfg(feature = "built")]
    println!("--info      -i      Prints build information");
}

pub fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

#[cfg(feature = "built")]
pub fn print_info() {
    use crate::screen::build_info::info;

    for (label, value) in info(true) {
        println!("{label}: {value}");
    }
}

pub fn print_unrecognised(option: String) {
    println!("Unrecognised option '{option}'");
    print_help();
}

fn contains<const T: usize>(args: &[String], flags: [&str; T]) -> bool {
    args.iter().any(|f| flags.contains(&f.as_str()))
}
