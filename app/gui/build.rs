#[cfg(windows)]
#[cfg(target="release")]
fn main() {
    extern crate winres;
    let mut ws = winres::WindowsResource::new();
    ws.set_icon("./res/img/logo/icon.ico")
        .compile()
        .unwrap();
}

#[cfg(windows)]
#[cfg(not(target="release"))]
fn main() {}

#[cfg(unix)]
fn main() {}
