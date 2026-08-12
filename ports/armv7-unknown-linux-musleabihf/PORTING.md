# ARMv7 hard-float musl C2Rust port

## Provenance

* Source: `https://github.com/libffi-rs/libffi-rs.git`, branch `rrir`
* Exact source HEAD: `893bd63954735468305c83ddd56718e3863fdacd`
* Upstream bundled libffi reports version 3.5.2.
* Translation worker: assigned Ubuntu x86_64 EC2 worker; no lifecycle operations were performed.
* Translator: C2Rust 0.22.1.
* C toolchain: musl.cc `arm-linux-musleabihf-gcc` 11.2.1.
* Rust validation: stable 1.97.1; no nightly features remain.
* Execution: qemu-arm 8.2.2.

Before porting, the recent `rrir` commits and `libffi/build.rs`,
`libffi-sys-rs/build/build.rs`, `common.rs`, `msvc.rs`, and `not_msvc.rs` were
reviewed. `logs/source-review.log` records the commit summary and hashes of the
reviewed build scripts.

This is a fresh translation, not a copy of another port. The actual bundled
libffi tree from that commit was configured with:

```text
CC=arm-linux-musleabihf-gcc CXX=arm-linux-musleabihf-g++ \
CFLAGS='-O2 -g0 -fPIC -march=armv7-a -mfloat-abi=hard -mfpu=vfpv3-d16' \
./configure --host=arm-linux-musleabihf --build=x86_64-pc-linux-gnu \
  --with-pic --disable-shared --disable-docs
```

Configure selected `src/arm/ffi.c` and `src/arm/sysv.S`, detected 32-bit
`size_t`, little endian, no `__int128_t`, and 8-byte `long double`. Bear
captured all eight actual compiler invocations. C2Rust was run over the seven
selected C translation units with explicit Clang target and musl sysroot:

```text
c2rust transpile compile_commands.c.json --output-dir translated \
  --emit-build-files -- \
  --target=armv7-unknown-linux-musleabihf \
  --sysroot=$MUSL_CROSS/arm-linux-musleabihf
```

The selected C sources and configured headers are retained under
`provenance/`. `compile_commands.json` is the equivalent relocatable database;
the untouched Bear output is in `logs/compile_commands.raw.json`.

## Necessary translation repairs

C2Rust translated all seven C files, while reporting two known unsupported C
constructs in `logs/c2rust.log`:

1. C complex definitions in `types.c`: the three exported complex `ffi_type`
   statics were written explicitly. Their element arrays, sizes, and
   alignments match the configured ARM ABI. Since ARM musl `long double` is
   binary64 here, `ffi_type_longdouble` is 8-byte sized/aligned and complex
   long double is 16 bytes.
2. GCC `__clear_cache` in `ffi_prep_closure_loc`: the skipped function was
   implemented in stable Rust and uses Linux ARM's private `cacheflush`
   syscall before publishing the trampoline.

Additional stable-Rust repairs replace C2Rust's nightly atomic intrinsics with
`core::sync::atomic`, replace two experimental opaque `FILE` extern types with
zero-sized opaque structs, and cast generated `usize` constants correctly.

C2Rust lowers `alloca` to a `Vec`, but ARM `sysv.S` temporarily installs that
buffer as SP. The assembly therefore records/restores the real caller SP in an
extended internal `call_frame`, while Rust reserves 1 MiB of downward-growing
callee stack. This is internal and does not change public `ffi_cif` or closure
layout. The closure allocator uses a private anonymous 4 KiB RWX mapping on
Linux, avoiding external temporary files and preserving executable closure
semantics. Production users with strict W^X policies should replace this with
a platform-specific dual mapping.

The target assembly is otherwise preserved from the configured source and is
compiled by target-asserting `build.rs` with the vendored configured headers.
The crate has no path, temporary-directory, remote, or system-libffi dependency.

## Validation

The worker validation commands and full output are retained under `logs/`:

* `cargo fmt --check`
* `cargo check --target armv7-unknown-linux-musleabihf`
* `cargo build --release --target armv7-unknown-linux-musleabihf`
* target `nm` checks for `ffi_call`, CIF, closure, assembly, and type symbols
* qemu execution of all tests

The qemu tests verify the 48-byte ARM `ffi_cif`, configured long-double and
complex descriptors, an integer `ffi_call` returning 42, and an allocated
executable closure returning 42. The test executable is a statically linked
ARM EABI5 ELF. `readelf -d` reports no dynamic section, and the recorded linker
line has no `-lffi`; therefore it cannot load system libffi.

Run `./test-qemu.sh` with `arm-linux-musleabihf-gcc`, its binutils, qemu-arm,
and the Rust target installed and available in `PATH`.
