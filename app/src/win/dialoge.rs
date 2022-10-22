// use native_dialog::{MessageType, MessageDialog};

// fn show_dialoge(title: &str, msg: &str, msg_type: MessageType) {
//     MessageDialog::new()
//         .set_title(title)
//         .set_text(msg)
//         .set_type(msg_type)
//         .show_alert()
//         .unwrap()
// }

// pub fn show_help_box() {
//     show_dialoge(
//         "No tracker modules",
//         "Please drag and drop a valid tracker module onto xmodits.\nSupported formats: IT, S3M, MOD, XM",
//         MessageType::Info
//     )
// }

// pub fn success() {
//     show_dialoge(
//         "Success!",
//         "Ripped samples successfully!",
//         MessageType::Info
//     )
// }

// pub fn success_partial<P: AsRef<std::path::Path>>(log_path: P) {
//     show_dialoge(
//         "Some errors have occured",
//         &format!("There were some errors while dumping. Check the logs at: \"{}\"", log_path.as_ref().display()),
//         MessageType::Warning
//     )
// }
// pub fn success_partial_no_log(error: &str) {
//     show_dialoge(
//         "Some errors have occured",
//         &format!("There were some errors while dumping, but I couldn't create a log file: {}", error),
//         MessageType::Warning
//     )
// }

// pub fn failed_single(error: &str) {
//     show_dialoge(
//         "Can't rip from this file",
//         &format!("{}", error),
//         MessageType::Warning
//     )
// }

// pub fn no_valid_modules() {
//     show_dialoge(
//         "No files provided",
//         "You haven't provided any files.\nSupported formats: IT, S3M, MOD, XM",
//         MessageType::Error
//     )
// }

// pub fn critical_error(error: &str, module: &std::ffi::OsStr) {
//     show_dialoge(
//         "Fatal Error",
//         &format!("An internal panic occured while attemping to rip from {:?}\n\nError: {}\nPlease provide this module in your bug report.", module, error),
//         MessageType::Error
//     )
// }