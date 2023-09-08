//! Configure how samples should be extracted

use data::config::SampleRippingConfig;

use iced::Element;
use iced::widget::{checkbox, column, container, text, row, pick_list, horizontal_rule};

use xmodits_lib::exporter::AudioFormat; // todo

#[derive(Debug, Clone)]
pub enum Message {
    SetFormat(AudioFormat),
    ToggleSelfContained(bool),
    ToggleStrictLoad(bool),
    SetFolderDepth(u8),
}

pub fn view<'a>(
    ripping: &'a SampleRippingConfig
) -> Element<'a, Message> {
    let col1 = column![
        checkbox("Self Contained", ripping.self_contained, Message::ToggleSelfContained),
        checkbox("Strict Loading", ripping.strict, Message::ToggleStrictLoad),
    ];
    let formats = data::SUPPORTED_FORMATS;
    let export_format = row![
        pick_list(
            formats, 
            Some(ripping.exported_format), 
            Message::SetFormat
        ),
        text("Export Format"),
    ];

    let options: &[u8] = &[1, 2, 3, 4, 5, 6, 7];

    let folder_scan_depth = row![
        pick_list(
            options,
            Some(ripping.folder_max_depth),
            Message::SetFolderDepth
        ),
        text("Export Format"),
    ];
    
    let settings = column![
        text("Ripping Configuration"),
        col1,
        export_format,
        horizontal_rule(1),
        folder_scan_depth
    ];

    container(settings)
        .into()
}
