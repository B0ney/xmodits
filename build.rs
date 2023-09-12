fn main() {
    #[cfg(windows)]
    {
        static_vcruntime::metabuild();
        // TODO: replace with embed resource
        winres::WindowsResource::new()
            .set_icon("./dist/windows/icon.ico")
            .compile()
            .expect("embed windows icon");
    }

    #[cfg(feature = "build_info")]
    built::write_built_file().expect("Failed to acquire build-time information");
}
