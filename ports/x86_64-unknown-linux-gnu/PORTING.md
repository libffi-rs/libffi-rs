# x86_64-unknown-linux-gnu C2Rust port

## Provenance

- Source repository: `https://github.com/libffi-rs/libffi-rs.git`
- Branch: `rrir`
- Exact source commit: `893bd63954735468305c83ddd56718e3863fdacd`
- Bundled libffi version: 3.5.2
- Target: `x86_64-unknown-linux-gnu`
- Generation host: assigned AWS Ubuntu 24.04 x86_64 instance `i-0aebea07f1ed10f26`
- C2Rust: 0.22.1
- Translation compiler: Ubuntu Clang 18.1.3
- Validation Rust: stable 1.97.1

Commits `23efc98` through `893bd63`, the existing `libffi/` translation, `libffi/build.rs`, and `libffi-sys-rs/build/{common,not_msvc,msvc}.rs` were inspected before generation. In particular, this standalone crate follows the recent target-specific build-script design: `build.rs` rejects every target except this one and uses `cc` to compile the retained configured architecture assembly and headers.

## Target configuration and translation

A clean copy of `libffi-sys-rs/libffi` from the exact checkout was configured and built natively on the assigned worker:

```sh
./configure \
  --with-pic --disable-shared --disable-docs \
  --prefix="$HOME/work/configured/install" \
  CC=clang CFLAGS="-O0 -g"
bear --output ../compile_commands.all.json -- make -j2 V=1
make install
```

Configure identified build, host, and target as `x86_64-pc-linux-gnu`; detected 8-byte `double`, 16-byte `long double`, little endian, closures, and `TARGET=X86_64`. The configured static library selected these C inputs:

- `src/prep_cif.c`
- `src/types.c`
- `src/raw_api.c`
- `src/java_raw_api.c`
- `src/closures.c`
- `src/tramp.c`
- `src/x86/ffi64.c`
- `src/x86/ffiw64.c`

It also selected `src/x86/unix64.S` and `src/x86/win64.S`. Those files cannot be translated by C2Rust and are retained in `asm/x86/`.

Bear captured the real configured compiler invocations. Assembly entries were removed to form the eight-entry C input database, and C2Rust was actually run on the worker:

```sh
c2rust transpile --emit-modules --overwrite-existing compile_commands.json
```

`compile_commands.json` is a path-normalized record of those commands (the captured options and relative paths are unchanged apart from `/usr/bin/clang` becoming `clang`). `logs/upstream-configure.log`, `logs/upstream-make.log`, and `logs/c2rust.log` contain the target configuration, selected build, and translation output. C2Rust emitted Rust for all eight inputs and reported its known inability to translate the three complex-valued descriptor statics.

## Target adaptations

1. Recreated `ffi_type_complex_float`, `ffi_type_complex_double`, and `ffi_type_complex_longdouble`. Their SysV sizes/alignments are 8/4, 16/8, and 32/16 respectively and are tested.
2. Represented GNU/Linux x86-64's x87 80-bit `long double` storage as a 16-byte, 16-byte-aligned opaque Rust type. It is not incorrectly represented as `c_double` or IEEE binary128.
3. Kept the x86-64 six-field `ffi_cif` layout and `FFI_DEFAULT_ABI = FFI_UNIX64` (value 2). There is no AArch64/macOS-only CIF field.
4. Replaced C2Rust's unstable glibc opaque extern types with zero-sized opaque structures; only pointers to these private libc types are used.
5. Replaced removed compiler atomic intrinsics in the translated closure allocator with stable `AtomicI32` and `AtomicUsize` operations preserving relaxed/acquire/release order.
6. Added the target assembly `rust_call_shim.S`. C `alloca` in `ffi_call` becomes a heap `Vec` in C2Rust output, but `ffi_call_unix64` deliberately treats that block as the called function's machine stack. The shim makes a dynamically sized real-stack copy, invokes the retained upstream assembly, and restores its frame.
7. Forced `-fcf-protection=none` only for retained assembly. The configured Clang C translation did not define `__CET__`, so `UNIX64_TRAMP_SIZE` is 32. Ubuntu's GCC defaults to `__CET__=3`; without this flag the `cc` build would silently emit 40-byte ENDBR trampolines against translated 32-byte indexing. A closure test exposed that mismatch. The flag makes assembly preprocessing match the exact configured C input and works with either GCC or Clang.
8. Added stable-Rust pointer casts required by the generated code. There is no translated Rust inline assembly in this backend, so no Rust asm-template brace escaping was needed.

The crate has no absolute build paths, no system-libffi link directive, and no dependency on the original checkout. Runtime assembly includes only the four configured headers and x86 private headers vendored here.

## Validation on the assigned target

`logs/final-validation.log` records a clean native validation after `cargo clean`:

```sh
cargo fmt --all -- --check
cargo check --target x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu
cargo test --target x86_64-unknown-linux-gnu -- --nocapture --test-threads=1
```

Results:

- format check: passed;
- target check: passed (translated-code warnings only);
- target build: passed;
- tests: 9/9 passed, then the exact test executable passed 9/9 again when run directly;
- `ffi_call` tests cover integer and floating-point values, register exhaustion/stack arguments, and structures by value;
- executable closure allocation/preparation/invocation/free round trip passed;
- long-double and all omitted complex descriptor layouts passed;
- `nm` found defined call, CIF, closure, assembly, and type symbols in the produced archive;
- the final ELF test executable itself defines `ffi_call`, `ffi_prep_cif`, `ffi_prep_closure_loc`, `ffi_closure_alloc`, and `ffi_type_longdouble`;
- `readelf -d` showed only `libgcc_s.so.1`, `libc.so.6`, and the GNU loader as `DT_NEEDED`; there is no system `libffi` dependency.

## Limitations

The result remains mechanically translated unsafe code and retains C2Rust warnings, including equivalent per-module C declarations diagnosed as clashing Rust extern declarations. Rust has no stable x87 `long double` scalar; callers must use compatible 16-byte storage. UNIX64 call and closure paths are executed; configured Win64/EFI64 support is retained but not executed under the native Linux ABI. Unwind propagation and deprecated Java raw APIs are not covered by this crate's tests.
