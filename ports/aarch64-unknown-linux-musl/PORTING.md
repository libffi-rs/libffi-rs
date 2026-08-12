# AArch64 musl C2Rust libffi port

## Provenance and target

- Target: `aarch64-unknown-linux-musl` (AArch64 LP64, little endian, musl)
- Source: `https://github.com/libffi-rs/libffi-rs.git`, branch `rrir`
- Exact source commit: `893bd63954735468305c83ddd56718e3863fdacd`
- Bundled libffi: 3.5.2
- Translator: C2Rust 0.22.1
- Worker: assigned AWS Ubuntu 24.04 AArch64 instance `i-0cee56b91bd471f6a` (Graviton, `aarch64`)
- Rust validation toolchain: stable 1.97.1

The recent `rrir` commits (`23efc98` through `893bd63`), the existing experimental `libffi/` translation, `libffi/build.rs`, and `libffi-sys-rs/build/*.rs` were read before porting. The Rust in this crate was generated anew from the configured AArch64-musl source set; it is not a copy of the macOS translation.

## Configuration and C2Rust

Ubuntu `musl-tools` provides `musl-gcc` but no target-prefixed wrapper, so `musl-gcc` was used directly. Linux UAPI include links were added under `/usr/include/aarch64-linux-musl` for upstream `tramp.c`. The exact configuration was:

```sh
CC=musl-gcc CFLAGS="-O0 -g -fPIC" \
  ../libffi-sys-rs/libffi/configure \
  --build=aarch64-unknown-linux-gnu \
  --host=aarch64-unknown-linux-musl \
  --with-pic --disable-shared --disable-docs
make -j2 V=1
```

Configure selected `TARGET=AARCH64`, these seven C translation units, and one architecture assembly input:

```text
src/prep_cif.c
src/types.c
src/raw_api.c
src/java_raw_api.c
src/closures.c
src/tramp.c
src/aarch64/ffi.c
src/aarch64/sysv.S
```

`compile_commands.json` records Clang's `aarch64-unknown-linux-musl` frontend, musl headers, configured headers, and exactly those seven C files. The C inputs needed to replay translation are retained under `porting/c-source`; Cargo never compiles them. The run was:

```sh
porting/transpile.sh # materializes C2Rust-required absolute paths, then runs:
c2rust transpile porting/compile_commands.absolute.json \
  --output-dir generated --emit-modules --overwrite-existing \
  --translate-const-macros conservative
```

C2Rust translated all seven files. As expected, it reported that the three C complex statics and `__builtin___clear_cache` were unsupported. See `logs/c2rust.log`.

## Target-specific fixes

1. The Linux AArch64 `ffi_cif` layout has six fields and is 32 bytes; it does **not** contain macOS trampoline-table fields or `aarch64_nfixedargs`. `FFI_DEFAULT_ABI` is SYSV (1).
2. AArch64 musl/GCC reports `sizeof(long double) == 16`, alignment 16, mantissa 113: IEEE binary128. `ffi_type_longdouble` therefore uses opaque 16-byte/16-aligned storage, never `c_double`.
3. The omitted complex descriptors were recreated as pairs of their scalar representation. Complex long double is size 32/alignment 16.
4. C2Rust-generated AArch64 vector assembly braces were escaped for Rust `asm!` templates.
5. `sysv.S`, which C2Rust cannot translate, is retained and compiled with configured headers by `build.rs`. A small target assembly cache-flush helper implements the semantics of the omitted `__builtin___clear_cache` for executable closure trampolines.
6. Removed compiler atomics in translated closure allocation were replaced with stable `AtomicI32`/`AtomicUsize` operations preserving relaxed/acquire/release order.
7. C2Rust's unstable opaque `FILE` extern types were replaced by zero-sized opaque C-layout marker structs. Target-sized generated constants were corrected for stable Rust.
8. Upstream call assembly temporarily switches SP to caller-provided storage. The translation provides a 16-byte-aligned 64 KiB downward callee-stack reserve and restores/deallocates it after assembly returns. Closure argument arrays are pointer-aligned.

## Self-contained build and system-libffi exclusion

`build.rs` asserts the exact Cargo target and compiles only `asm/aarch64/sysv.S` and `asm/aarch64/clear_cache.S` through `cc`. `.cargo/config.toml` selects `musl-gcc` for this target. There is no `#[link(name = "ffi")]`, `cargo:rustc-link-lib=ffi`, pkg-config probe, prebuilt libffi, or system-libffi search path. The final archive itself defines the call, closure, prep, and type symbols. Tests are statically linked musl executables with no dynamic dependencies.

## Validation on the assigned worker

Run:

```sh
cargo fmt --all -- --check
cargo check --target aarch64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-musl
cargo test --target aarch64-unknown-linux-musl -- --nocapture --test-threads=1
```

`logs/final-validation.log` records all commands and results, direct execution of the static test binary, ELF/static-link checks, and required-symbol checks. Ten end-to-end tests pass: target layout/default ABI, version, binary128/complex descriptor layout, integer extension, floating point, register-to-stack overflow, small integer calls, mixed structures, AArch64 HFA structure calls/returns, and an executable closure callback.

## Limitations

- This is mechanically translated unsafe Rust and retains warnings about equivalent per-module C declarations and other generated code style.
- Stable Rust has no C ABI binary128 scalar. The descriptor has the correct ABI storage layout, but numeric long-double calls need an appropriate C-compatible shim/storage type.
- The translated `ffi_call` provides 64 KiB of callee stack below its temporary argument context; calls needing more are unsupported.
- Deprecated raw/Java APIs, variadic calls, unwinding, every closure allocator fallback, and non-SYSV ABI values were not exhaustively exercised.
