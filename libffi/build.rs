use std::env;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    assert_eq!(
        (arch.as_str(), os.as_str()),
        ("aarch64", "macos"),
        "only aarch64 macOS",
    );

    cc::Build::new()
        .file("asm/aarch64/sysv.S")
        .include("asm/include")
        .include("asm/aarch64")
        .compile("ffi_sysv");
}
