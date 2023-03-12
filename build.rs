extern crate bindgen;

use std::env;
use std::path::PathBuf;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn print_core() {
    println!(
        "cargo:rustc-link-search=native={}/Core/lib/macos/arm64",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn print_core() {
    println!(
        "cargo:rustc-link-search=native={}/Core/lib/macos/x86_64",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn print_core() {
    println!(
        "cargo:rustc-link-search=native={}/Core/lib/linux/x86_64",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn print_core() {
    println!(
        "cargo:rustc-link-search=native={}/Core/lib/windows/x86",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn print_core() {
    println!(
        "cargo:rustc-link-search=native={}/Core/lib/windows/x86_64",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}

fn main() {
    print_core();
    println!("cargo:rustc-link-lib=static=Live2DCubismCore");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // .default_enum_style(bindgen::EnumVariation::Rust {
        //     non_exhaustive: false,
        // })
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
