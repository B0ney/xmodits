use std::path::Path;

use xmodits_lib::exporter::AudioFormat;

use crate::config::SampleNameConfig;

pub fn preview_sample_name(config: &SampleNameConfig, export_format: &AudioFormat) -> String {
    // todo!()
    use xmodits_lib::interface::{Sample, name::Context};

    let module_name = "music.it";
    let filename = "SNARE.WAV";
    let name = "snare";
    let path = Path::new("~/Downloads").join(module_name);

    let namer_func = config.build_func();
    let formatter = export_format.get_impl();

    let dummy_sample = Sample {
        filename: Some(filename.into()),
        name: name.into(),
        index_raw: 5,
        ..Default::default()
    };

    let context = Context {
        total: 10,
        extension: formatter.extension(),
        highest: 10,
        source_path: Some(&path),
    };

    let sequential_index = 0;

    namer_func(&dummy_sample, &context, sequential_index)
}
