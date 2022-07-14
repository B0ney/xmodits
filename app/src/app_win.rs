use std::io::Write;
use std::thread;
use std::sync::mpsc;
use std::path::PathBuf;
use rand::Rng;
use xmodits_lib::Error;

use crate::app;
use crate::dialoge;

enum Msg {
    Success,
    SuccessPartial(Vec<String>),
    LatestModule(std::ffi::OsString),
}

// I'm doing this crap because "using the terminal on Windows feels weird"
pub fn run(modules: Vec<PathBuf>, dest_dir: PathBuf)-> Result<(), Error> {
    let (tx, rx) = mpsc::channel::<Msg>();

    let mut latest_module: std::ffi::OsString = std::ffi::OsString::new();

    // Let external thread dump samples.
    // If it panics, we can notify the user.
    let dest = dest_dir.clone();
    let dumper_thread = thread::spawn(move || {
        let mut errors: Vec<String> = Vec::new();

        modules
            .iter()
            .for_each(|mod_path| {
                // send name of module it is currently ripping
                // please refactor
                tx.send(Msg::LatestModule(mod_path.file_name().unwrap().to_owned())).unwrap();

                if let Err(error) = app::dump_samples(mod_path, &dest) {
                    errors.push(format!(
                        "Error ripping: {:?}\n      - {}\n\n",
                        mod_path.file_name().unwrap(),
                        error
                    ));
                }
                
            }
        );

        // Send errors to main thread if there's any.
        match errors.is_empty() {
            true => tx.send(Msg::Success).unwrap(),
            false => tx.send(Msg::SuccessPartial(errors)).unwrap()
        }
    });

    loop {
        match rx.recv() {
            Ok(Msg::LatestModule(module)) => {
                latest_module = module; // used in case the thread panics
            }

            Ok(Msg::Success) => dialoge::success(),

            Ok(Msg::SuccessPartial(errors)) => {
                let error_log = PathBuf::new()
                    .join(&dest_dir)
                    .join(
                        format!(
                            "xmodits-error-log-{:04X}.txt",
                            rand::thread_rng().gen::<u16>()
                        )
                    );
                
                match std::fs::File::create(&error_log) {
                    Ok(mut file) => {
                        errors.iter().for_each(|s|{
                            file.write_all(s.as_bytes());
                        });
                        dialoge::success_partial(&error_log)
                    },
                    Err(error) => dialoge::success_partial_no_log(&error.to_string()),
                }
            },

            _ => break,
        }
    }
    check_thread_panic(&dumper_thread.join(), &latest_module);
    Ok(())
}

// If the thread panics, the send/recv channels are severed, breaking out of the loop.
// We fetch the thread's dying wish and display it as a fatal error to the user 
// alongside the module it was trying to rip.
fn check_thread_panic(r: &thread::Result<()>, module: &std::ffi::OsStr) {
    match r {
        Ok(_) => {},
        Err(e) => {
            match (
                e.downcast_ref::<String>(),
                e.downcast_ref::<&'static str>()
            ) {
                (Some(e), None) => dialoge::critical_error(&e, module),
                (None, Some(e)) => dialoge::critical_error(e, module),
                _ => dialoge::critical_error("Unkown error...", module), // This should never happen
            }
        }
    }
}