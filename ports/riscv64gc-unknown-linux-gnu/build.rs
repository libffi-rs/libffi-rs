use std::env;

fn main() {
    let target = env::var("TARGET").expect("TARGET");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("target arch");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("target OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("target env");

    assert_eq!(
        (
            target.as_str(),
            arch.as_str(),
            os.as_str(),
            target_env.as_str()
        ),
        ("riscv64gc-unknown-linux-gnu", "riscv64", "linux", "gnu"),
        "this crate only supports riscv64gc-unknown-linux-gnu",
    );

    println!("cargo:rerun-if-changed=asm/riscv/sysv.S");
    println!("cargo:rerun-if-changed=asm/riscv/internal.h");
    println!("cargo:rerun-if-changed=asm/include/fficonfig.h");
    println!("cargo:rerun-if-changed=asm/include/ffi.h");
    println!("cargo:rerun-if-changed=asm/include/ffitarget.h");
    println!("cargo:rerun-if-changed=asm/include/ffi_cfi.h");

    cc::Build::new()
        .file("asm/riscv/sysv.S")
        .include("asm/include")
        .include("asm/riscv")
        .flag_if_supported("-march=rv64gc")
        .flag_if_supported("-mabi=lp64d")
        .compile("ffi_riscv_sysv");
}
