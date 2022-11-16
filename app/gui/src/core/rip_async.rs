use std::path::{Path, PathBuf};
use tokio::io::{AsyncRead, AsyncWrite};

use tokio::fs::{File, self, };
// use futures_util::future::join_all;

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
/// experimental. tried it, it's not really any better + uses a lot of memory since xmodits loads modules into memory directly 
pub async fn run(
    paths: &[PathBuf],
    destination: &PathBuf,
    sample_namer: &SampleNamerFunc,
    create_dir_if_absent: bool,
) {
    let joined_futures = 
        paths
            .iter()
            .filter(|f| f.is_file())
            .map(|path| rip(path, &destination, sample_namer, create_dir_if_absent))
    ;

    for task in joined_futures {
        task.await;
    };
}

async fn rip(
    mod_path: &PathBuf,
    folder: &PathBuf,
    namer: &SampleNamerFunc,
    create_dir_if_absent: bool
) -> Result<(), Error> {
    let folder = name_folder(&folder, &mod_path, create_dir_if_absent);
    if folder.is_dir() && create_dir_if_absent {
        return Err(XmoditsError::FileError(format!("Folder already exists: {}", &folder.display())));
    }
    dbg!("bah");
    let mut module = xmodits_lib::load_module(mod_path)?;

    dump_advanced(&mut module, &folder, namer, create_dir_if_absent).await
}

async fn dump_advanced(
    module: &mut TrackerModule,
    folder: &Path,
    sample_namer_func: &SampleNamerFunc,
    create_dir_if_absent: bool,
)  -> Result<(), Error>  {

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
    for i in 0..module.number_of_samples() {
        async_export(module, folder, i, sample_namer_func).await?;
    }

    Ok(())
}

async fn async_export(
    tracker: &mut TrackerModule,
    folder: &Path,
    index: usize,
    name_sample: &SampleNamerFunc
) -> Result<(), Error> {
    let smp = &tracker.list_sample_data()[index];
    
    let file: PathBuf = PathBuf::new()
            .join(folder)
            .join(name_sample(smp, index));
    
    async_write_wav(tracker, &file, index).await
}

async fn async_write_wav(
    module: &mut TrackerModule,
    file: &Path,
    index: usize,
)  -> Result<(), Error> {
    let smp = &module.list_sample_data()[index];
    let mut wav: File = File::create(file).await?;
    let wav_header = Wav::header(smp.rate, smp.bits, smp.len as u32, smp.is_stereo, smp.is_interleaved).header_data;
   
    wav.write_all(&wav_header).await?;
    wav.write_all(module.pcm(index)?).await?;
    Ok(())
}