//! Preview how ripped samples will be named

use data::config::{SampleNameConfig, SampleNameParams, SampleRippingConfig};
use xmodits_lib::{export::name::Context, Sample};

use super::extraction::ExtractionConfig;

pub fn preview_name<'a>(
    params: &SampleNameParams,
    extraction: &ExtractionConfig,
) -> String {
    let filename = params.sample_filename.clone();
    let name = params.sample_name.clone();
    let source_path = &params.module_source;

    let namer_func = build_func(extraction);
    let formatter = extraction.exported_format.get_impl();

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

pub fn build_func(cfg: &ExtractionConfig) -> Box<dyn xmodits_lib::export::SampleNamerTrait> {
    xmodits_lib::export::SampleNamer {
        index_only: cfg.index_only,
        index_padding: cfg.index_padding,
        index_raw: cfg.index_raw,
        lower: cfg.lower,
        upper: cfg.upper,
        prefix_source: cfg.prefix,
        prefer_filename: cfg.prefer_filename,
        ..Default::default()
    }
    .into()
}