#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    // if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // use WindowsResource;
   
        // let mut ws = winres::WindowsResource::new();
    // ws.set_icon_with_id("./res/img/logo/icon.ico", "icon");
    // #[cfg(unix)]
    // ws.set_toolkit_path("/usr/bin")
    //     .set_windres_path("x86_64-w64-mingw32-windres")
    //     .set_ar_path("x86_64-w64-mingw32-ar");
    // ws.compile().unwrap();
    // // }
}

#[cfg(unix)]
fn main() {

}