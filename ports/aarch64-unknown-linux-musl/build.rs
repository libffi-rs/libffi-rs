use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo did not set TARGET");
    assert_eq!(
        target, "aarch64-unknown-linux-musl",
        "this port only supports aarch64-unknown-linux-musl",
    );

    println!("cargo:rerun-if-changed=asm/aarch64/sysv.S");
    println!("cargo:rerun-if-changed=asm/aarch64/clear_cache.S");
    println!("cargo:rerun-if-changed=asm/include");
    println!("cargo:rerun-if-changed=asm/aarch64");

    cc::Build::new()
        .files(["asm/aarch64/sysv.S", "asm/aarch64/clear_cache.S"])
        .include("asm/include")
        .include("asm/aarch64")
        .flag_if_supported("-fPIC")
        .compile("ffi_aarch64_asm");
}
