use std::{env, path::PathBuf};

fn main() {
    let libpath = PathBuf::from("libqpdf").canonicalize().unwrap();
    let header_path = libpath.join("qpdf").join("qpdf-c.h");
    let header_str = header_path.to_str().unwrap();

    println!("cargo:rustc-link-lib=static=qpdf");

    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=gnutls");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=jpeg");

    if cfg!(windows) {
        println!("cargo:rustc-link-search=native=C:\\libs\\qpdf");
    }

    if cfg!(unix) {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/openssl@3/lib");
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/gnutls/lib");
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/zlib/lib");
        println!("cargo:rustc-link-search=native=/opt/homebrew/opt/jpeg/lib");
    }

    let bindings = bindgen::Builder::default()
        .header(header_str)
        .clang_arg(format!("-I{}", libpath.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out_path).unwrap();
    bindings
        .write_to_file(PathBuf::from(".bindings.debug.rs"))
        .unwrap()
}
