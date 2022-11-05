use std::io::Write;
use std::thread;
use std::sync::Arc;
use crossbeam_channel::unbounded;
use std::path::PathBuf;
use std::env;
use rand::Rng;
use xmodits_lib::Error;
// use xmodits::app;
// use xmodits::app_win::dialoge;

enum Msg {
    Success,
    SuccessPartial(Vec<String>),
    Error(Vec<String>),
    LatestModule(usize),
}

// I'm doing this crap because "using the terminal on Windows feels weird"
pub fn run(modules: Vec<PathBuf>, dest_dir: PathBuf) {
    // Store modules in Arc<T> to avoid expensive clones.
    // The main thread still needs access to the modules in case the thread panics.
    // If that happens, we can tell the user what file cased the panic.
    let modules: Arc<Vec<PathBuf>> = Arc::new(modules);
    let mut latest_module: usize = 0;

    let (tx, rx) = unbounded::<Msg>();

    // External thread will dump samples
    // If it panics, we can notify the user.
    let tx_modules: Arc<Vec<PathBuf>>   = modules.clone();
    let tx_dest_dir: PathBuf            = dest_dir.clone();
    
    let dumper_thread = thread::spawn(move || {
        // Preallocate potential errors to avoid reallocation in hot loop 
        let mut errors: Vec<(usize, Error)> = Vec::with_capacity(tx_modules.len());
        
        // Iterate through modules and dump them
        tx_modules
            .iter()
            .enumerate()
            .for_each(|(index, mod_path)| {
                // Send index of module it is currently ripping.
                // Should be cheap to do.
                tx.send(Msg::LatestModule(index)).unwrap();

                // If ripping fails, provide index of module & its error message.
                // We push (usize, Box<dyn Error>) because it is cheap to do.
                // This is good for performance in a hot loop.
                if let Err(error) = app::dump_samples(mod_path, &tx_dest_dir) {
                    errors.push((index, error));
                }
            }
        );

        // Send errors to main thread if there's any.
        match errors.is_empty() {
            true => tx.send(Msg::Success).unwrap(),
            false => {
                // When the loop finishes and if we get any errors,
                // we can format the errors.
                let errors: Vec<String> = errors
                    .iter()
                    .map(|(index, error)| {
                        format!(
                            "Error ripping: {:?}\n{}\n\n",
                            tx_modules[*index].file_name().unwrap_or_default(),
                            error
                        )
                    })
                    .collect();

                if errors.len() > 1 {
                    tx.send(Msg::SuccessPartial(errors)).unwrap()
                } else { 
                    tx.send(Msg::Error(errors)).unwrap() 
                }
            }
        }
    });

    loop {
        match rx.recv() {
            Ok(Msg::LatestModule(module)) => {
                latest_module = module; // used in case the thread panics
            }
            
            Ok(Msg::Success) => dialoge::success(),

            Ok(Msg::Error(e)) => dialoge::failed_single(&e[0]),

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
    check_thread_panic(dumper_thread.join(), &modules[latest_module]);
}

// If the thread panics, the send/recv channels are severed, breaking out of the loop.
// We fetch the thread's dying wish and display it as a fatal error to the user 
// alongside the module it was trying to rip.
fn check_thread_panic(r: thread::Result<()>, module: &PathBuf) {
    if let Err(e) = r {
        let modname = module.file_name().unwrap_or_default();

        match (
                e.downcast_ref::<String>(),
                e.downcast_ref::<&'static str>()
            ) {
                (Some(e), None) => dialoge::critical_error(&e, modname),
                (None, Some(e)) => dialoge::critical_error(e, modname),
                _ => dialoge::critical_error("Unknown error...", modname), // This should never happen
            }
    }
}

fn main() {
    let args: Vec<std::ffi::OsString> = env::args_os().skip(1).collect();
    let mut paths: Vec<PathBuf> = args
        .iter()
        .map(|f| PathBuf::from(f))
        .collect();

    if args.len() == 0 { 
        return dialoge::show_help_box();
    }

    let dest_dir: PathBuf = match paths.last().unwrap() {
        p if p.is_dir() && paths.len() > 1 => paths.pop().unwrap(),
        _ => env::current_dir().unwrap(),
    };

    // Filter paths to just contain files.
    let modules: Vec<PathBuf> = paths
        .into_iter()
        .filter(|f| f.is_file())
        .collect();

    if modules.len() == 0 { 
        return dialoge::no_valid_modules();
    }

    return run(modules, dest_dir);
}