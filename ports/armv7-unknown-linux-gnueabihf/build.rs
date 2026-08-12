use std::env;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("target arch");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("target OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("target environment");

    assert_eq!(
        (arch.as_str(), os.as_str(), target_env.as_str()),
        ("arm", "linux", "gnu"),
        "this crate only supports armv7-unknown-linux-gnueabihf",
    );
    assert_eq!(
        env::var("TARGET").as_deref(),
        Ok("armv7-unknown-linux-gnueabihf"),
        "this crate only supports armv7-unknown-linux-gnueabihf",
    );

    cc::Build::new()
        .file("asm/arm/sysv.S")
        .include("asm/include")
        .include("asm/arm")
        .flag_if_supported("-fexceptions")
        .compile("ffi_armv7_sysv");

    println!("cargo:rerun-if-changed=asm/arm/sysv.S");
    println!("cargo:rerun-if-changed=asm/arm/internal.h");
    println!("cargo:rerun-if-changed=asm/include/ffi.h");
    println!("cargo:rerun-if-changed=asm/include/ffi_cfi.h");
    println!("cargo:rerun-if-changed=asm/include/fficonfig.h");
    println!("cargo:rerun-if-changed=asm/include/ffitarget.h");
}
