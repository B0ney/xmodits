use std::path::Path;
use xmodits_lib::{TrackerDumper, tracker_formats::*};
mod utils;
use crate::utils::verify_sample_num;

#[test]
fn mod_empty() {
    assert!(
        MODFile::load_module("tests/mods/mod/empty.mod").is_err()
    );
}

#[test]
fn xpk_mod_unsupported() {
    assert!(
        MODFile::load_module("tests/mods/mod/synthmat.mod").is_err()
    );
}

#[test]
fn mod_no_samples() {
    assert!(
        MODFile::load_module("tests/mods/mod/no_samples.mod").is_err()
    );
}

check_sample_number!(
    mod_test_1
    path: "tests/mods/mod/echobea3.mod",
    with: 15
);

check_sample_number!(
    mod_test_2
    path: "tests/mods/mod/slash-kill-maim-hit.mod",
    with: 19
);

check_sample_number!(
    mod_test_3
    path: "tests/mods/mod/chop.mod",
    with: 5
);

check_sample_number!(
    mod_test_4
    path: "tests/mods/mod/sleep.mod",
    with: 9
);


check_sample_number!(
    mod_test_5
    path: "tests/mods/mod/space_debris.mod",
    with: 17
);

/* ####################################################################### */
