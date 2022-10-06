use std::path::{Path, PathBuf};
use xmodits_lib::XmoditsError;

pub fn dump_samples<T,U>(mod_path: T, dest_dir: U) -> Result<(), XmoditsError>
where   T: AsRef<Path>,
        U: AsRef<Path>,
{
    let modname: String = mod_path.as_ref().file_name().unwrap().to_str().unwrap().replace(".", "_");
    let folder: PathBuf = dest_dir.as_ref().join(modname);

    if folder.exists() {
        return Err(XmoditsError::FileError(format!("Folder already exists: {}", &folder.display())));
    }

    xmodits_lib::load_module(mod_path)?
        .dump(&folder, true)
}