use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::fs::{File, self};
use futures_util::future::join_all;

use xmodits_lib::wav::Wav;
use xmodits_lib::{load_module, TrackerModule, Error, SampleNamerFunc, XmoditsError};

fn name_folder(destination: &PathBuf, path: &PathBuf, with_folder: bool) -> PathBuf {
    match with_folder {
        true => {
            let modname: String = path
                .file_name().unwrap()
                .to_str().unwrap()
                .replace(".", "_");
            
            let new_folder: PathBuf = destination.join(modname);

            new_folder
        },
        _ => destination.to_path_buf(),
    }
}

pub async fn run(
    paths: &[PathBuf],
    destination: &PathBuf,
    sample_namer: &SampleNamerFunc,
    create_dir_if_absent: bool,
) {
    let joined_futures = join_all(
        paths
            .iter()
            .filter(|f| f.is_file())
            .map(|path| rip(path, &destination, sample_namer, create_dir_if_absent))
    );

    joined_futures.await
        .into_iter()
        .zip(paths)
        .filter(|(f,e)| f.is_err())
        .for_each(|(e, p)| eprintln!("Error {} <-- \"{}\"", e.err().unwrap(), p.display()));
}

async fn rip(
    mod_path: &PathBuf,
    folder: &PathBuf,
    namer: &SampleNamerFunc,
    create_dir_if_absent: bool
) -> Result<(), Error> {
    let mut module = xmodits_lib::load_module(mod_path)?;
    // dbg!(mod_path);
    // dbg!(folder);
    let folder = name_folder(&folder, &mod_path, create_dir_if_absent);

    if folder.is_dir() && create_dir_if_absent {
        return Err(XmoditsError::FileError(format!("Folder already exists: {}", &folder.display())));
    }

    if module.number_of_samples() == 0 {
        return Err(XmoditsError::EmptyModule);
    }

    if !&folder.is_dir() {
        if create_dir_if_absent {
            fs::create_dir(&folder).await?;
                // .map_err(|err| helpful_io_error(err, folder.as_ref()))?;
        } else {
            return Err(
                XmoditsError::file(
                    &format!("Destination '{}' either doesn't exist or is not a directory", folder.display())
                )
            );
        }
    }
    // let

    // let dest_path = 
    for i in 0..module.number_of_samples() {
        async_export(&mut module, &folder, i, namer).await?;
    };
    Ok(())
}


async fn async_export(tracker: &mut TrackerModule, folder: &Path, index: usize, name_sample: &SampleNamerFunc) -> Result<(), Error> {
    let smp = &tracker.list_sample_data()[index];
    
    let file: PathBuf = PathBuf::new()
            .join(folder)
            .join(name_sample(smp, index));
    
    let mut wav: File = File::create(file).await?;
    
    let wav_header = Wav::header(smp.rate, smp.bits, smp.len as u32, smp.is_stereo).header_data;

    wav.write_all(&wav_header).await?;
    wav.write_all(tracker.pcm(index)?).await?;
    
    Ok(())
}