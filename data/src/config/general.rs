use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GeneralConfig {
    // pub theme: Themes,
    pub logging_path: Option<PathBuf>,
    pub non_gui_quiet_output: bool,
    pub non_gui_use_cwd: bool,
}
