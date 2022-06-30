mod app;
mod xmodits;
mod modloader;

use xmodits::XmoditsApp;
use eframe::{run_native, IconData, NativeOptions};
fn main() {
    tracing_subscriber::fmt::init();
    let native_options = NativeOptions {
        min_window_size: Some((540.0, 480.0).into()),
        drag_and_drop_support: true,
        // icon_data: Some(load_icon_data(include_bytes!("res/icon/icon.png"))),
        icon_data: None,
        ..Default::default()
    };

    run_native(
        "Xmodits - Dump samples from tracker modules",
        native_options,
        Box::new(|cc| Box::new(XmoditsApp::new_app(cc))),
    );
}

// yoinked from discord like a true programmer
pub fn load_icon_data(image_data: &[u8]) -> IconData {
    let image = image::load_from_memory(image_data)
        .expect("Uh... The icon is not a valid image format. Please recompile with a valid one.");
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_raw().clone();

    IconData {
        rgba: pixels,
        width: image.width(),
        height: image.height(),
    }
}
