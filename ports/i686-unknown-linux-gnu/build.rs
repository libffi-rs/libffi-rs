use std::env;
fn main() {
    let target = env::var("TARGET").expect("Cargo did not set TARGET");
    assert_eq!(
        target, "i686-unknown-linux-gnu",
        "this port only supports i686-unknown-linux-gnu"
    );
    assert_eq!(env::var("CARGO_CFG_TARGET_ARCH").as_deref(), Ok("x86"));
    assert_eq!(env::var("CARGO_CFG_TARGET_OS").as_deref(), Ok("linux"));
    assert_eq!(env::var("CARGO_CFG_TARGET_ENV").as_deref(), Ok("gnu"));
    cc::Build::new()
        .files([
            "asm/x86/sysv.S",
            "asm/x86/rust_call_shim.S",
            "asm/x86/rust_call_shim.S",
        ])
        .include("asm/include")
        .include("asm/x86")
        .flag("-m32")
        .flag_if_supported("-fPIC")
        .flag_if_supported("-fexceptions")
        .compile("ffi_i686_sysv");
    for path in [
        "asm/x86/sysv.S",
        "asm/x86/rust_call_shim.S",
        "asm/x86/internal.h",
        "asm/x86/asmnames.h",
        "asm/include/ffi.h",
        "asm/include/ffi_cfi.h",
        "asm/include/fficonfig.h",
        "asm/include/ffitarget.h",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
}
