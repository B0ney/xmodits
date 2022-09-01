use std::path::{Path, PathBuf};
use xmodits_lib::XmoditsError;

pub fn dump_samples(mod_path: &PathBuf, dest_dir: &PathBuf) -> Result<(), XmoditsError> {
    let modname: String = mod_path.file_name().unwrap().to_str().unwrap().replace(".", "_");
    let folder: PathBuf = dest_dir.join(modname);

    if folder.exists() {
        return Err(XmoditsError::FileError(format!("Folder already exists: {}", &folder.display())));
    }

    xmodits_lib::load_module(mod_path)?
        .dump(&folder, true)
}