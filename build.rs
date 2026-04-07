fn main() {
    // Compile C++ library
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cpp_include_dir = format!("{}/udp_prague", manifest_dir);
    let cpp_source_dir = format!("{}/udp_prague", manifest_dir);

    cc::Build::new()
        .cpp(true)
        .file(format!("{}/prague_cc.cpp", cpp_source_dir))
        .include(&cpp_include_dir)
        .flag_if_supported("-std=c++14")
        .opt_level(3)
        .warnings(true)
        .compile("udp_prague");

    // Rebuild triggers
    println!("cargo:rerun-if-changed=udp_prague/prague_cc.cpp");
    println!("cargo:rerun-if-changed=udp_prague/prague_cc.h");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(format!("{}/prague_cc.h", cpp_include_dir))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .clang_arg(format!("-I{}", cpp_include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings");

    // Write bindings to Rust source
    let rust_source_file = std::path::Path::new(&manifest_dir).join("src/bindings.rs");
    bindings
        .write_to_file(rust_source_file)
        .expect("Couldn't write bindings!");
}
