use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/AppCastingMOOSApp_c.cpp")
        .cpp(true)
        .warnings(true)
        .extra_warnings(true)
        .compile("moosbinding");

    println!("cargo:rustc-link-lib=MOOS");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rerun-if-changed=src/AppCastingMOOSApp_c.cpp");
    println!("cargo:rerun-if-changed=src/AppCastingMOOSApp_c.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/AppCastingMOOSApp_c.hpp")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .no_copy("MoosApp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
