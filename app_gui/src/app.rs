use eframe::egui;
use crate::xmodits::{XmoditsApp,Msg};

impl eframe::App for XmoditsApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // todo!()
        render_top_panel(self, ctx);
    }
}

fn render_top_panel(app: &mut XmoditsApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            if ui.button("Open").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Tracker Modules", &["it", "mod", "s3m"])
                    .pick_file()
                {
                    // BUG: picking file names with "#" will be changed to %23, breaking the path
                    // This issue is present in xdg-dialog
                    if let Some(send) = &app.app_tx {
                        send.send(Msg::LoadModule(path))
                            .expect("failed to send ebook load event");
                    }
                };
            }
        })
    });

}

