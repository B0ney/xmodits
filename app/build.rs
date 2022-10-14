fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        use winres::WindowsResource;
        let mut ws = WindowsResource::new();
        ws.set_icon_with_id("../extras/logos/ico/icon.ico", "icon");
        #[cfg(unix)]
        ws.set_toolkit_path("/usr/bin")
            .set_windres_path("x86_64-w64-mingw32-windres")
            .set_ar_path("x86_64-w64-mingw32-ar");
        ws.compile().unwrap();
    }
}