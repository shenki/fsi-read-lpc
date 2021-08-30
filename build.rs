use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    println!("Target: {}", target);

    if target.contains("arm") {
        println!("cargo:rustc-link-search=lib/arm/");
    } else if target.contains("powerpc64le") {
        println!("cargo:rustc-link-search=lib/ppc64le/");
    } else if target.contains("x86") {
        println!("cargo:rustc-link-search=lib/x86/");
    } else {
        panic!("Unknown target: {}", target);
    }

    println!("cargo:rustc-link-lib=static=pdbg");
    println!("cargo:rustc-link-lib=static=fdt");
}
