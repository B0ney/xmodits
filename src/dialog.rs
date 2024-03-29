use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use std::path::Path;

fn show_dialog(title: &str, msg: impl Into<String>, msg_type: MessageLevel) -> MessageDialog {
    MessageDialog::new()
        .set_title(title)
        .set_description(msg)
        .set_level(msg_type)
}

pub fn show_help_box() {
    show_dialog(
        "No tracker modules",
        "If you want to rip from a folder, please launch the GUI.",
        MessageLevel::Info,
    )
    .show();
}

pub fn path_contains_folder() {
    show_dialog(
        "Use the GUI for folders",
        "If you want to rip from a folder, please launch the GUI.",
        MessageLevel::Info,
    )
    .show();
}

pub fn success<P: AsRef<Path>>(dest: P) {
    let dialog = show_dialog(
        "Success!",
        format!(
            "Successfully ripped samples to {}.\n\nShow Results?",
            dest.as_ref().display()
        ),
        MessageLevel::Info,
    )
    .set_buttons(MessageButtons::YesNo);

    if let MessageDialogResult::Yes = dialog.show() {
        let _ = open::that_detached(dest.as_ref());
    }
}

pub fn success_partial<P: AsRef<Path>>(destination: P, log_path: P) {
    let dialog = show_dialog(
        "Some errors have occurred",
        format!(
            "xmodits could not rip everything. Check the logs at:\n{}.\n\nShow Results and Errors?",
            log_path.as_ref().display()
        ),
        MessageLevel::Warning,
    )
    .set_buttons(MessageButtons::YesNo);

    if let MessageDialogResult::Yes = dialog.show() {
        let _ = open::that_detached(destination.as_ref());
        let _ = open::that_detached(log_path.as_ref());
    }
}

pub fn success_partial_no_log(error: &str) {
    show_dialog(
        "Some errors have occurred",
        format!(
            "xmodits could not rip everything. And it could not create a log file: {}",
            error
        ),
        MessageLevel::Warning,
    )
    .show();
}

pub fn failed_single(error: &str) {
    show_dialog("Cannot rip from this file", error, MessageLevel::Warning).show();
}

pub fn no_valid_modules() {
    show_dialog(
        "No files provided",
        "You haven't provided any valid files!\n\nAllowed extensions: .it  .xm  .s3m  .mod  .umx  .mptm\n\nHINT: You can disable this by unchecking \"Strict Loading\" from the GUI, make sure to save if you do!",
        MessageLevel::Error,
    ).show();
}

pub fn critical_error(error: &str) {
    show_dialog("FATAL ERROR (>_<)", error, MessageLevel::Error).show();
}
