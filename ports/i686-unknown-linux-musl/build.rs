use std::env;

const TARGET: &str = "i686-unknown-linux-musl";

fn main() {
    let target = env::var("TARGET").expect("Cargo did not set TARGET");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("missing target arch");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("missing target OS");
    let env_name = env::var("CARGO_CFG_TARGET_ENV").expect("missing target env");
    let pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").expect("missing pointer width");

    assert_eq!(target, TARGET, "this port is target-specific");
    assert_eq!(arch, "x86", "this port requires 32-bit x86");
    assert_eq!(os, "linux", "this port requires Linux");
    assert_eq!(env_name, "musl", "this port requires musl");
    assert_eq!(pointer_width, "32", "this port requires ILP32");

    println!("cargo:rerun-if-changed=vendor/x86/sysv.S");
    println!("cargo:rerun-if-changed=vendor/x86/internal.h");
    println!("cargo:rerun-if-changed=vendor/include/ffi.h");
    println!("cargo:rerun-if-changed=vendor/include/ffi_cfi.h");
    println!("cargo:rerun-if-changed=vendor/include/fficonfig.h");
    println!("cargo:rerun-if-changed=vendor/include/ffitarget.h");

    cc::Build::new()
        .target(TARGET)
        .file("vendor/x86/sysv.S")
        .include("vendor/include")
        .include("vendor/x86")
        .flag("-march=i686")
        .flag("-fPIC")
        .warnings(false)
        .compile("ffi_i686_sysv");
}
