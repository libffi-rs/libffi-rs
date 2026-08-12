use std::env;

fn main() {
    let target = env::var("TARGET").expect("Cargo must set TARGET");
    assert_eq!(
        target, "sparc64-unknown-linux-gnu",
        "this crate is only for sparc64 GNU/Linux"
    );
    assert_eq!(env::var("CARGO_CFG_TARGET_ARCH").as_deref(), Ok("sparc64"));
    assert_eq!(env::var("CARGO_CFG_TARGET_OS").as_deref(), Ok("linux"));
    assert_eq!(env::var("CARGO_CFG_TARGET_ENV").as_deref(), Ok("gnu"));

    println!("cargo:rerun-if-changed=asm/sparc/v9.S");
    println!("cargo:rerun-if-changed=asm/sparc/flush.S");
    println!("cargo:rerun-if-changed=asm/sparc/internal.h");
    println!("cargo:rerun-if-changed=include/ffi.h");
    println!("cargo:rerun-if-changed=include/ffitarget.h");
    println!("cargo:rerun-if-changed=artifacts/fficonfig.h");

    cc::Build::new()
        .target(&target)
        .compiler(
            env::var("CC_sparc64_unknown_linux_gnu")
                .unwrap_or_else(|_| "sparc64-linux-gnu-gcc".into()),
        )
        .file("asm/sparc/v9.S")
        .file("asm/sparc/flush.S")
        .include("artifacts")
        .include("include")
        .include("asm/sparc")
        .flag("-fPIC")
        .warnings(false)
        .compile("ffi_sparc64_asm");
}
