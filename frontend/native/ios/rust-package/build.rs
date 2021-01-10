extern crate cbindgen;

use cbindgen::Language::C;

fn main() {
    // let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(".")
        .with_language(C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("lib.h");
}

