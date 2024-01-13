use std::{env, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let bridge_dir = "./libfffs"; // Path to your bridge directory

    // Compile Objective-C code with ARC
    let status = Command::new("clang")
        .args(&[
            "-c",
            format!("{}/fffs.m", bridge_dir).as_str(),
            "-o",
            format!("{}/fffs.o", out_dir).as_str(),
            "-fmodules",
            "-fobjc-arc",
            "-O3",
        ])
        .status()
        .expect("Failed to execute clang");

    if !status.success() {
        panic!("Failed to compile Objective-C code");
    }

    // Create a static library using 'ar'
    Command::new("ar")
        .args(&[
            "rcs",
            format!("{}/libfffs.a", out_dir).as_str(),
            format!("{}/fffs.o", out_dir).as_str(),
        ])
        .status()
        .expect("Failed to create static library");

    println!("cargo:rustc-link-lib=static=fffs");
    println!("cargo:rustc-link-search=native={}", out_dir);
}
