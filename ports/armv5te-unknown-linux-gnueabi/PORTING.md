# ARMv5TE porting record

## Provenance

- Source: `https://github.com/libffi-rs/libffi-rs.git`, branch `rrir`
- Exact source commit: `893bd63954735468305c83ddd56718e3863fdacd`
- Bundled upstream libffi version: 3.5.2
- Translator: C2Rust 0.22.1
- Build target: `armv5te-unknown-linux-gnueabi` (little-endian ARMv5TE,
  soft-float GNU EABI)
- Configure host: `arm-linux-gnueabi`; compiler flags:
  `-march=armv5te -mfloat-abi=soft -O2 -g0 -fPIC`

The source repository's recent commits and its `libffi/build.rs` and
`libffi-sys-rs/build/{build,not_msvc,msvc,common}.rs` were reviewed before
translation. The full configure/build/C2Rust evidence is retained in `logs/`.

## Configured source selection

Bundled libffi was configured out-of-tree with `--with-pic --disable-shared
--disable-docs`. Configure selected `TARGET=ARM`, `TARGETDIR=arm`, no distinct
long-double type (`sizeof(long double) == sizeof(double) == 8`), static
trampolines, and these units:

- `src/prep_cif.c`
- `src/types.c`
- `src/raw_api.c`
- `src/java_raw_api.c`
- `src/closures.c`
- `src/tramp.c`
- `src/arm/ffi.c`
- `src/arm/sysv.S` (preserved as target assembly)

`logs/compile_commands.full.json` is Bear's exact database from the successful
cross build. `logs/compile_commands.json` is its seven-C-unit C2Rust input.
C2Rust required the extra Clang frontend argument `--target=arm-linux-gnueabi`;
the initial omitted-target diagnostic and successful run are both recorded.
The unmodified generated Rust output is in `logs/c2rust-raw/`.

## Stable-Rust repairs

C2Rust generated the operational body of every selected C unit, with focused
repairs for translator limitations:

- ARM long double is represented by `c_double`, matching configure (8 bytes,
  type code aliases double).
- C2Rust rejected three C complex-valued initializers; the three ordinary
  `ffi_type` complex descriptors were restored with exact configured ARM
  sizes/alignments and pointer arrays.
- C2Rust omitted `ffi_prep_closure_loc` because it cannot lower
  `__builtin___clear_cache`; the function was transcribed and calls libgcc's
  ARM `__clear_cache` implementation. Executable closures validate this path.
- Unstable generated extern types, `VaList`-only unused inline helpers, and
  removed atomic intrinsics were converted to opaque stable types and stable
  `core::sync::atomic` operations.
- C2Rust turns C `alloca` into a heap `Vec`. Upstream `ffi_call_SYSV` assumes
  its prepared area is on the real downward-growing stack. The preserved ARM
  assembly therefore copies prepared argument words to the native stack,
  retains/restores SP and r4 around the call, and leaves upstream's return-type
  dispatch table spacing intact. This is enabled only by the target-asserting
  build script's `FFI_C2RUST_HEAP_STACK` define.

The generated modules intentionally retain translation-unit-local duplicate
C declarations. Rust warns that declarations from separate modules use
nominally different types; all are `repr(C)` copies of the same configured
headers and cross unit boundaries only through C ABI symbols.

## Build and test

The crate is locked to the one target in `build.rs`; a wrong target fails the
build. The exact configured headers and ARM assembly are vendored under
`asm/`. Cargo dependencies are vendored under `vendor/`.

```sh
cargo build --offline --target armv5te-unknown-linux-gnueabi --release
cargo test  --offline --target armv5te-unknown-linux-gnueabi --test ffi
```

The checked-in runner uses `qemu-arm -L /usr/arm-linux-gnueabi`; this runtime
sysroot is only a test runner facility, never a crate build/link dependency.
Tests cover configured layout/long double, integer `ffi_call` in registers,
`ffi_call` with stack arguments, and an allocated executable closure.

See `logs/validation-summary.log`, `logs/nm-symbols.log`,
`logs/test-needed.log`, and `logs/qemu-tests-final.log` for retained evidence.
The test executable defines its libffi symbols itself and has no `NEEDED`
entry for system libffi.
