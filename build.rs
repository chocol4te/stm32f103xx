use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        // Put the linker script somewhere the linker can find it
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("interrupts.x"))
            .unwrap()
            .write_all(include_bytes!("interrupts.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());

        println!("cargo:rerun-if-changed=interrupts.x");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
