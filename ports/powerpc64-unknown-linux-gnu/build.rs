use std::env;

fn main() {
    let actual = (
        env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
        env::var("CARGO_CFG_TARGET_OS").unwrap(),
        env::var("CARGO_CFG_TARGET_ENDIAN").unwrap(),
        env::var("CARGO_CFG_TARGET_ENV").unwrap(),
    );
    assert_eq!(
        actual,
        (
            "powerpc64".into(),
            "linux".into(),
            "big".into(),
            "gnu".into()
        ),
        "only big-endian powerpc64 GNU/Linux is supported",
    );

    cc::Build::new()
        .files([
            "asm/powerpc/sysv.S",
            "asm/powerpc/ppc_closure.S",
            "asm/powerpc/linux64.S",
            "asm/powerpc/linux64_closure.S",
        ])
        .include("asm/include")
        .include("asm")
        .include("asm/powerpc")
        .flag("-fPIC")
        .compile("ffi_powerpc64_asm");
}
