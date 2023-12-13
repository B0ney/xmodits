use std::{fs::File, path::PathBuf};

use audio_engine::SamplePack;

// TODO
pub async fn load_sample_pack(path: PathBuf) -> SamplePack {
    tokio::task::spawn_blocking(move || {
        let mut file = File::open(path).unwrap();
        let module = xmodits_lib::load_module(&mut file).unwrap();
        SamplePack::build(&*module)
    })
    .await
    .unwrap()
}
