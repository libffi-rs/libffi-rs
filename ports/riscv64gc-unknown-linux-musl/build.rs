use std::env;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("CARGO_CFG_TARGET_ENV");
    let width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").expect("CARGO_CFG_TARGET_POINTER_WIDTH");
    assert_eq!(
        (
            arch.as_str(),
            os.as_str(),
            target_env.as_str(),
            width.as_str()
        ),
        ("riscv64", "linux", "musl", "64"),
        "this configured port only supports riscv64gc-unknown-linux-musl",
    );
    println!("cargo:rerun-if-changed=asm/riscv/sysv.S");
    println!("cargo:rerun-if-changed=asm/riscv/internal.h");
    println!("cargo:rerun-if-changed=asm/include/ffi.h");
    println!("cargo:rerun-if-changed=asm/include/ffitarget.h");
    println!("cargo:rerun-if-changed=asm/include/fficonfig.h");
    println!("cargo:rerun-if-changed=asm/include/ffi_cfi.h");
    cc::Build::new()
        .file("asm/riscv/sysv.S")
        .include("asm/include")
        .include("asm/riscv")
        .warnings(false)
        .compile("ffi_riscv_sysv");
}
