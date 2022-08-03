extern crate bindgen;
extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    // Build the spectre code with cc
    // src/spectre/api/c/spectre-algorithm.c
    cc::Build::new()
        .file("src/spectre/api/c/spectre-types.c")
        .file("src/spectre/api/c/base64.c")
        .file("src/spectre/api/c/aes.c")
        .file("src/spectre/api/c/spectre-algorithm.c")
        .file("src/spectre/api/c/spectre-algorithm_v0.c")
        .file("src/spectre/api/c/spectre-algorithm_v1.c")
        .file("src/spectre/api/c/spectre-algorithm_v2.c")
        .file("src/spectre/api/c/spectre-algorithm_v3.c")
        .file("src/spectre/api/c/spectre-util.c")
        .file("src/spectre/api/c/spectre-marshal-util.c")
        .file("src/spectre/api/c/spectre-marshal.c")
        .include("src/spectre/api/c")
        .warnings(false)
        .cargo_metadata(true)
        .define("SPECTRE_SODIUM", Some("1"))
        .compile("libspectre.a");
    // .file("src/spectre/api/c/mpw-jni.c")

    // Tell cargo to tell rustc to link the system mpw .so
    // shared library.
    // all handled by metadata
    // export LD_LIBRARY_PATH=/home/timo/Programmieren/rust_mpw/src/masterpassword-c/core/lib/linux/x86_64/
    println!("cargo:rustc-link-search=native={}", "/app/lib/");
    println!("cargo:rustc-link-lib=sodium");
    // cargo:rustc-link-lib=[KIND=]NAME

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/spectre/spectre_wrapper.h")
        // .header("src/spectre/api/c/spectre-types.h")
        // .header("src/spectre/api/c/spectre-marshal.h")
        // .header("src/spectre/api/c/spectre-marshal-util.h")
        // .header("src/spectre/api/c/spectre-util.h")
        // .header("src/spectre/api/c/spectre-algorithm.h")
        // .disable_name_namespacing()
        // .constified_enum_module("*")
        // format the output
        // .hide_type("SpectreKeyPurpose")
        // .constified_enum_module("*")
        .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/8/include/")
        .rustfmt_bindings(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("spectre_bindings.rs")).expect("Couldn't write bindings!");
}

/*Expected build command:
-O3 -DMP_VERSION=2.6-cli-5-106-g180cc3d-dirty -DSPECTRE_SODIUM=1 -Iapi/c -Isrc api/c/base64.c api/c/aes.c api/c/spectre-algorithm.c api/c/spectre-algorithm_v0.c api/c/spectre-algorithm_v1.c api/c/spectre-algorithm_v2.c api/c/spectre-algorithm_v3.c api/c/spectre-types.c api/c/spectre-util.c api/c/spectre-marshal-util.c api/c/spectre-marshal.c src/spectre-cli-util.c -lsodium src/spectre-cli.c -o spectre
*/
