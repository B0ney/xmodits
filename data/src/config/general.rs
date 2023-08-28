use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GeneralConfig {
    /// Theme of the application
    // pub theme: Themes,
    // pub sfx: bool,
    // pub folder_recursion_depth: u8,
    pub logging_path: Option<PathBuf>,
    pub worker_threads: usize,
    pub non_gui_quiet_output: bool,
    /// Use the current working directory for extraction
    pub non_gui_use_cwd: bool,
}
