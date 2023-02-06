// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rustc-link-lib=librusty_v8_debug_x86_64-unknown-linux-gnu.a");
    // // Use the `cc` crate to build a C file and statically link it.
    // cc::Build::new()
    //     .file("src/hello.c")
    //     .compile("hello");
}