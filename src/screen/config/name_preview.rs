//! Preview how ripped samples will be named

use std::path::PathBuf;

use data::config::{SampleNameConfig, SampleNameParams, SampleRippingConfig};
use data::xmodits_lib::interface::{name::Context, Sample};

use crate::widget::Element;

#[derive(Debug, Clone)]
pub enum Message {
    ModuleName(String),
    FileName(Option<String>),
    Source(PathBuf),
    RawIndex(u16),
    SeqIndex(u16),
}

pub fn update(name_params: &mut SampleNameParams, message: Message) {
    tracing::info!("{:?}", &message);
    
    match message {
        Message::ModuleName(module_name) => name_params.module_name = module_name,
        Message::FileName(file_name) => name_params.sample_filename = file_name,
        Message::Source(source) => name_params.module_source = source,
        Message::RawIndex(raw_index) => name_params.raw_index = raw_index,
        Message::SeqIndex(seq_index) => name_params.seq_index = seq_index,
    }
}

pub fn view<'a>() -> Element<'a, Message> {
    todo!()
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
