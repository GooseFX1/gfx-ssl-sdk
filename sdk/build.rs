use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-lib=dylib=gfx_black_box");

    println!("cargo:rustc-link-search={}/lib", manifest_dir);
}
