use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Compile our assembly boot code into an object file.
    let status = Command::new("aarch64-elf-gcc")
        .args(&["-c", "src/boot.S", "-o", out_dir.join("boot.o").to_str().unwrap()])
        .status()
        .expect("Failed to execute aarch64-elf-gcc");

    if !status.success() {
        panic!("aarch64-elf-gcc failed");
    }

    // Tell the Rust compiler to pass our compiled boot code directly to the linker.
    println!("cargo:rustc-link-arg={}", out_dir.join("boot.o").display());
    println!("cargo:rerun-if-changed=src/boot.S");
}