use std::env;
fn main() {
    let target = env::var("TARGET").expect("TARGET");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("target arch");
    let os = env::var("CARGO_CFG_TARGET_OS").expect("target os");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("target env");
    let endian = env::var("CARGO_CFG_TARGET_ENDIAN").expect("target endian");
    let width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").expect("pointer width");
    assert_eq!(
        target, "powerpc64le-unknown-linux-musl",
        "this crate is target-specific"
    );
    assert_eq!(
        (arch.as_str(), os.as_str(), target_env.as_str()),
        ("powerpc64", "linux", "musl")
    );
    assert_eq!((endian.as_str(), width.as_str()), ("little", "64"));
    cc::Build::new()
        .files([
            "asm/powerpc/sysv.S",
            "asm/powerpc/ppc_closure.S",
            "asm/powerpc/linux64.S",
            "asm/powerpc/linux64_closure.S",
        ])
        .include("asm")
        .include("asm/include")
        .include("asm/powerpc")
        .define("HAVE_CONFIG_H", None)
        .flag_if_supported("-fno-lto")
        .warnings(false)
        .compile("ffi_powerpc64le_asm");
    println!("cargo:rerun-if-changed=asm");
}
