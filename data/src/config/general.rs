use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GeneralConfig {
    // pub theme: Themes,
    pub logging_path: Option<PathBuf>,
    pub non_gui_quiet_output: bool,
    pub non_gui_use_cwd: bool,
}
