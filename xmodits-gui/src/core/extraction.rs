use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write, Seek},
    path::PathBuf,
};

use walkdir::WalkDir;

/// Traversing deeply nested directories can use a lot of memory.
///
/// For that reason we write the output to a file
/// 
/// Todo: make this async?
/// Todo: expand tilde
pub fn traverse(dirs: Vec<PathBuf>) {
    // create a file in read-write mode
    let mut file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./test.txt")
        .unwrap();

    // traverse list of directories, output to a file
    let max_depth = 5;
    dirs.into_iter().for_each(|f| {
        WalkDir::new(f)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| f.path().is_file())
            .for_each(|f| {
                file.write_fmt(format_args!("{}\n", f.path().display()))
                    .unwrap()
            })
    });

    // rewind cursor to beginning
    file.rewind().unwrap();

    // wrap file to bufreader
    BufReader::new(file)
        .lines()
        .filter_map(|f| f.ok())
        .for_each(|f| println!("{}", f));
}


#[test]
fn traverse_() {
    traverse(vec!["~/Downloads/".into()]);
    // load();
}
