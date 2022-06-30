use std::io;
#[cfg(windows)] use winres::WindowsResource;

fn main() -> io::Result<()> {
    #[cfg(windows)] {
        WindowsResource::new()
            .set_icon("res/icon.ico")
            .compile()?;
    }
    Ok(())
}
