// #[cfg(test)]
// mod tests {
use std::path::Path;

use xmodits_lib::{TrackerDumper, tracker_formats::*};

#[test]
fn it_empty() {
    assert!(
        ITFile::load_module("tests/mods/it/empty.it").is_err()
    );
}

#[test]
fn it_test_mmcmp() {
    assert!(
        ITFile::load_module("tests/mods/it/creagaia.it").is_err()
    );
}
#[test]
fn it_no_samples() {
    let a = ITFile::load_module("tests/mods/it/no_samples.it").unwrap();
    let folder = "test/exports/";
    let name = "IT-please-delete";
    let export_path = Path::new(folder).join(name);

    assert_eq!(a.number_of_samples(), 0);
    assert!(!export_path.exists());
    assert!(a.dump(&folder, name).is_err())
}

#[test]
fn it_should_be_17_samples_not_18() {
    let a = ITFile::load_module("tests/mods/it/17_samples.it").unwrap();
    assert_eq!(a.number_of_samples(), 17);
}

#[test]
fn it_test_1() {
    let a = ITFile::load_module("tests/mods/it/asikwp_-_fc-freedrive_chiptune.it").unwrap();
    assert_eq!(a.number_of_samples(), 9);
}

#[test]
fn it_test_2() {
    let a = ITFile::load_module("tests/mods/it/beyond_-_flute.it").unwrap();
    assert_eq!(a.number_of_samples(), 2);
}

#[test]
fn it_test_3() {
    let a = ITFile::load_module("tests/mods/it/sm-safari.it").unwrap();
    assert_eq!(a.number_of_samples(), 19);
}

#[test]
fn it_test_4() {
    let a = ITFile::load_module("tests/mods/it/songofthesky.it").unwrap();
    assert_eq!(a.number_of_samples(), 14);
}