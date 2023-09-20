//! Preview how ripped samples will be named

use std::path::{Path, PathBuf};

use data::config::{SampleNameConfig, SampleRippingConfig};
use data::xmodits_lib::interface::{name::Context, Sample};

use iced::widget::text;

use crate::widget::Text;

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
            module_name: Default::default(),
            sample_filename: Default::default(),
            sample_name: Default::default(),
            module_source: Default::default(),
            raw_index: Default::default(),
            seq_index: Default::default(),
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
) -> Text<'a> {
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
        source_path: Some(&source_path),
    };

    text(namer_func(&dummy_sample, &context, params.seq_index as usize))
}
