use super::Cli;
use super::cli as application;
use progress_bar::*;

pub fn rip(cli: Cli) {
    init_progress_bar(cli.paths.len());

    let total_size = application::total_size_MB(&cli.paths);

    if total_size < 512.0 { // change to >
        print_progress_bar_info(
            "Info:",
            &format!("Ripping {:.2} MiB worth of trackers. Please wait...", total_size),
            Color::Green, Style::Normal);
    }

    set_progress_bar_action("Ripping", Color::Blue, Style::Bold);

    (0..cli.paths.len())
        .into_iter()
        .for_each(|f| {
            std::thread::sleep(std::time::Duration::from_millis(1));
                if let Err(e) = Err::<usize, &str>(":<") {
                    print_progress_bar_info("Error:", &format!("{}", e), Color::Red, Style::Normal);
                }
            // if f % 9 ==0 {
            //     print_progress_bar_info("Error:", ":(", Color::Red, Style::Normal);
            // }
            inc_progress_bar();
        }
    );
    finalize_progress_bar();

    // for i in 0..81 {
    //     // load page
    //     std::thread::sleep(std::time::Duration::from_millis(50));

    //     // log the result
    //     if i == 14 {
    //         print_progress_bar_info("Failed", "to load https://zefzef.zef", Color::Red, Style::Normal);
    //     } else if i == 41 {
    //         print_progress_bar_info("Success", "loading https://example.com", Color::Green, Style::Bold);
    //     }

    //     // update the progression by 1
    //     inc_progress_bar();
    // }

    // finalize_progress_bar();
}

#[cfg(feature="advanced")]
pub fn rip_parallel(cli: Cli) {
    use rayon::prelude::*;

    init_progress_bar(cli.paths.len());

    let total_size = application::total_size_MB(&cli.paths);

    if total_size < 512.0 {
        print_progress_bar_info(
            "Warning:",
            &format!("Ripping {:.2} MiB worth of trackers in parallel is no faster when done serially.", total_size),
            Color::Yellow, Style::Normal);
    }

    set_progress_bar_action("Ripping", Color::Blue, Style::Bold);

    (0..cli.paths.len())
        .into_par_iter()
        .for_each(|f| {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            if f % 9 ==0 {
                print_progress_bar_info("Error:", ":(", Color::Red, Style::Normal);
            }
            inc_progress_bar();
        }
    );
    finalize_progress_bar();
}


