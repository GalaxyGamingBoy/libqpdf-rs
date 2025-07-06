use std::{env, path::PathBuf};

fn main() {
    let libpath = PathBuf::from("libqpdf").canonicalize().unwrap();
    let header_path = libpath.join("qpdf").join("qpdf-c.h");
    let header_str = header_path.to_str().unwrap();

    println!("cargo:rustc-link-lib=static=qpdf");
    println!("cargo:rustc-link-lib=c++");
    if cfg!(windows) {
        println!("cargo:rustc-link-search=native=C:\\libs\\qpdf");
    } else {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
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
