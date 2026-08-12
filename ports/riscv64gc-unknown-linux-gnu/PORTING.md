# RISC-V 64 GNU/Linux C2Rust libffi port

## Scope

- Rust target: `riscv64gc-unknown-linux-gnu` (`rv64gc`, hard-float `lp64d`)
- Source: `https://github.com/libffi-rs/libffi-rs.git`, branch `rrir`, detached commit `893bd63954735468305c83ddd56718e3863fdacd`
- Vendored libffi version: 3.5.2
- Translator: C2Rust 0.22.1
- Validation compiler/runtime: Ubuntu 24.04 `riscv64-linux-gnu-gcc` 13.3 and QEMU user 8.2
- Rust: stable 1.97.1

The package and Rust library names are target-specific so Cargo tests cannot silently substitute the host's system libffi. The build compiles only `asm/riscv/sysv.S`; all selected C translation units are represented by Rust modules.

## EC2 incident

The assigned instance `i-024cb2467152f5ab7` was already in `shutting-down` state before the first SSH connection completed. It could not be restarted. It was waited to `terminated` and verified. No replacement instance or other AWS resource was created. To salvage the port, the same Ubuntu cross-toolchain workflow was run in a local isolated Ubuntu 24.04 Docker container. Logs retain this fact rather than claiming remote execution.

## Configure and source selection

The source checkout was inspected at the exact commit, including the last six commits, `libffi/`, `libffi/build.rs`, and `libffi-sys-rs/build/*.rs`. The configured upstream build was produced with:

```sh
mkdir -p build && cd build
CC=riscv64-linux-gnu-gcc CFLAGS='-O0 -g -fPIC' \
  "$SOURCE/libffi-sys-rs/libffi/configure" \
  --host=riscv64-unknown-linux-gnu \
  --with-pic --disable-shared --disable-docs
make V=1
```

The resulting build selected exactly these C files:

```text
src/prep_cif.c
src/types.c
src/raw_api.c
src/java_raw_api.c
src/closures.c
src/tramp.c
src/riscv/ffi.c
```

It selected `src/riscv/sysv.S` as the only assembly input. The compile database used the configured build directory and generated headers (`build/fficonfig.h`, `build/include/ffi.h`, and `build/include/ffitarget.h`) with the same source include directories and target ABI.

## C2Rust invocation and parser workaround

The initial accurate target commands used Clang's RISC-V frontend:

```text
clang --target=riscv64-linux-gnu -march=rv64gc \
  --sysroot=/usr/riscv64-linux-gnu -DHAVE_CONFIG_H \
  -Ibuild -I$SOURCE/libffi-sys-rs/libffi \
  -I$SOURCE/libffi-sys-rs/libffi/include -Ibuild/include \
  -I$SOURCE/libffi-sys-rs/libffi/src -O0 -g -fPIC -fexceptions
```

C2Rust 0.22.1's AST converter crashed on Clang 18's built-in RISC-V vector types (`TagTypeUnknown`) even though `rv64gc` does not enable V. The translation was therefore run with the same RISC-V-configured headers and sources through a host Clang parser with these target macros:

```text
-D__riscv=1 -D__riscv_xlen=64 -D__riscv_float_abi_double=1
```

Both parser hosts are LP64 little-endian with 16-byte `long double` and `__int128` alignment. C2Rust was then run as:

```sh
c2rust transpile --emit-modules --fail-on-error compile_commands.json
```

For `src/riscv/ffi.c` only, a temporary transpiler input replaced the four empty floating-register constraint statements with equivalent assignments and replaced unsupported `__builtin___clear_cache` with a declaration/call to glibc's RISC-V `__riscv_flush_icache`. This temporary C input is not a build dependency. The produced Rust is `src/riscv/ffi.rs`.

## Manual Rust/assembly fixes

1. Replaced C2Rust's unsupported `f128` crate representation with `F128`, a 16-byte, 16-aligned storage type. It is used for ABI layout only. `ffi_type_longdouble` remains kind 4, size 16, alignment 16; tests verify this.
2. This configured RISC-V target does not define `FFI_TARGET_HAS_COMPLEX_TYPE`, so upstream does not build complex type statics. No fake `c_double` complex/long-double representation was added.
3. Replaced unstable translated `extern type` declarations with opaque zero-sized C-layout marker structs.
4. Replaced removed C2Rust atomic intrinsics with stable `AtomicI32`/`AtomicUsize` helpers preserving Relaxed, Acquire, and Release orderings.
5. Fixed C2Rust pointer and constant integer casts required by stable Rust.
6. C2Rust lowers C `alloca` to a heap `Vec`, but upstream RISC-V assembly temporarily makes that allocation the callee stack and originally recovers the caller stack by arithmetic. `ffi_call_asm` now saves/restores the real Rust stack pointer in an enlarged context frame. Rust reserves 1 MiB of aligned downward stack headroom in the heap allocation. This is necessary for correct calls from stable Rust without a dynamic-stack-allocation primitive.
7. The closure cache flush uses libc's RISC-V-specific `__riscv_flush_icache`; it does not use a host-only inline `fence.i` substitute.
8. `ffi_cif` has the configured RISC-V fields `riscv_nfixedargs` and `riscv_unused`. It does not retain the macOS-only `aarch64_nfixedargs` field.

## Build and test

Install `gcc-riscv64-linux-gnu`, `qemu-user`, and the Rust target, then run:

```sh
rustup target add riscv64gc-unknown-linux-gnu
export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=riscv64-linux-gnu-gcc
export CC_riscv64gc_unknown_linux_gnu=riscv64-linux-gnu-gcc
cargo fmt --check
cargo check --target riscv64gc-unknown-linux-gnu
cargo build --target riscv64gc-unknown-linux-gnu
./test-qemu.sh
```

`test-qemu.sh` derives QEMU's sysroot from the cross compiler rather than embedding a checkout or temporary path. Tests exercise integer sign extension, hard-float arguments/results, register overflow to the stack, small integers, two structure conventions, libffi version/layout, long double metadata, and an executable closure callback.

See `logs/final-validation.log`, `logs/test.log`, and `logs/nm.log` for captured evidence.

## Limitations

- `ffi_call` provides 1 MiB of callee stack headroom because stable Rust has no direct equivalent of the C dynamic `alloca` contract required by this assembly. Calls needing more stack are unsupported.
- C2Rust-generated Linux closure allocation contains translated dlmalloc and executable-mapping code and remains highly unsafe. The normal closure allocation/callback path is tested under QEMU, but every allocator fallback and deprecated raw/Java API is not exhaustively tested.
- The crate intentionally asserts exactly `riscv64gc-unknown-linux-gnu` in `build.rs`.
- The crate links the target libc as Rust programs normally do, but never links system libffi.
