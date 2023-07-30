#[cfg(windows)]
fn main() {
    extern crate winres;
    let mut ws = winres::WindowsResource::new();
    ws.set_icon("./assets/img/logo/icon.ico").compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
