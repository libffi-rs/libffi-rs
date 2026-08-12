# libffi for loongarch64-unknown-linux-gnu (C2Rust)

Self-contained, target-specific Rust port of bundled libffi 3.5.2. The C
implementation was translated with C2Rust 0.22.1; configured headers and the
upstream LoongArch assembly are vendored. Cargo dependencies are also vendored.

A LoongArch64 GNU C compiler/linker is required because `build.rs` assembles the
target source and compiles the small stack bridge.

```sh
export CC_loongarch64_unknown_linux_gnu=loongarch64-unknown-linux-gnu-gcc
export AR_loongarch64_unknown_linux_gnu=loongarch64-unknown-linux-gnu-ar
export CARGO_TARGET_LOONGARCH64_UNKNOWN_LINUX_GNU_LINKER=loongarch64-unknown-linux-gnu-gcc
cargo build --offline --target loongarch64-unknown-linux-gnu
```

For cross-executed tests, set Cargo's runner to `qemu-loongarch64 -L <sysroot>`.
See `PORTING.md` and `logs/` for provenance and validation evidence.
