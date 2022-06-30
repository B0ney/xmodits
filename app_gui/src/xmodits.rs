use crossbeam_channel::{unbounded, Receiver, Sender};
use xmodits_lib::{TrackerDumper, TrackerModule, Error};
use eframe::egui;
use std::thread;

use crate::modloader::load_module;

pub enum ThreadMsg {
    NewModule(Result<TrackerModule, Error>),
    ErrorMsg(Error),
    SuccessMsg(String)
}

// Application sends these messages to thread
pub enum Msg {
    LoadModule(std::path::PathBuf),
    DumpModule(std::path::PathBuf),
    ExportModule(usize),
    NewCtx(egui::Context),

}

pub struct XmoditsApp {
    /// Events that ColdBrewApp broadcasts to spawned threads
    pub app_tx: Option<Sender<Msg>>, //
    /// Events that ColdBrewApp receives from spawned threads
    pub thread_rx: Option<Receiver<ThreadMsg>>, //
}

impl XmoditsApp {
    fn new() -> Self {
        Self {
            app_tx: None,
            thread_rx: None
        }
    }

    pub fn new_app(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut app = Self::new();
        app.init();

        // Send a clone of the app's context to the thread
        // This allows the thread to request a repaint when needed!
        // This helps keep cpu usage at a minimum
        if let Some(s) = &app.app_tx {
            s.send(Msg::NewCtx(cc.egui_ctx.clone())).unwrap();
            // Tell thread to load library before app starts.
            // s.send(Msg::LoadLibrary).unwrap();
        };
        app
    }

    /// Will handle the dumping process
    pub fn init(&mut self) {
        // Setup channels to allow main thread and spawned thread to communicate
        let (app_tx, app_rx) = unbounded();
        let (thread_tx, thread_rx) = unbounded();

        self.app_tx = Some(app_tx);
        self.thread_rx = Some(thread_rx);

        // First thread is spawned to handle 
        {
            let mut frame: Option<egui::Context> = None;
            let mut tracker_module:  Option<TrackerModule> = None;

            thread::spawn(move || loop {
                match app_rx.recv() {
                    Ok(Msg::LoadModule(path)) => {
                        match load_module(path) {
                            Ok(module) => {
                                tracker_module = Some(module);

                                thread_tx.send(
                                    ThreadMsg::SuccessMsg("Loaded Module Successfully".to_string())
                                ).unwrap();
                            }
                            Err(e) => {
                                if let Err(e) = thread_tx.send(ThreadMsg::ErrorMsg(e)) {
                                    tracing::error!("Can't send ErrMsg:{}", e);
                                };
                            }
                        }

                        // Tell ui thread to update after we send ebook.
                        if let Some(egui_frame) = frame.as_ref() {
                            egui_frame.request_repaint();
                        };
                    }

                    Ok(Msg::DumpModule(path)) => {
                        if let Some(tracker) = tracker_module {
                            match tracker.dump(&path) {
                                Ok(())  => todo!(),
                                Err(_) => todo!(),
                            }
                        }
                        
                        // Tell ui thread to update after we load library
                        if let Some(egui_frame) = frame.as_ref() {
                            egui_frame.request_repaint();
                        };
                    }

                    Ok(Msg::NewCtx(ctx)) => frame = Some(ctx),

                    Err(e) => {
                        tracing::error!("Receiving failed!: {}", e);
                        break;
                    }
                    _ => {}
                };
            });
        }        
    }

    /// Non blocking way to update internal values
    ///
    /// ```
    /// impl eframe::App for ColdBrewApp {
    ///     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    ///         self.update_app(); // <-- placed here
    /// }
    /// ```
    pub fn update_app(&mut self) {
        use ThreadMsg::*;
        if let Some(event) = &self.thread_rx {
            match event.try_recv() {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }
}