extern crate bindgen;

use std::env;
use std::path::{PathBuf, Path};

fn main() {
    println!("cargo:rustc-link-lib=static=feast");
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=blas");

    println!("cargo:rerun-if-changed=wrapper.h");

    let feast_dir = env::var("FEAST_ROOT").unwrap();
    //let feast_include_dir = Path::new(&feast_dir).join("/include");
    println!("cargo:rustc-link-search=native={}", Path::new(&feast_dir)
        .join("lib/x64").display());

    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("wrapper.h")
    //     .clang_arg("-I/opt/FEAST/4.0/include")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");
    //
    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
