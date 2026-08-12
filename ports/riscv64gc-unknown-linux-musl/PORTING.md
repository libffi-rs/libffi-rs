# RISC-V 64 musl C2Rust port

This crate is a source port of bundled libffi 3.5.2 from
`libffi-rs` branch `rrir`, exact commit
`893bd63954735468305c83ddd56718e3863fdacd`. It only supports
`riscv64gc-unknown-linux-musl`; `build.rs` rejects every other target.
It does not probe or link a host/system libffi.

## Reproduction summary

The work was performed on the assigned Ubuntu x86_64 worker. A
`riscv64-linux-musl` GCC 11.2.1 toolchain from musl.cc and qemu-riscv64 8.2.2
were used. The bundled upstream tree was copied to a fresh build directory and
actually configured with:

```text
CC=riscv64-linux-musl-gcc AR=riscv64-linux-musl-ar \
RANLIB=riscv64-linux-musl-ranlib CFLAGS='-O0 -g -fPIC' \
./configure --host=riscv64-unknown-linux-musl \
  --build=x86_64-pc-linux-gnu --with-pic --disable-shared --disable-docs
```

Bear captured the verbose configured build. The resulting database contains
exactly the seven selected C units plus `src/riscv/sysv.S`. The C-only
`c2rust/compile_commands.json` adds explicit clang
`--target=riscv64-linux-musl` and the cross sysroot, and selects:

- `src/prep_cif.c`
- `src/types.c`
- `src/raw_api.c`
- `src/java_raw_api.c`
- `src/closures.c`
- `src/tramp.c`
- `src/riscv/ffi.c`

C2Rust 0.22.1 was built against LLVM/Clang 14 and run over that configured
database. The first LLVM 18 attempt failed on newer RISC-V vector builtin AST
kinds and is retained in `logs/c2rust-transpile.log`; the successful LLVM 14
run is `logs/c2rust-transpile-llvm14.log`. Unmodified generated output is in
`c2rust/raw-output/`.

## Stable-Rust fixes and target details

- Removed generated nightly feature gates. Opaque `FILE` declarations use
  stable zero-sized opaque C structs, and generated atomic intrinsics use
  stable `core::sync::atomic` operations.
- RISC-V musl config reports long double size/alignment 16/16. It is represented
  by aligned 16-byte opaque storage; the translated ABI code only classifies
  and copies it, so no unavailable Rust `f128` arithmetic is needed.
- This configured RISC-V target does **not** define
  `FFI_TARGET_HAS_COMPLEX_TYPE`; consequently configured `types.c` does not
  instantiate `ffi_type_complex_*`. Their absence is intentional, matching the
  selected upstream objects and configured header.
- C's empty constrained floating-register asm templates were data moves into
  and out of the `call_context`; they are stable Rust assignments here. Actual
  ABI register dispatch remains in vendored `asm/riscv/sysv.S`.
- C2Rust could not translate `__builtin___clear_cache`. The completed
  `ffi_prep_closure_loc` writes the exact upstream trampoline words and calls
  musl's `__riscv_flush_icache` implementation.
- `ffi_cif` retains the configured RISC-V fields `riscv_nfixedargs` and
  `riscv_unused`; `FFI_DEFAULT_ABI` is `FFI_SYSV == 1`.
- C2Rust models `alloca` with heap vectors. The assembly call bridge was
  minimally adapted to copy stacked arguments onto the real machine stack and
  restore the Rust caller's stack pointer. The ten-argument test exercises
  this path. Closure entry/return still uses the upstream RISC-V assembly.
- `asm/include/` contains the configured `ffi.h`, `ffitarget.h`, and
  `fficonfig.h`, plus `ffi_cfi.h`; builds have no generated-header dependency.

## Build and test

Install the Rust target and put a musl RISC-V cross C compiler named
`riscv64-linux-musl-gcc` on `PATH` (needed by `cc` for `sysv.S`):

```sh
rustup target add riscv64gc-unknown-linux-musl
cargo fmt --check
cargo check --target riscv64gc-unknown-linux-musl
cargo build --release --target riscv64gc-unknown-linux-musl
cargo test --target riscv64gc-unknown-linux-musl --test e2e -- --nocapture
```

The checked-in target config uses rust-lld, static musl, and qemu-riscv64.
Tests cover a basic `ffi_call`, ten integer arguments (including stack
arguments), configured long-double metadata, and an allocated executable
closure invoked as a function.

See `logs/validation-summary.log`, `logs/nm-required-symbols.log`, and
`logs/no-system-libffi.log` for final evidence. Earlier failed attempts are
kept rather than hidden.
