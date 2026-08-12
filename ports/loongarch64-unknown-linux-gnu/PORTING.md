# LoongArch64 GNU C2Rust port

## Provenance

- Source repository: `https://github.com/libffi-rs/libffi-rs.git`
- Branch/revision: `rrir` at exactly `893bd63954735468305c83ddd56718e3863fdacd`
- Bundled upstream libffi: 3.5.2 (`FFI_VERSION_NUMBER=30502`)
- Translator: C2Rust 0.22.1 (`cargo install --locked c2rust --version 0.22.1`)
- Target: `loongarch64-unknown-linux-gnu`, LP64D
- Host used for the port: Ubuntu 24.04 x86_64

Before translating, commits `23efc98` through `893bd63` were reviewed, including the
existing AArch64/macOS C2Rust cleanup, `libffi/build.rs`, and
`libffi-sys-rs/build/{build.rs,common.rs,not_msvc.rs}`. No existing target port was
copied.

Ubuntu did not provide a LoongArch GNU cross GCC. The configured build used the
Loongson project release asset:

`https://github.com/loongson/build-tools/releases/download/2025.08.08/x86_64-cross-tools-loongarch64-binutils_2.45-gcc_15.1.0-glibc_2.42.tar.xz`

SHA-256 is recorded in `logs/toolchain.sha256`. It supplies GCC 15.1.0, binutils
2.45, glibc 2.42, and an LP64D sysroot. The toolchain is only a reproduction and
test prerequisite; this crate contains no path or link dependency on it.

## Configure, compilation database, and translation

The exact bundled libffi tree was copied to a clean build tree and configured:

```sh
export CC=loongarch64-unknown-linux-gnu-gcc
export CXX=loongarch64-unknown-linux-gnu-g++
export AR=loongarch64-unknown-linux-gnu-ar
export RANLIB=loongarch64-unknown-linux-gnu-ranlib
export STRIP=loongarch64-unknown-linux-gnu-strip
./configure \
  --host=loongarch64-unknown-linux-gnu \
  --build=x86_64-pc-linux-gnu \
  --prefix="$PREFIX" --with-pic --disable-shared --disable-docs \
  CFLAGS="-O2 -g"
bear --output compile_commands.json -- make -j2 V=1
```

Configure selected `TARGET = LOONGARCH64`, 64-bit `size_t`, 16-byte/16-aligned
`long double`, little endian, LP64D as ABI 3, and these units:

- `src/{prep_cif,types,raw_api,java_raw_api,closures,tramp}.c`
- `src/loongarch/ffi.c`
- `src/loongarch/sysv.S`

The complete Bear database and C-only database are in `logs/`. C2Rust was run on
all seven configured C units (not the assembly):

```sh
c2rust transpile --emit-modules --overwrite-existing \
  --output-dir "$OUT" --filter ".*[.]c$" compile_commands.c.json -- \
  --target=loongarch64-unknown-linux-gnu --sysroot="$SYSROOT"
```

See `logs/configure.log`, `logs/build-c.log`, and
`logs/c2rust-transpile.log`.

## Stable-Rust adaptations

The generated code was kept module-for-module. The following mechanical fixes
were needed on stable Rust:

- Replaced unstable opaque extern types with pointer-compatible `c_void` aliases.
- Replaced removed C2Rust atomic intrinsics with `AtomicI32`/`AtomicUsize` using
  the same relaxed/acquire/release orderings.
- Represented GNU LoongArch binary128 `long double` as a 16-byte, 16-aligned
  opaque type; the translated code only needs layout and moves values as bytes.
- Added correctly laid-out complex descriptor statics. Upstream LoongArch 3.5.2
  does not define `FFI_TARGET_HAS_COMPLEX_TYPE`, so `ffi_prep_cif` still rejects
  complex arguments as upstream does; the descriptors satisfy Unix
  `libffi-sys` consumers without pretending the ABI supports complex calls.
- Implemented C2Rust's sole unimplemented builtin,
  `__builtin___clear_cache`, with the exact four trampoline instruction words
  from upstream and LoongArch `ibar 0`, matching GCC output.
- C2Rust lowers `alloca` to heap `Vec`s. `ffi_call_asm` deliberately switches SP
  into its alloca frame, so calling it on the heap crashes. The small
  `asm/loongarch/stack_bridge.c` copies the translated frame to a genuine C
  `alloca`, invokes the **unmodified upstream** `sysv.S`, and copies return
  registers back. This is the only non-upstream C code.

`build.rs` asserts the exact Cargo target and compiles only the vendored assembly
and stack bridge against the configured vendored headers. There is no system
libffi probe or fallback. Cargo dependencies are vendored under `vendor/`; builds
can run offline.

## Validation

All final commands succeeded; logs are retained:

- `cargo fmt --check`
- offline `cargo check --target loongarch64-unknown-linux-gnu`
- release target build and target `nm` checks
- QEMU execution using `qemu-loongarch64 -L <toolchain-sysroot>`
- two `ffi_call` tests (register-only and ten arguments including stack slots)
- executable closure allocation, preparation, trampoline execution, callback,
  and free
- configured `ffi_cif`, closure, long-double, and complex-descriptor layouts

`logs/nm-compare.log` shows no upstream `ffi*` symbol absent from the port. The
only additions are the stack bridge and the three documented complex descriptor
statics. `logs/no-system-libffi.log` records the test executable's dynamic
section: it needs only the loader, libc, and libgcc_s, and has no libffi
dependency.
