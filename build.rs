fn main() {
    #[cfg(windows)]
    {
        static_vcruntime::metabuild();
        // TODO: replace with embed resource
        winres::WindowsResource::new()
            .set_icon("./dist/windows/icon.ico")
            .compile()
            .expect("embed windows icon");
        // println!("cargo:rerun-if-changed=dist/windows/xmodits.rc");
        // embed_resource::compile("./dist/windows/xmodits.rc",embed_resource::NONE);
    }

    #[cfg(feature = "build_info")]
    built::write_built_file().expect("Failed to acquire build-time information");
}
