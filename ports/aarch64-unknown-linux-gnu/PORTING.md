# AArch64 GNU/Linux C2Rust libffi port

## Scope and status

- Rust target: `aarch64-unknown-linux-gnu`
- Source: `https://github.com/libffi-rs/libffi-rs.git`, branch `rrir`, detached commit `893bd63954735468305c83ddd56718e3863fdacd`
- Vendored libffi: 3.5.2
- Translator: C2Rust 0.22.1
- Worker: the assigned AWS Graviton instance `i-0f9401cf7efdec4f5` (Ubuntu 24.04, native AArch64)
- Rust: stable 1.97.1
- Native compilers: GCC 13.3.0 and Clang 18.1.3

This is a native, tested port. Translation, compilation, linking, `ffi_call`, and executable closure tests all ran directly on the assigned Graviton worker. No Docker, QEMU, cross execution, or system libffi was used. The instance lifecycle was not modified.

The package and Rust library names are target-specific. `build.rs` asserts the complete target tuple and compiles only the retained AArch64 assembly. It emits no system-libffi link directive.

## Source review

Before translation, the recent commits and all current standalone-port inputs were inspected, including `libffi/`, `libffi/build.rs`, `libffi/tests/e2e.rs`, and `libffi-sys-rs/build/*.rs`. The recent `build.rs` target assertion plus `cc` assembly-compilation pattern is retained here. Evidence is in:

- `logs/recent-commits.log`
- `logs/recent-key-diffs.log`
- `logs/inspected-files.log`

## Native configure and compilation database

The vendored source was configured out of tree on the AArch64 worker:

```sh
mkdir -p /home/ubuntu/work/build
cd /home/ubuntu/work/build
CC=clang CFLAGS='-O0 -g -fPIC' \
  /home/ubuntu/libffi-rs/libffi-sys-rs/libffi/configure \
  --with-pic --disable-shared --disable-docs
bear --output /home/ubuntu/work/compile_commands.bear.json -- make -j2 V=1
```

Configure identified all build, host, and target tuples as `aarch64-unknown-linux-gnu`. It measured `double` as 8 bytes and `long double` as 16 bytes, detected `__int128_t`, and selected exactly these C translation units:

```text
src/prep_cif.c
src/types.c
src/raw_api.c
src/java_raw_api.c
src/closures.c
src/tramp.c
src/aarch64/ffi.c
```

It selected `src/aarch64/sysv.S` as the sole assembly input. The C-only database passed to C2Rust is preserved byte-for-byte as `porting/compile_commands.absolute.json` and `logs/compile_commands.exact.json`. Its absolute paths are historical generation evidence only; Cargo does not read it. The crate itself has no remote-path dependency.

See `logs/configure.log` and `logs/configured-build.log` for the complete configure and verbose native build output.

## C2Rust invocation

C2Rust was installed and run natively with:

```sh
export LIBCLANG_PATH=/usr/lib/llvm-18/lib
export LLVM_CONFIG_PATH=/usr/bin/llvm-config-18
cd /home/ubuntu/work
c2rust transpile --emit-modules --fail-on-error compile_commands.json
c2rust transpile --emit-modules compile_commands.json
```

The first strict invocation stopped at C2Rust's unsupported `_Complex` static initializer. The second invocation translated all seven configured C files and intentionally reported only the three complex statics and Clang's `__builtin___clear_cache` as unsupported. Those target-specific omissions were then restored manually. See `logs/c2rust-transpile.log` and `logs/c2rust-transpile-allow-errors.log`.

The Rust sources in `src/` are this Linux/AArch64 run's output, not copies of the existing macOS translation.

## Retained assembly and headers

C2Rust cannot translate `src/aarch64/sysv.S`, so it is retained as `asm/aarch64/sysv.S`. `build.rs` compiles it with `cc` after asserting exactly:

```text
TARGET=aarch64-unknown-linux-gnu
arch=aarch64, os=linux, env=gnu
```

Only the selected assembly's required configured headers are included:

```text
asm/include/fficonfig.h
asm/include/ffi.h
asm/include/ffitarget.h
asm/include/ffi_cfi.h
asm/aarch64/internal.h
```

The assembly supplies the `ffi_call_SYSV` and SYSV closure entry points. Required symbols are checked in `logs/final-validation.log` and listed fully in `logs/nm-full.log`.

## Target ABI and stable-Rust fixes

1. **Linux `ffi_cif`:** every generated definition has the configured 32-byte GNU/Linux layout: ABI, argument count, two pointers, byte count, and flags. It does not retain macOS's `aarch64_nfixedargs`. `FFI_DEFAULT_ABI` is `FFI_SYSV` (`1`).
2. **GNU AArch64 long double:** C2Rust's unavailable `::f128::f128` spelling was replaced by a 16-byte, 16-aligned `F128` ABI-storage type. `ffi_type_longdouble` remains kind 4, size 16, alignment 16. It is not represented as `c_double`.
3. **Complex descriptors:** C2Rust 0.22.1 cannot emit C complex-valued static initializers. The target's omitted descriptors were restored as complex float (8/4), complex double (16/8), and complex long double (32/16), each with the correct element descriptor.
4. **Cache maintenance:** the omitted `ffi_clear_cache` wrapper was restored using GNU AArch64 libgcc's `__clear_cache`. Both the writable trampoline and executable code mapping are flushed, matching upstream.
5. **Rust asm templates:** six AArch64 structure-store register lists had their literal braces escaped for Rust's `asm!` parser.
6. **Stable Rust:** experimental generated extern types became opaque C-layout marker structs. Removed C2Rust atomic intrinsics became stable `AtomicI32`/`AtomicUsize` operations with the translated Relaxed, Acquire, and Release orderings. Target-sized constants were corrected.
7. **Translated `alloca`:** upstream assembly temporarily uses the call allocation as SP. C2Rust changed the allocation to a heap vector, so `ffi_call` now uses 16-byte-aligned storage with 1 MiB of downward-growing callee stack headroom before the call context. The real Rust SP is restored by retained assembly before the vector is freed.

## Build and validation

On native AArch64 GNU/Linux:

```sh
cargo fmt --check
cargo check --target aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cargo test --target aarch64-unknown-linux-gnu -- --test-threads=1
```

Final results (`logs/final-validation.log`):

- `cargo fmt --check`: passed
- target `cargo check`: passed
- target release build: passed
- target tests: **8 passed, 0 failed, 0 ignored**
- test executable: native 64-bit AArch64 ELF and executed directly
- dynamic dependencies: only `libgcc_s`, `libc`, loader, and VDSO; **no system libffi**
- source self-containment scan: passed
- `nm`: all required call, closure, preparation, scalar type, long-double type, and complex type symbols found

The tests exercise:

- scalar integer extension and double calls
- seven small integer arguments
- thirteen floating arguments, forcing register overflow to the stack
- a mixed by-value structure
- an AArch64 homogeneous floating aggregate
- libffi version, 32-byte `ffi_cif`, long-double metadata, and all complex metadata
- closure allocation, closure preparation, executable trampoline callback, user data, and closure free

## Limitations

- The translated call path reserves 1 MiB of callee stack headroom per active `ffi_call`; callees requiring more are unsupported.
- C2Rust's translated Linux closure allocator and deprecated raw/Java APIs remain highly unsafe and are not exhaustively tested. The normal memfd/executable closure allocation and callback path is tested.
- Generated modules duplicate compatible C declarations, producing warnings about clashing extern declaration types. These are translation artifacts; the tested layouts match.
- The historical compilation database and logs contain worker paths as evidence. No build script or Rust/assembly source refers to them, and no system libffi is linked.
