/*On windows, using dialoge is how we can effectively comminucate with the end-user.*/
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
}

// I'm doing this crap because "using the terminal on Windows feels weird"
pub fn run(modules: Vec<PathBuf>, dest_dir: PathBuf)-> Result<(), Error> {
    let (tx, rx) = mpsc::channel::<Msg>();

    // Let external thread dump samples.
    // If it panics, we can notify the user.
    let dest = dest_dir.clone();
    let dumper_thread = thread::spawn(move || {
        let mut errors: Vec<String> = Vec::new();

        modules
            .iter()
            .for_each(|mod_path| {
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
    }).join();

    loop {
        if thread_panic(&dumper_thread) {
            break;
        }

        match rx.recv() {
            Ok(msg) => {
                match msg {
                    Msg::Success => dialoge::success(),

                    Msg::SuccessPartial(errors) => {
                        let error_log = PathBuf::new()
                            .join(&dest_dir)
                            .join(
                                format!(
                                    "xmodits-error-log-{:04X}.txt",
                                    rand::thread_rng().gen::<u16>()
                                )
                            );

                        if error_log.exists() {
                            dialoge::success_partial_no_log(
                                &format!("\"{}\" - Already exists...", 
                                error_log.display())
                            );
                            break;
                        }
                        
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
                }; 
                break;
            }
            Err(_) => { break; }
        }
    }

    Ok(())
}

fn thread_panic(r: &thread::Result<()>) -> bool {
    match r {
        Ok(_) => false,
        Err(e) => {
            match e.downcast_ref::<String>() {
                Some(e) => dialoge::critical_error(&e),
                None => {
                    match e.downcast_ref::<&'static str>() {
                        Some(e) => dialoge::critical_error(e), 
                        None => dialoge::critical_error("Unkown error...") // This should never happen
                    }
                }
            }
            true
        }
    }
}
