# SPARC64 GNU/Linux C2Rust port

This is a target-specific, self-contained Rust port of bundled libffi 3.5.2 from
`libffi-rs` branch `rrir`, exact commit
`893bd63954735468305c83ddd56718e3863fdacd`. It was produced on the assigned
Ubuntu x86_64 worker with C2Rust 0.22.1; it was not copied from another port.
The five recent source commits and tool versions are in
`logs/toolchain-and-source.log`.

## Configuration and translation

The bundled source was configured out of tree with:

```sh
CC=sparc64-linux-gnu-gcc CFLAGS="-O2 -g -fPIC" \
  libffi/configure --host=sparc64-unknown-linux-gnu \
  --build=x86_64-pc-linux-gnu --disable-shared --disable-docs --with-pic
```

`make V=1` selected eight C units:

- `src/{prep_cif,types,raw_api,java_raw_api,closures,tramp}.c`
- `src/sparc/{ffi,ffi64}.c`

and `src/sparc/{v8,v9}.S`. Under the configured SPARC64 preprocessor state,
`ffi.c` and `v8.S` are empty; `ffi64.c` and `v9.S` are active. The complete
Bear-observed database (all ten units) is `artifacts/compile_commands.json`; its
eight-C-unit filter is `artifacts/compile_commands.c2rust.json`. C2Rust was run
on that filter as:

```sh
c2rust transpile --emit-modules --emit-no-std --log-level INFO \
  artifacts/compile_commands.c2rust.json -- \
  --target=sparc64-unknown-linux-gnu
```

The explicit Clang target is essential: without it C2Rust takes the SPARC V8 C
branch. The final observed run is `logs/c2rust-transpile-bear.log`. Configure,
C build, and database logs are also retained. Absolute worker paths in the JSON
and logs are provenance only; no crate build reads them.

## Stable-Rust repairs

C2Rust emitted all active functions but logged three bounded target issues:

1. Clang complex static initializers are unsupported. `src/types.rs` defines the
   three missing symbols using values measured from configured SPARC64 C:
   complex float `(size,align)=(8,4)`, double `(16,8)`, and long double
   `(32,16)`.
2. C2Rust emitted a non-existent `::f128::f128` path for the scalar long-double
   descriptor. Its configured ABI metadata is directly encoded as size/alignment
   `(16,16)`; no Rust arithmetic on long doubles is introduced.
3. C2Rust cannot lower the SPARC GCC template
   `flush %0; flush %0+8`. `asm/sparc/flush.S` implements precisely those two
   flushes and an ordinary V9 return sequence. The original target `v9.S` is
   preserved unchanged and compiled with it.

Additional mechanical compatibility repairs replace C2Rust 0.22.1 removed
atomic intrinsics with stable `AtomicI32`/`AtomicUsize` operations using the same
Acquire/Release/Relaxed orderings, replace three unstable extern opaque types
with zero-sized opaque structs, and use `std` because C2Rust-generated VLA/alloca
lowering uses `Vec`. The translated public C ABI remains `#[repr(C)]` and
`#[no_mangle]`. Duplicate module-local declarations can produce harmless
`clashing_extern_declarations` warnings because C2Rust generates distinct Rust
type names with identical C layouts.

The configured `ffi_cif` is 40 bytes and has SPARC `nfixedargs` at offset 32.
`FFI_DEFAULT_ABI` is V9 (`1`). See `logs/configured-layout-qemu.log` and the
runtime assertion in `tests/e2e.rs`.

## Build and test

Prerequisites are a stable Rust toolchain with this target,
`gcc-sparc64-linux-gnu`, and `qemu-user`. `build.rs` rejects every target except
`sparc64-unknown-linux-gnu`, invokes the SPARC compiler through `cc`, and uses
only the vendored configured headers/assembly.

```sh
cargo fmt --all -- --check
cargo check --target sparc64-unknown-linux-gnu
cargo build --release --target sparc64-unknown-linux-gnu
cargo test --target sparc64-unknown-linux-gnu -- --nocapture
```

`.cargo/config.toml` supplies the cross linker and QEMU runner. The three QEMU
tests validate configured long-double/complex metadata, a mixed integer/FP
`ffi_call`, and an allocated executable closure that calls from generated SPARC
trampoline code back into Rust. All pass; see `logs/cargo-test-qemu.log`.

`logs/nm-required-symbols.log` records required definitions in the produced
static library. `logs/no-system-libffi.log` shows the test executable defines
its own `ffi_*` symbols, has only glibc/loader/libgcc `NEEDED` entries, and has no
system-libffi path or soname string.
