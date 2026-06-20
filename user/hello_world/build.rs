fn main() {
    println!("cargo:rustc-link-arg=-Tuser/hello_world/linker.ld");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=linker.ld");
}
