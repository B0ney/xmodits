#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut ws = winres::WindowsResource::new();
    ws.set_icon("./res/img/logo/icon.ico")
        .compile()
        .unwrap();
}

#[cfg(unix)]
fn main() {}
