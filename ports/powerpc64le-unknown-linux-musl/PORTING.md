# powerpc64le-unknown-linux-musl C2Rust port

## Provenance

* Source: `https://github.com/libffi-rs/libffi-rs.git`, branch `rrir`
* Exact HEAD: `893bd63954735468305c83ddd56718e3863fdacd`
* Translation worker: Ubuntu x86_64 EC2 `i-028d3d96e47fdae15`
* Translator: C2Rust 0.22.1 (`logs/c2rust-install.log`)
* Target compiler: musl.cc `powerpc64le-linux-musl-gcc` 11.2.1
* Validation: Rust 1.97.1 and QEMU ppc64le 8.2.2

The recent `rrir` commits and the repository `libffi/build.rs`, `libffi-sys-rs/build/build.rs`, and non-MSVC build logic were reviewed first. This crate was generated from the bundled libffi at that HEAD, not from another port.

## Configuration and translation

Bundled libffi was configured out of tree with:

```sh
CC=powerpc64le-linux-musl-gcc \
AR=powerpc64le-linux-musl-ar \
RANLIB=powerpc64le-linux-musl-ranlib \
CFLAGS="-O2 -g -fno-omit-frame-pointer" \
libffi/configure --host=powerpc64le-linux-musl \
  --build=x86_64-pc-linux-gnu --disable-shared --enable-static
bear --output compile_commands.full.json -- make -j2 V=1
```

Configuration detected little endian, 64-bit pointers, 8-byte `double`, and an 8-byte target `long double`. The build selected `prep_cif.c`, `types.c`, `raw_api.c`, `java_raw_api.c`, `closures.c`, `tramp.c`, `powerpc/ffi.c`, `powerpc/ffi_sysv.c`, and `powerpc/ffi_linux64.c`, plus the four PowerPC assembly units. `ffi_sysv.c`, `sysv.S`, and `ppc_closure.S` preprocess to inactive/empty code for POWERPC64; they remain in the full database/build and the assembly is vendored.

`c2rust transpile --emit-modules --emit-no-std --emit-c-decl-map --fail-on-error` was run over the configured commands. The exact databases, declaration maps, raw translated Rust, patched `ffi.c` input, and logs are under `c2rust/` and `logs/`. `logs/c2rust-active-final.log` records a clean C2Rust run over all eight active C units (`RC=0`).

C2Rust 0.22.1 cannot lower a `_Float128` automatic initializer and its target Clang path aborts on PowerPC MMA builtin types. For `powerpc/ffi.c`, the 128-byte bounce buffer input was equivalently expressed as an aligned byte array. For `ffi_linux64.c`, the configured headers and PowerPC preprocessor definitions were retained while using the host frontend data model (which is also LP64); target-specific constants/layout were checked against the target-generated headers and corrected after translation. Failed attempts are preserved verbatim in the C2Rust logs.

## Stable-Rust cleanup

* `_Float128` storage is an explicitly 16-byte-aligned 16-byte opaque Rust type. Target `long double` descriptors remain 8-byte size/alignment as configured.
* C2Rust’s wrong 32-bit `FFI_LAST_ABI` and SYSV dispatch in `powerpc/ffi.rs` were corrected to Linux64 (`FFI_LAST_ABI = 16`, Linux64 prep/call/closure entry points, 64-bit stack-size argument).
* The small-structure bounce buffer is explicitly 16-byte aligned.
* Complex and int128 descriptor statics were restored with target sizes, alignments, type tags, and initialized element pointers.
* C2Rust’s x86-only `att_syntax` asm option was removed from the PowerPC cache-flush templates; the instructions and operands are unchanged.
* The Linux `closures.c` translation pulled in unstable intrinsic-heavy dlmalloc code. It was reduced to a stable mmap-backed executable closure allocator. Configured static trampolines are declined at runtime, selecting libffi’s ELFv2 inline closure trampoline. No system libffi is used.
* `build.rs` asserts the exact target, Linux/musl, little endian, and 64-bit width before compiling the configured vendored assembly with the vendored configured headers.

## Validation

Run with the musl cross tools and QEMU in `PATH`:

```sh
scripts/validate.sh
```

Recorded validation includes formatting, target check/release build, required `nm` symbols, `ffi_call` with two 64-bit arguments, and an allocated executable closure called under QEMU. The test executable is a statically linked PowerPC ELFv2 binary; `readelf -d` reports no dynamic section, proving it cannot load system libffi. See `logs/`.
