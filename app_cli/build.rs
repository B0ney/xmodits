use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        use winres::WindowsResource;
        let mut ws = WindowsResource::new();
        ws.set_icon_with_id("../extras/logos/icon.ico", "icon")
            .set_icon_with_id("../extras/logos/icon2.ico", "2")
            .set_icon_with_id("../extras/logos/icon3.ico", "3");
        #[cfg(unix)]
        ws.set_toolkit_path("/usr/bin")
            .set_windres_path("x86_64-w64-mingw32-windres")
            .set_ar_path("x86_64-w64-mingw32-ar");
        ws.compile().unwrap();
    }
}