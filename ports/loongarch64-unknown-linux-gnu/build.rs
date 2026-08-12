use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo must set TARGET");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Cargo must set target arch");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("Cargo must set target OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("Cargo must set target env");
    assert_eq!(
        target, "loongarch64-unknown-linux-gnu",
        "this crate is target-specific"
    );
    assert_eq!(
        (arch.as_str(), os.as_str(), target_env.as_str()),
        ("loongarch64", "linux", "gnu"),
        "wrong target configuration"
    );

    println!("cargo:rerun-if-changed=asm/loongarch/sysv.S");
    println!("cargo:rerun-if-changed=asm/loongarch/stack_bridge.c");
    println!("cargo:rerun-if-changed=asm/include/ffi.h");
    println!("cargo:rerun-if-changed=asm/include/ffitarget.h");
    println!("cargo:rerun-if-changed=asm/include/fficonfig.h");

    cc::Build::new()
        .file("asm/loongarch/sysv.S")
        .file("asm/loongarch/stack_bridge.c")
        .include("asm/include")
        .flag("-fPIC")
        .flag("-mcmodel=medium")
        .warnings(false)
        .compile("ffi_loongarch64_sysv");
}
