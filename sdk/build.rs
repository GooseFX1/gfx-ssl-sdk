fn main() {
    println!("cargo:rustc-link-lib=gfx_black_box");

    println!("cargo:rustc-link-search=./lib");
    // NOTE: When replacing the above with the line below,
    // the -L flag appears last, which I think is what we want.
    // But then, the following error occurs:
    // "cannot find -lgfx_black_box: No such file or directory"
    // So I feel like that is close to a solution to the linking error.
    //println!("cargo:rustc-link-arg=-L ./lib");
}
