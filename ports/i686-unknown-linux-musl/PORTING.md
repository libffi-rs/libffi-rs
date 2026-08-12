# i686-unknown-linux-musl C2Rust port

This crate is a target-specific, self-contained port of the bundled libffi in
`libffi-rs` branch `rrir` at exact commit
`893bd63954735468305c83ddd56718e3863fdacd` (`minor fixes`). It was produced on
the assigned Ubuntu x86_64 worker with C2Rust 0.22.1; it was not copied from
another port.

## Target and C baseline

The cross compiler was the musl.cc `i486-linux-musl-cross` GCC 11.2.1 toolchain,
used with `-march=i686`. A static 32-bit probe ran natively on the x86_64 worker.
The bundled libffi was configured out of tree with:

```sh
CC=i486-linux-musl-gcc \
CFLAGS='-march=i686 -O2 -fPIC' \
libffi-sys-rs/libffi/configure \
  --host=i686-unknown-linux-musl --build=x86_64-pc-linux-gnu \
  --disable-shared --enable-static --disable-docs --with-pic
```

Important configured facts were `TARGET=X86`, `TARGETDIR=x86`, 4-byte
`size_t`, a 12-byte/4-aligned x87 `long double`, closures and Go closures
enabled, static executable trampolines enabled, and GCC fastcall enabled. The
configured `ffi_cif` has exactly six 4-byte fields (24 bytes), and the aligned
`ffi_closure` is 32 bytes.

`bear -- make -j2 V=1` captured the actual successful build. The selected units
were:

- `src/prep_cif.c`
- `src/types.c`
- `src/raw_api.c`
- `src/java_raw_api.c`
- `src/closures.c` (including configured dlmalloc code)
- `src/tramp.c`
- `src/x86/ffi.c`
- `src/x86/sysv.S`

The C source entries are in `logs/compile_commands.json`; the full database,
including the target assembly entry, is in
`logs/compile_commands.all.json`. The baseline C archive had the required
`ffi_call`, `ffi_prep_cif`, closure, and long-double symbols.

## Translation

The common C units were translated with the target sysroot and target triple:

```sh
c2rust transpile --emit-build-files \
  --output-dir /home/ubuntu/work/c2rust-output \
  --overwrite-existing --preserve-unused-functions --log-level INFO \
  /home/ubuntu/work/compile_commands.json -- \
  --target=i686-unknown-linux-musl \
  --sysroot=/home/ubuntu/toolchains/i486-linux-musl-cross/i486-linux-musl
```

C2Rust 0.22.1 translated the common units but could not export Clang's
`fastcall` attributed function types in `src/x86/ffi.c`. For translation only,
the fastcall attribute macro was removed (`logs/c2rust-input.patch` and
`logs/compile_commands.x86-patched.json`); that unit was then successfully
translated separately. The generated Rust declarations
and definition were restored to `extern "fastcall"`, matching GCC and
`sysv.S`. The initial bounded failure and successful rerun are both retained in
`logs/c2rust-transpile*.log`.

## Stable-Rust and ABI repairs

The generated code was formatted and repaired without nightly features:

- C2Rust's incorrect `f128` model was replaced with the configured 12-byte,
  4-aligned x87 long-double descriptor.
- C2Rust cannot lower C `_Complex` declarations. The three exported complex
  descriptors and their static component arrays use the configured i686
  layouts (8, 16, and 24 bytes, all 4-aligned).
- The C `__attribute__((aligned(8)))` on `ffi_closure` was restored.
- Unstable generated extern types for `FILE` were replaced by opaque `repr(C)`
  structs.
- Removed compiler atomic intrinsics in the translated dlmalloc were replaced
  by stable 32-bit `AtomicU32` operations with the original orderings.
- One target-size-dependent dlmalloc expression was corrected to calculate in
  `usize` and narrow back to the configured 32-bit `size_t`.
- The generated x86 call frame was extended with its argument-byte count and
  saved copy registers. C2Rust lowers C `alloca` to a heap `Vec`; upstream
  `ffi_call_i386` normally turns that alloca area into the machine stack. The
  vendored target assembly therefore copies the prepared bytes from the Vec to
  the real stack before calling and restores the caller stack afterward. This
  prevents both heap allocator corruption and an artificial bounded callee
  stack. The upstream return dispatch, x87 handling, closure entry points,
  trampolines, and target assembly remain in `vendor/x86/sysv.S`.

`build.rs` rejects every target except exact `i686-unknown-linux-musl` and also
asserts x86, Linux, musl, and 32-bit pointers. It compiles the preserved assembly
with `cc`, `-march=i686`, and only the vendored configured headers. There is no
system-libffi probe or fallback.

## Build and validation

Put `i486-linux-musl-gcc` on `PATH`, install the Rust target, then run:

```sh
rustup target add i686-unknown-linux-musl
cargo fmt --all -- --check
cargo check --target i686-unknown-linux-musl
cargo build --release --target i686-unknown-linux-musl
cargo test --release --target i686-unknown-linux-musl
```

The included Cargo config selects that linker/compiler by tool name. No worker,
temporary-directory, configured-build, or system-libffi path is used by the
crate.

Both debug and optimized tests passed natively on the x86_64 worker. The seven
integration tests cover exact layouts and type descriptors, integer and deep
stack calls, mixed floating-point calls, struct argument/return, a real x87
80-bit long-double call, and an allocated executable closure round trip.

The optimized test executable was an ELF32 Intel 80386 static executable with
no dynamic section and no interpreter; `ldd` reported `not a dynamic
executable`. Required symbols were defined in both the produced static archive
and test executable. Runtime file tracing and the verbose Cargo link log showed
no system `libffi` load or link request. See:

- `logs/cargo-fmt-check.log`
- `logs/cargo-check.log`
- `logs/cargo-build-release-vv.log`
- `logs/cargo-test.log`
- `logs/cargo-test-release.log`
- `logs/validation.log`
- `logs/no-system-libffi.log`
- `logs/strace-test.log`

The generated Rust remains a direct unsafe C2Rust translation and should be
treated with the same trust assumptions as the original C implementation.
