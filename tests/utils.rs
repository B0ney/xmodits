pub mod file_hash {
    use sha2::{Digest, Sha256};
    use std::{fs, io, path::Path};

    pub fn hash_file(file: &mut fs::File) -> String {
        let mut hasher = Sha256::new();
        let _ = io::copy(file, &mut hasher).unwrap();

        format!("{:X}", hasher.finalize())
    }

    pub fn clean_test_export<P: AsRef<Path>>(root: P, number: usize) -> Result<(), u8> {
        for file in fs::read_dir(root.as_ref().join(format!("test_export_{}/", number)))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|f| f.extension() == Some(std::ffi::OsStr::new("wav")))
        {
            fs::remove_file(file).unwrap();
        }
        Ok(())
    }
}

pub mod file_compare {
    use super::hash_file;
    use std::{fs, path::Path};

    pub fn compare_files<T, U>(files: Vec<(&str, &str)>, export_path: T, origin_path: U)
    where
        T: AsRef<Path>,
        U: AsRef<Path>,
    {
        files.iter().for_each(|(export, orig)| {
            let p1 = export_path.as_ref().join(export);
            let p2 = origin_path.as_ref().join(orig);

            let mut export_: fs::File = fs::File::open(p1).unwrap();
            let mut orig_: fs::File = fs::File::open(p2).unwrap();

            assert_eq!(
                hash_file(&mut export_),
                hash_file(&mut orig_),
                "{}",
                format!(
                    "\n\nFILE MISMATCH!:\n     - {} (original)\n     - {}'\n\n",
                    orig, export
                )
            );
        });
    }
}

pub fn verify_sample_num(expected: usize, given: usize, modname: &str) {
    assert_eq!(
        expected,
        given,
        "{}",
        format!(
            "\n\nMODNAME: {}\n     EXPECTED: {} SAMPLES, GOT: {} INSTEAD\n\n",
            modname, expected, given
        )
    );
}

/// macro to verify sample number
/// ```
/// check_sample_number!(
///     test_name
///     path: "path/to/a/tracker.mod",
///     with: EXPECTED_SAMPLE_NUMBER   
/// )
/// ```
#[macro_export]
macro_rules! check_sample_number {
    // ($test_name:ident kind: $tracker:ty, path: $path:expr, with: $smp_no:tt) => {
    ($test_name:ident path: $path:expr, with: $smp_no:tt) => {
        #[test]
        fn $test_name() {
            let a = xmodits_lib::load_module($path).unwrap();
            verify_sample_num($smp_no, a.number_of_samples(), a.module_name());
        }
    };
}

// pub use verify_sample_num;
pub use file_compare::compare_files;
pub use file_hash::{clean_test_export, hash_file};
