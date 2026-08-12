use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo sets TARGET");
    assert_eq!(
        target, "armv7-unknown-linux-musleabihf",
        "this configured port only supports armv7-unknown-linux-musleabihf"
    );
    assert_eq!(env::var("CARGO_CFG_TARGET_ARCH").as_deref(), Ok("arm"));
    assert_eq!(env::var("CARGO_CFG_TARGET_OS").as_deref(), Ok("linux"));
    assert_eq!(env::var("CARGO_CFG_TARGET_ENV").as_deref(), Ok("musl"));

    cc::Build::new()
        .target(&target)
        .file("asm/arm/sysv.S")
        .include("asm/include")
        .include("asm/arm")
        .flag("-march=armv7-a")
        .flag("-mfloat-abi=hard")
        .flag("-mfpu=vfpv3-d16")
        .warnings(false)
        .compile("ffi_arm_sysv");

    println!("cargo:rerun-if-changed=asm/arm/sysv.S");
    println!("cargo:rerun-if-changed=asm/arm/internal.h");
    println!("cargo:rerun-if-changed=asm/include");
}
