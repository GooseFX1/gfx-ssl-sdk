fn main() {
    println!("cargo:rustc-link-lib=dylib=gfx_black_box");

    println!("cargo:rustc-link-search=./lib");
}
