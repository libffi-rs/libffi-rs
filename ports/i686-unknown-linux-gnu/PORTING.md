# i686 GNU/Linux C2Rust port

## Status

This is a self-contained, native-tested C2Rust port of libffi 3.5.2 for
`i686-unknown-linux-gnu`. It was made from the `libffi-rs` `rrir` branch at
exact commit `893bd63954735468305c83ddd56718e3863fdacd` on the assigned live
AWS Ubuntu worker `i-09b8b4fc1e77b8faf` (`18.234.152.83`). The instance was
used as instructed and no instance lifecycle or AWS resource operation was
performed.

The release test suite passes all 10 tests as native 32-bit ELF processes on
the x86_64 host. This artifact does not link to a system libffi.

## Source review

The recent source commits reviewed before porting were:

```text
893bd63 minor fixes
1f3108f test
8e43bdf start on cleanup
25ac199 fix compile
23efc98 initial c2rust
```

Review covered `libffi/`, its target-specific `build.rs`, the expanded tests,
and `libffi-sys-rs/build/{build.rs,common.rs,not_msvc.rs}`. Accordingly this
standalone crate asserts the exact Cargo target and uses `cc` to compile only
the retained target assembly with the configured vendored headers.

## Worker setup and configuration

Ubuntu's i386 architecture and multilib packages were installed, including
`gcc-multilib`, `g++-multilib`, `libc6-dev-i386`, `libc6:i386`, Clang/LLVM 18,
and libclang 18. Every configured C compilation includes `-m32`.

The bundled libffi source was configured out of tree with:

```sh
CC=gcc CFLAGS='-m32 -O0 -g -fPIC' \
  ../libffi-sys-rs/libffi/configure \
  --host=i686-pc-linux-gnu --build=x86_64-pc-linux-gnu \
  --with-pic --disable-shared --disable-docs
bear -- make -j2 V=1
```

Configure selected `TARGET=X86`, `src/x86/ffi.c`, and `src/x86/sysv.S`. The
seven configured C translation units were:

```text
src/prep_cif.c
src/types.c
src/raw_api.c
src/java_raw_api.c
src/closures.c
src/tramp.c
src/x86/ffi.c
```

`porting/compile_commands.absolute.json` is the Bear-generated, absolute
compilation database used for those C files. It contains `-m32`,
`HAVE_CONFIG_H`, and the configured/source include paths. Assembly is excluded
from the C2Rust input by design.

## C2Rust 0.22.1

C2Rust was installed and run on the worker with LLVM/libclang 18:

```sh
export LIBCLANG_PATH=/usr/lib/llvm-18/lib
export LLVM_CONFIG_PATH=/usr/bin/llvm-config-18
export LLVM_LIB_DIR=/usr/lib/llvm-18/lib
c2rust transpile compile_commands.c-only.json \
  --emit-modules --emit-no-std --output-dir ../c2rust-i686 \
  --overwrite-existing
```

The first run translated all six common files, reported the known unsupported
C complex initializers, then hit a C2Rust type-AST panic on x86 fastcall
function types in `src/x86/ffi.c`; see `logs/c2rust.log`. For translation only,
`porting/ffi-c2rust-input.c` removes the `FFI_DECLARE_FASTCALL` macro expansion
without changing function bodies. The one-entry accurate database is
`porting/ffi-fastcall-workaround.json`; the successful architecture rerun is
`logs/c2rust-ffi.log`. The generated `ffi_closure_inner` was then explicitly
restored to Rust's stable `extern "fastcall"` ABI because retained assembly
calls it with arguments in ECX/EDX.

Thus the architecture Rust is an actual C2Rust 0.22.1 translation of the
configured i686 C, not a copy of the macOS or x86-64 port.

## Retained assembly and stack adapter

C2Rust cannot translate `src/x86/sysv.S`, so it is preserved as
`asm/x86/sysv.S`. Its configured dependencies are the only headers vendored
under `asm/include` and `asm/x86`.

C2Rust lowers C `alloca()` to a heap `Vec`, but upstream `ffi_call_i386`
installs that allocation as ESP. `asm/x86/rust_call_shim.S` fixes this safely:
it dynamically allocates the argument block plus `call_frame` on the real
thread stack, copies the translated block, calls the upstream fastcall entry,
and restores the shim frame independently of the ESP value left by upstream
assembly. The 128 KiB callee-stack regression test demonstrates that calls use
the native stack rather than a fixed heap reserve.

No C inline asm was translated to Rust, so no Rust asm braces needed escaping.

## Target ABI corrections

- `FFI_DEFAULT_ABI` is i686 SYSV (`1`), and `ffi_cif` has the configured
  six-field 24-byte layout. It has no macOS `aarch64_nfixedargs` field.
- `gcc -m32` and configure report 4-byte pointers/long, a 12-byte,
  4-aligned x87 `long double`, and a 24-byte, 4-aligned complex long double.
  `ffi_type_longdouble` and all three complex descriptors use those values.
  Exact compiler evidence is in `logs/target-abi.log`.
- C2Rust omits complex-valued static initializers; the three public complex
  descriptors and their element arrays were restored manually.
- C2Rust's obsolete atomic intrinsics in the closure allocator were replaced
  with stable `AtomicI32`/`AtomicUsize` operations. Experimental extern types
  were replaced with opaque zero-sized declarations.
- Static-trampoline alternate closure entries are selected by ABI instead of
  comparing Rust function pointers, which is not reliable under optimization.

## Build and validation

On an x86_64 GNU/Linux host with multilib/i386 runtime packages:

```sh
rustup target add i686-unknown-linux-gnu
cargo fmt --check
cargo check --target i686-unknown-linux-gnu
cargo build --release --target i686-unknown-linux-gnu
cargo test --release --target i686-unknown-linux-gnu -- --test-threads=1
```

The release tests cover:

- closure allocation, executable trampoline, and callback round trip;
- repeated `f64` calls/returns and thirteen `f32` stack arguments;
- signed small-integer extension and seven `u8` arguments;
- mixed and three-float structures passed/returned by value;
- a callee using 128 KiB of native stack;
- exact target layouts, long-double/complex descriptors, and version 3.5.2.

`logs/nm-required-symbols.log` records call, closure, preparation, assembly,
and all primitive/complex type symbols. `logs/no-system-libffi.log` shows the
release test is an i386 ELF whose only dynamic dependencies are libc, libgcc,
and the i386 loader—there is no `libffi` dependency. This also prevents tests
from silently succeeding against Ubuntu's system libffi.

The generated translation remains intentionally close to C2Rust output and
therefore emits non-fatal generated-code warnings. All required check, build,
and native runtime validations pass on stable Rust.
