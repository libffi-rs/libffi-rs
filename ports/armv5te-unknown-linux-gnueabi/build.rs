use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo did not provide TARGET");
    assert_eq!(
        target, "armv5te-unknown-linux-gnueabi",
        "this configured port is only valid for ARMv5TE soft-float GNU EABI"
    );
    assert_eq!(env::var("CARGO_CFG_TARGET_ARCH").as_deref(), Ok("arm"));
    assert_eq!(env::var("CARGO_CFG_TARGET_ENV").as_deref(), Ok("gnu"));
    assert_eq!(env::var("CARGO_CFG_TARGET_ENDIAN").as_deref(), Ok("little"));
    assert_eq!(
        env::var("CARGO_CFG_TARGET_POINTER_WIDTH").as_deref(),
        Ok("32")
    );

    cc::Build::new()
        .file("asm/arm/sysv.S")
        .include("asm/include")
        .include("asm/arm")
        .flag("-march=armv5te")
        .flag("-mfloat-abi=soft")
        .define("FFI_BUILDING", None)
        .define("FFI_C2RUST_HEAP_STACK", None)
        .warnings(false)
        .compile("ffi_armv5te_asm");

    println!("cargo:rerun-if-changed=asm/arm/sysv.S");
    println!("cargo:rerun-if-changed=asm/arm/internal.h");
    println!("cargo:rerun-if-changed=asm/include/ffi.h");
    println!("cargo:rerun-if-changed=asm/include/ffitarget.h");
    println!("cargo:rerun-if-changed=asm/include/fficonfig.h");
    println!("cargo:rerun-if-changed=asm/include/ffi_cfi.h");
}
