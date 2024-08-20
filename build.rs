use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the compiled C library.
    println!("cargo:rustc-link-lib=static=mathlib");

    // Compile the C code
    cc::Build::new()
        .file("mathlib/Src/mathlib.c")
        .include("mathlib/Inc")        // Add the Inc directory to the include path
        .compile("mathlib");

    // Generate the bindings to the C library
    let bindings = bindgen::Builder::default()
        .header("mathlib/Inc/mathlib.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
