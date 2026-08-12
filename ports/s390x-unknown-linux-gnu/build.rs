use std::env;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("target architecture");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("target OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("target environment");

    assert_eq!(
        (arch.as_str(), os.as_str(), target_env.as_str()),
        ("s390x", "linux", "gnu"),
        "only s390x GNU/Linux is supported",
    );

    println!("cargo:rerun-if-changed=asm/s390/sysv.S");
    println!("cargo:rerun-if-changed=asm/s390/call_int.S");
    println!("cargo:rerun-if-changed=asm/s390/internal.h");
    println!("cargo:rerun-if-changed=asm/include/ffi.h");
    println!("cargo:rerun-if-changed=asm/include/ffitarget.h");
    println!("cargo:rerun-if-changed=asm/include/fficonfig.h");

    cc::Build::new()
        .files(["asm/s390/sysv.S", "asm/s390/call_int.S"])
        .include("asm/include")
        .include("asm/s390")
        .compile("ffi_s390x_asm");
}
