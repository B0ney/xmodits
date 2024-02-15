//! Preview how ripped samples will be named

use data::config::{SampleNameConfig, SampleNameParams, SampleRippingConfig};
use xmodits_lib::interface::{name::Context, Sample};

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
