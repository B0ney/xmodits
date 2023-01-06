#[cfg(windows)]
fn main() {
    extern crate winres;
    let mut ws = winres::WindowsResource::new();
    ws.set_icon("./res/img/logo/icon.ico").compile().unwrap();
}

#[cfg(unix)]
fn main() {}
