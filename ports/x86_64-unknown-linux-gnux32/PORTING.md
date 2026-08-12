# x86_64-unknown-linux-gnux32 C2Rust port

## Provenance

- Source: `https://github.com/libffi-rs/libffi-rs.git`, branch `rrir`
- Exact source HEAD: `893bd63954735468305c83ddd56718e3863fdacd`
- Bundled libffi: 3.5.2
- C2Rust: 0.22.1
- Generation worker: assigned AWS Ubuntu x86_64 instance `i-07af490f370c3b7d0`
- Target: x32 ILP32 ABI on the x86-64 instruction set
- Translation compiler: Ubuntu Clang 18.1.3
- Validation compiler: stable Rust 1.97.1

The recent `rrir` commits through the exact HEAD, `libffi/build.rs`, and
`libffi-sys-rs/build/{build,common,not_msvc,msvc}.rs` were inspected before
translation. No existing target port was used as the translation input.

## Configuration and C2Rust

The worker was provisioned with x32 multilib headers/libraries. A clean copy of
the bundled upstream source was configured as a cross configuration because
the running kernel cannot execute x32 programs:

```sh
CC=clang CFLAGS='-O0 -g -mx32' ./configure \
  --build=x86_64-pc-linux-gnu \
  --host=x86_64-unknown-linux-gnux32 \
  --with-pic --disable-shared --disable-docs \
  --prefix="$HOME/work/configured/install"
bear --output compile_commands.all.json -- make -j2 V=1
make install
```

Configure selected `TARGET=X86_64`, identified `long double` as 16 bytes, and
built a genuine x32 static library. Bear captured these configured C units:

- `src/prep_cif.c`
- `src/types.c`
- `src/raw_api.c`
- `src/java_raw_api.c`
- `src/tramp.c`
- `src/closures.c`
- `src/x86/ffi64.c`

It also selected `src/x86/unix64.S`. The seven-entry C database was passed to
C2Rust with:

```sh
c2rust transpile --emit-modules --overwrite-existing compile_commands.json
```

C2Rust emitted all seven Rust modules and reported its expected inability to
emit the three complex-valued descriptor statics. `compile_commands.json` is a
path-normalized version of the captured commands, retaining the actual `-mx32`
and configured include semantics. `generation/` vendors the selected C inputs,
their include dependencies, and configured headers; `scripts/transpile.sh`
proves the database can be replayed without the original checkout. See
`logs/upstream-*.log`, `logs/c2rust.log`, and
`logs/c2rust-self-contained.log`.

## Target adaptations

1. `size_t`, pointers, and `long` remain 32-bit under the Rust x32 target, while
   `ffi_arg`, `ffi_sarg`, and the backend's register slots remain 64-bit.
2. GNU x32's x87 80-bit `long double` is represented by opaque 16-byte,
   16-byte-aligned storage, not by `f64` or IEEE binary128.
3. The omitted complex descriptors were recreated with x32 sizes/alignments:
   float 8/4, double 16/8, and long double 32/16.
4. `ffi_cif` is the configured six-field, 24-byte x32 layout and
   `FFI_DEFAULT_ABI = FFI_UNIX64` (2). No target-inapplicable CIF field exists.
5. C2Rust's unstable glibc extern types were replaced with stable opaque
   structs, and obsolete atomic intrinsics were replaced with stable
   `AtomicI32`/`AtomicUsize` operations using equivalent orderings.
6. The configured `unix64.S` is retained and compiled by target-asserting
   `build.rs` with `-mx32`, `-fcf-protection=none`, and vendored configured
   headers. The latter keeps assembly trampoline sizing consistent with the
   configured Clang C translation (`__CET__` absent).
7. C2Rust lowers C `alloca` to a heap `Vec`, but `ffi_call_unix64` intentionally
   steals that allocation as the called function's machine stack. The retained
   x86-64 assembly is entered through `rust_call_shim.S`, which zero-extends the
   x32 byte count, copies the argument block to real stack storage, calls the
   upstream entry, and restores its frame. There is no translated Rust inline
   assembly/template in this configured backend.

The maintained Rust sources use no unstable language/library feature. Stable
Rust distributes this target in 1.97.1; `rust-toolchain.toml` requests it.

## Validation and execution limitation

The following stable commands passed after a clean build (translated-code
warnings remain):

```sh
cargo fmt --all -- --check
cargo check --target x86_64-unknown-linux-gnux32
cargo build --target x86_64-unknown-linux-gnux32
cargo test --target x86_64-unknown-linux-gnux32 --no-run
```

`tests/abi_layout.c` passed Clang `-mx32` compile-time ABI assertions.
`tests/e2e.rs` builds an x32 test executable with five tests covering descriptor
and CIF layout, integer register/stack `ffi_call`, SSE `ffi_call`, structures by
value, and an executable closure allocation/preparation/call/free round trip.
The tests were **not executed**: the assigned AWS kernel explicitly has
`CONFIG_X86_X32_ABI` unset, so native launch returns `Exec format error` (126),
and Ubuntu qemu-user 8.2.2 rejects x32 ELF with `Invalid ELF image for this
architecture` (255). Exact evidence is in `logs/execution-attempt.log`; no
execution success is claimed.

Compile/link validation is complete:

- Rust outputs and both assembly objects are ELF32 x86-64 (x32).
- `nm` finds defined CIF, call, closure, complex/long-double, upstream assembly,
  and Rust-call-shim symbols in the archive.
- The linked e2e executable itself defines the required call/closure/type
  symbols.
- Its only `DT_NEEDED` libraries are `libgcc_s.so.1` and `libc.so.6`; there is no
  system `libffi` dependency.
- Source/build configuration contains no absolute generation path and Cargo
  has no libffi dependency or link directive.

See `logs/stable-final-validation.log`, `logs/final-test-build.log`,
`logs/symbols-and-self-containment.log`, and `logs/abi-static-assert.log`.
Runtime semantics remain unverified solely because this worker provides no x32
execution mechanism.
