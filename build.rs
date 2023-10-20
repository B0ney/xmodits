fn main() {
    #[cfg(windows)]
    {
        static_vcruntime::metabuild();

        winresource::WindowsResource::new()
            .set_icon("./dist/windows/icon.ico")
            // Set dpi awareness so that rfd won't show blurred text 
            .set_manifest(include_str!("./dist/windows/xmodits.manifest")) 
            .compile()
            .expect("embed windows icon");
    }

    #[cfg(feature = "build_info")]
    built::write_built_file().expect("Failed to acquire build-time information");
}
