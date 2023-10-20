//! Preview how ripped samples will be named

use std::path::PathBuf;

use data::config::{SampleNameConfig, SampleRippingConfig};
use data::xmodits_lib::interface::{name::Context, Sample};

#[derive(Debug, Clone)]
pub enum Message {
    ModuleName(String),
    FileName(Option<String>),
    Source(PathBuf),
    RawIndex(u16),
    SeqIndex(u16),
}

#[derive(Debug)]
pub struct SampleNameParams {
    module_name: String,
    module_source: PathBuf,
    sample_filename: Option<String>,
    sample_name: String,
    raw_index: u16,
    seq_index: u16,
}

impl Default for SampleNameParams {
    fn default() -> Self {
        Self {
            module_name: String::from("music"),
            sample_filename: Some(String::from("kick_1.wav")),
            sample_name: String::from("kick.wav"),
            module_source: PathBuf::from("~/Downloads/music.it"),
            raw_index: 7,
            seq_index: 0,
        }
    }
}

impl SampleNameParams {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ModuleName(_) => todo!(),
            Message::FileName(_) => todo!(),
            Message::Source(_) => todo!(),
            Message::RawIndex(_) => todo!(),
            Message::SeqIndex(_) => todo!(),
        }
    }
}

pub fn preview_name<'a>(
    params: &SampleNameParams,
    naming: &'a SampleNameConfig,
    ripping: &'a SampleRippingConfig,
) -> String {
    let filename = params.sample_filename.clone();
    let name = params.sample_name.clone();
    let source_path = &params.module_source;

    let namer_func = naming.build_func();
    let formatter = ripping.exported_format.get_impl();

    let dummy_sample = Sample {
        filename: filename.map(|f| f.into_boxed_str()),
        name: name.into(),
        index_raw: params.raw_index,
        ..Default::default()
    };

    let context = Context {
        total: 10,
        extension: formatter.extension(),
        highest: 10,
        source_path: Some(source_path),
    };

    namer_func(&dummy_sample, &context, params.seq_index as usize)
}
