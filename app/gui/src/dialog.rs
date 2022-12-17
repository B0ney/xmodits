use native_dialog::{MessageDialog, MessageType};
use std::path::Path;

fn show_dialoge(title: &str, msg: &str, msg_type: MessageType) {
    MessageDialog::new()
        .set_title(title)
        .set_text(msg)
        .set_type(msg_type)
        .show_alert()
        .unwrap()
}

pub fn show_help_box() {
    show_dialoge(
        "No tracker modules",
        "If you wish to rip from a folder, please launch the GUI instead.",
        MessageType::Info,
    )
}

pub fn success<P: AsRef<Path>>(dest: P) {
    show_dialoge(
        "Success!",
        &format!("Successfully ripped samples to {}", dest.as_ref().display()),
        MessageType::Info,
    )
}

pub fn success_partial<P: AsRef<Path>>(log_path: P) {
    show_dialoge(
        "Some errors have occured",
        &format!(
            "There were some errors while dumping. Check the logs at: {}",
            log_path.as_ref().display()
        ),
        MessageType::Warning,
    )
}

pub fn success_partial_no_log(error: &str) {
    show_dialoge(
        "Some errors have occured",
        &format!(
            "There were some errors while dumping, but xmodits could not create a log file: {}",
            error
        ),
        MessageType::Warning,
    )
}

pub fn failed_single(error: &str) {
    show_dialoge("Cannot rip from this file", error, MessageType::Warning)
}

// pub fn no_valid_modules() {
//     show_dialoge(
//         "No files provided",
//         "You haven't provided any files.\nSupported formats: IT, XM, S3M, MOD, UMX",
//         MessageType::Error,
//     )
// }

pub fn critical_error(error: &str) {
    show_dialoge(
        "FATAL ERROR (>_<)",
        &format!("{}\n\nThe program will now terminate.", error),
        MessageType::Error,
    )
}
