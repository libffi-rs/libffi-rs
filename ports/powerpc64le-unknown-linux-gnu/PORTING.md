# powerpc64le-unknown-linux-gnu C2Rust port

Standalone translation of the vendored libffi at libffi-rs `rrir` commit
`893bd63954735468305c83ddd56718e3863fdacd`. Package name:
`libffi-c2rust-powerpc64le-linux-gnu`.

## Source selection and translation

The assigned EC2 instance was already `shutting-down` when first contacted and
never accepted SSH. AWS subsequently reported it `terminated`. To salvage the
port without creating any AWS resource, the work was performed in an isolated
Ubuntu 24.04 arm64 Docker container on the controlling machine.

Installed tools included `gcc-powerpc64le-linux-gnu`, `qemu-user`, Clang 18,
Bear, and C2Rust 0.22.1. Commands (paths shown are historical generation paths,
not build-time dependencies):

```sh
git clone --branch rrir https://github.com/libffi-rs/libffi-rs.git repo
git -C repo checkout 893bd63954735468305c83ddd56718e3863fdacd
cp -a repo/libffi-sys-rs/libffi configured
cd configured
CC=powerpc64le-linux-gnu-gcc CFLAGS='-O0 -g -fPIC' \
  ./configure --host=powerpc64le-unknown-linux-gnu \
  --with-pic --disable-shared --disable-docs
bear --output compile_commands.all.json -- make -j2 V=1
```

The configured makefile selected these C translation units:

- `src/prep_cif.c`, `src/types.c`, `src/raw_api.c`
- `src/java_raw_api.c`, `src/closures.c`, `src/tramp.c`
- `src/powerpc/ffi.c`, `src/powerpc/ffi_sysv.c`,
  `src/powerpc/ffi_linux64.c`

The three PowerPC files were given distinct temporary basenames before C2Rust
to avoid all three colliding as `ffi.rs`. The actual transpilation was:

```sh
c2rust transpile --emit-modules --disable-rustfmt \
  compile_commands.port.json -- \
  -D__powerpc64__=1 -D__powerpc__=1 -D__PPC64__=1 \
  -D__LITTLE_ENDIAN__=1 -D_CALL_ELF=2
```

C2Rust 0.22.1's Clang AST exporter crashes on Clang's PowerPC
`__vector_quad`/`__vector_pair` builtin AST types when passed a PowerPC target.
The target macros above select the configured PPC64LE libffi paths while the
transpiler runs with the LP64 little-endian host data model. All generated Rust
in this crate came from that target-selected run; `ffi_sysv.rs` is empty as
expected because it is the inactive 32-bit implementation.

## Manual fixes

- `_CALL_ELF=2` is passed explicitly so C2Rust translates the Linux ELFv2
  trampoline and ABI paths (the transpiler itself is not running a PPC frontend).
- C2Rust cannot emit C complex static initializers. `src/types.rs` defines the
  three complex descriptors and immutable element arrays explicitly.
- PowerPC64LE GNU `long double` is a 16-byte, 16-aligned ABI object for this
  configuration (`SIZEOF_LONG_DOUBLE=16`, GCC reports 106 mantissa bits). Its
  descriptor uses those ABI properties, not `c_double` and not a Rust numeric
  representation.
- Preserved `linux64.S` and `linux64_closure.S`; `build.rs` target-asserts
  powerpc64/little-endian/Linux and compiles both with `cc`.
- C2Rust emitted removed atomic intrinsics and unstable extern opaque types in
  `closures.rs`; these were replaced with stable atomic operations and opaque
  zero-sized declarations. Integer constant/pointer casts were also corrected.
- The PowerPC `ffi_cif` has `nfixedargs`; it does not use macOS AArch64's
  `aarch64_nfixedargs` field.

## Reproduction and tests

On Ubuntu with the cross compiler and qemu installed:

```sh
rustup target add powerpc64le-unknown-linux-gnu
./scripts/test-qemu.sh
powerpc64le-linux-gnu-nm -g --defined-only \
  target/powerpc64le-unknown-linux-gnu/debug/liblibffi_c2rust_powerpc64le.a
```

The qemu suite executes scalar, floating-point, stack-spill, signed-integer,
struct-by-value, and executable closure callback paths. See `logs/` for
configure, transpilation, formatting, build, test, and symbol inspection logs. The crate does not link to system libffi and has no dependency
on the source checkout or generation paths. Its only non-Rust build inputs are
the vendored configured headers and two target assembly files.
