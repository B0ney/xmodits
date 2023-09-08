//

use data::config::SampleNameConfig;

use iced::Element;
use iced::widget::{checkbox, column, container, text, row, pick_list, horizontal_rule};


#[derive(Debug, Clone)]
pub enum Message {
    IndexOnly(bool),
    IndexRaw(bool),
    UpperCase(bool),
    LowerCase(bool),
    IndexPadding(u8),
    PreferFilename(bool),
    PrefixSamples(bool),
}

pub fn view<'a>(
    config: &'a SampleNameConfig,
    preview_name: &'a fn(&SampleNameConfig) -> String,
) -> Element<'a, Message> {
    let col1 = column![
        checkbox("Index Only", config.index_only, Message::IndexOnly),
        checkbox("Preserve Index", config.index_raw, Message::IndexRaw),
        checkbox("Prefix Samples", config.prefix, Message::PrefixSamples),
    ];

    let col2 = column![
        checkbox("Upper Case", config.upper, Message::UpperCase),
        checkbox("Lower Case", config.index_raw, Message::LowerCase),
        checkbox("Prefer Filename", config.prefer_filename, Message::PreferFilename),
    ];
    
    let checkboxes = row![col1, col2];

    let options: &[u8] = &[1, 2, 3, 4];

    let idx_padding = row![
        pick_list(options, Some(config.index_padding), Message::IndexPadding),
        "Index Padding"
    ];

    let name_preview = column![
        text(preview_name(config))
    ];

    let settings = column![
        text("Sample Naming"),
        checkboxes,
        idx_padding,
        horizontal_rule(1),
        name_preview
    ];

    container( settings)
        // .width(Length::Fill)
        .into()
}
