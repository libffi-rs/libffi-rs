# powerpc64-unknown-linux-gnu C2Rust port

## Status

This is a self-contained stable-Rust crate for big-endian PowerPC64 GNU/Linux
(ELFv1), translated from the bundled libffi 3.5.2 at libffi-rs branch `rrir`,
exact revision `893bd63954735468305c83ddd56718e3863fdacd`. The package is
`libffi-c2rust-powerpc64-linux-gnu`. It builds its vendored target assembly and
does not build or link C source or system libffi.

All generation and validation ran on the assigned Ubuntu x86_64 AWS worker
`i-0dad7a6af132b41e6` using `gcc-powerpc64-linux-gnu`, C2Rust 0.22.1, and
`qemu-ppc64`. No EC2 lifecycle operation was performed.

## Source review

Before porting, the recent `rrir` commits `23efc98`, `25ac199`, `8e43bdf`,
`1f3108f`, and `893bd63` were reviewed, along with:

- the current `libffi/build.rs` target assertion and assembly build;
- `libffi-sys-rs/build/{build.rs,common.rs,not_msvc.rs}`;
- `libffi-sys-rs/libffi/configure.host`; and
- the selected PowerPC C, assembly, ABI, and internal headers.

The current checkout's AArch64/macOS translation was used only to understand
the repository's intended crate shape. This port was produced by independently
configuring and translating the selected PowerPC sources below.

## Configuration and compilation database

The bundled upstream source was copied and configured out of tree with the
actual cross tools:

```sh
export CC=powerpc64-linux-gnu-gcc CXX=powerpc64-linux-gnu-g++
export AR=powerpc64-linux-gnu-ar AS=powerpc64-linux-gnu-as
export LD=powerpc64-linux-gnu-ld NM=powerpc64-linux-gnu-nm
export RANLIB=powerpc64-linux-gnu-ranlib STRIP=powerpc64-linux-gnu-strip
export CFLAGS='-O0 -g -fPIC'
./configure --host=powerpc64-unknown-linux-gnu \
  --with-pic --disable-shared --disable-docs
bear --output compile_commands.all.json -- make -j2 V=1
```

Configure detected big endian, 16-byte `long double`, and cross compilation.
The actual build completed as a Power ELFv1 static archive. Bear recorded the
real cross-GCC commands, configured build directory, generated `fficonfig.h`,
generated `ffi.h`, and selected PowerPC headers. `compile_commands.json` is the
C-only nine-entry database used for translation; `compile_commands.all.json`
is the complete Bear result including assembly.

Configured C units:

```text
src/prep_cif.c
src/types.c
src/raw_api.c
src/java_raw_api.c
src/closures.c
src/tramp.c
src/powerpc/ffi.c
src/powerpc/ffi_sysv.c
src/powerpc/ffi_linux64.c
```

Configured assembly units:

```text
src/powerpc/sysv.S
src/powerpc/ppc_closure.S
src/powerpc/linux64.S
src/powerpc/linux64_closure.S
```

The first two assembly files preprocess to inactive content for PPC64, but are
still selected by upstream and deliberately retained and compiled here. The
ELFv1 call and executable-closure implementations are in `linux64.S` and
`linux64_closure.S`.

## C2Rust 0.22.1

A direct C2Rust pass over the GCC database was attempted first and preserved in
`logs/c2rust-target.log`. C2Rust's Clang exporter does not inherit GCC's target
predefined macros and attempted inactive PPC32 code; it also reported an
unsupported `Float128` initializer in `ffi_call_int`.

The successful pass still used the actual configured source files, generated
headers, include paths, and selected units, while supplying the target
predefines explicitly to C2Rust's LP64 Clang frontend:

```sh
c2rust transpile --emit-modules --disable-rustfmt \
  --output-dir generated compile_commands.json -- \
  -D__powerpc64__=1 -D__powerpc__=1 -D__PPC64__=1 -D__PPC__=1 \
  -D__BIG_ENDIAN__=1 -D_BIG_ENDIAN=1 -D_CALL_ELF=1 \
  -D__LONG_DOUBLE_128__=1 -D__LONG_DOUBLE_IBM128__=1
```

This selects the configured big-endian PPC64 ELFv1 paths, including the proper
`FFI_DEFAULT_ABI = FFI_LINUX | FFI_LINUX_LONG_DOUBLE_128` value 10. C2Rust
translated all selected units; `ffi_sysv.rs` is empty as expected for PPC64.
See `logs/c2rust.log`.

## Stable-Rust and target-specific fixes

- C2Rust skipped only `ffi_call_int` because of the `float128 smst_buffer`
  initializer. That function was translated directly from the same configured
  C function. Its opaque 16-byte bounce storage, big-endian right-justification
  for narrow and integer-complex returns, and ELFv1 `ffi_call_LINUX64` path are
  preserved.
- GNU powerpc64 `long double` is IBM double-double: 16-byte size, 16-byte
  alignment, and 106 mantissa bits. Rust has no C-compatible scalar for it, so
  generated code uses opaque 16-byte storage and ABI-correct `ffi_type`
  metadata rather than pretending it is `f64`.
- C2Rust cannot emit C complex static initializers. The float, double, and long
  double complex descriptors and immutable element arrays are defined with
  target sizes/alignments 8/4, 16/8, and 32/16.
- `ffi_cif` retains the PowerPC layout: ABI, argument count, type pointers,
  bytes, flags, and `nfixedargs`; size 40 and alignment 8. The default ABI is
  10, not the host-frontend value 8.
- Removed generated atomic intrinsics were replaced with stable
  `AtomicI32`/`AtomicUsize` operations preserving Relaxed, Acquire, and Release
  ordering. Generated unstable glibc extern types were replaced by opaque
  stable declarations, and generated integer constant types were corrected.
- Architecture assembly and its templates were not translated or rewritten.
  `build.rs` rejects every target except `(powerpc64, linux, big, gnu)` and
  compiles all four upstream selected `.S` files with `cc` and the vendored
  configured/source headers.
- ELFv1 function descriptors are intentional. Consequently `nm` reports public
  function descriptor symbols as `D` and dot-prefixed code entries as `T`.

## Validation

On Ubuntu with the cross compiler, Rust target, and QEMU installed:

```sh
cargo fmt --all -- --check
cargo check --target powerpc64-unknown-linux-gnu
cargo build --target powerpc64-unknown-linux-gnu
cargo test --target powerpc64-unknown-linux-gnu -- --nocapture --test-threads=1
powerpc64-linux-gnu-nm -g --defined-only \
  target/powerpc64-unknown-linux-gnu/debug/liblibffi_c2rust_powerpc64.a
```

QEMU ran six integration tests successfully. They exercise:

- ten integer arguments across GPRs and the stack overflow area;
- thirteen floating-point arguments across FPRs and stack spill;
- big-endian narrow integer returns;
- structure arguments and returns by value;
- an allocated executable closure trampoline invoking a Rust callback; and
- target `ffi_cif`, long-double, and complex descriptor layouts.

The test executable is Power ELFv1 and has only `libgcc_s.so.1`, `libc.so.6`,
and `ld64.so.1` as `DT_NEEDED` entries. It has no system `libffi` dependency,
and its `ffi_call`, `ffi_prep_cif`, closure, and type symbols are defined in the
executable. Exact evidence is in `logs/test.log`, `logs/nm-required.log`, and
`logs/no-system-libffi.log`.

## Reproduction

`.cargo/config.toml` selects `powerpc64-linux-gnu-gcc` and the runner
`qemu-ppc64 -L /usr/powerpc64-linux-gnu`. `scripts/test-qemu.sh` runs formatting,
target check/build/test, symbol checks, and the no-system-libffi check. Absolute
paths in the compilation databases are historical generation evidence only;
no crate build script or Rust source references the source checkout, worker,
`/tmp`, or system libffi.

Benign warnings remain in mechanically translated modules (duplicate
module-local C declarations and unused generated expressions). Stable target
check, build, and all runtime tests pass.
