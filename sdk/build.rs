use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-lib=dylib=gfx_black_box");

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    println!("cargo:rustc-link-search={}/lib/darwin/arm64", manifest_dir);

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    println!("cargo:rustc-link-search={}/lib/darwin/x86_64", manifest_dir);

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    println!("cargo:rustc-link-search={}/lib/linux/x86_64", manifest_dir);
}
