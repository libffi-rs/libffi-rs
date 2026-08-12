use std::env;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let endian = env::var("CARGO_CFG_TARGET_ENDIAN").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    assert_eq!(
        (
            arch.as_str(),
            os.as_str(),
            endian.as_str(),
            target_env.as_str(),
        ),
        ("powerpc64", "linux", "little", "gnu"),
        "only powerpc64le GNU/Linux is supported",
    );

    cc::Build::new()
        .files(["asm/powerpc/linux64.S", "asm/powerpc/linux64_closure.S"])
        .include("asm/include")
        .include("asm/powerpc")
        .flag("-fPIC")
        .compile("ffi_powerpc64le_asm");
}
