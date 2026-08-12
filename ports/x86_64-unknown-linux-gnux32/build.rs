use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo did not set TARGET");
    assert_eq!(
        target, "x86_64-unknown-linux-gnux32",
        "this port only supports x86_64-unknown-linux-gnux32"
    );
    println!("cargo:rerun-if-changed=asm");
    cc::Build::new()
        .files(["asm/x86/unix64.S", "asm/x86/rust_call_shim.S"])
        .include("asm/include")
        .include("asm/x86")
        .flag("-mx32")
        .flag_if_supported("-fcf-protection=none")
        .flag_if_supported("-fPIC")
        .compile("ffi_x32_asm");
}
