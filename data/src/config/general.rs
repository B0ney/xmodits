use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::theme::Themes;

use super::name_params::SampleNameParams;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct GeneralConfig {
    pub theme: Themes,
    pub logging_path: Option<PathBuf>,
    pub non_gui_quiet_output: bool,
    pub non_gui_use_cwd: bool,
    pub hide_gif: bool,
    pub suppress_warnings: bool,
    pub show_errors_in_text_editor: bool,
    pub sample_name_params: SampleNameParams,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            logging_path: None,
            non_gui_quiet_output: false,
            non_gui_use_cwd: false,
            hide_gif: false,
            suppress_warnings: false,
            theme: Themes::default(),
            sample_name_params: SampleNameParams::default(),
            show_errors_in_text_editor: true,
        }
    }
}
