use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo did not set TARGET");
    assert_eq!(
        target, "x86_64-unknown-linux-musl",
        "this port only supports x86_64-unknown-linux-musl",
    );

    println!("cargo:rerun-if-changed=asm/x86/unix64.S");
    println!("cargo:rerun-if-changed=asm/x86/win64.S");
    println!("cargo:rerun-if-changed=asm/x86/rust_call_shim.S");
    println!("cargo:rerun-if-changed=asm/include");
    println!("cargo:rerun-if-changed=asm/x86");

    cc::Build::new()
        .files([
            "asm/x86/unix64.S",
            "asm/x86/win64.S",
            "asm/x86/rust_call_shim.S",
        ])
        .include("asm/include")
        .include("asm/x86")
        .flag_if_supported("-fPIC")
        .compile("ffi_x86_64_asm");
}
