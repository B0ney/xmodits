fn main() {
    // TODO: replace with embed resource
    #[cfg(windows)]
    winres::WindowsResource::new()
        .set_icon("./assets/img/logo/icon.ico")
        .compile()
        .unwrap();

    #[cfg(feature = "with_metadata")]
    built::write_built_file().expect("Failed to acquire build-time information");
}
