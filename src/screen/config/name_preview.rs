use std::path::{Path, PathBuf};

use data::config::{SampleNameConfig, SampleRippingConfig};
use data::xmodits_lib::interface::{name::Context, Sample};

use iced::widget::text;

use crate::widget::Text;

#[derive(Debug)]
pub struct SampleNameParams {
    module_name: String,
    filename: Option<String>,
    name: String,
    source: PathBuf,
    raw_index: u16,
    seq_index: u16,
}

pub fn preview_name<'a>(
    naming: &'a SampleNameConfig,
    ripping: &'a SampleRippingConfig,
) -> Text<'a> {
    let module_name = "music.it";
    let filename = "SNARE.WAV";
    let name = "snare";
    let path = Path::new("~/Downloads").join(module_name);

    let namer_func = naming.build_func();
    let formatter = ripping.exported_format.get_impl();

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

    text(namer_func(&dummy_sample, &context, sequential_index))
}
