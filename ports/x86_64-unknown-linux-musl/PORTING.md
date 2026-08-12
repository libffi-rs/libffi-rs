# x86_64-unknown-linux-musl C2Rust port

## Provenance

- Source repository: `https://github.com/libffi-rs/libffi-rs.git`
- Branch: `rrir`
- Source commit: `893bd63954735468305c83ddd56718e3863fdacd`
- Bundled libffi version: 3.5.2
- C2Rust: 0.22.1
- Rust: stable (validated with Rust 1.89.0 on x86_64 Alpine/musl)
- Target: `x86_64-unknown-linux-musl`

The assigned EC2 instance was already `shutting-down` at the first AWS query and SSH timed out. AWS reported `User initiated (2026-08-12 05:33:16 GMT)`; the instance was then observed in `terminated`. No replacement instance or other AWS resource was created. Port generation and target execution therefore used a local, emulated x86_64 Linux container instead.

Before translation, commits `23efc98` through `893bd63`, the existing macOS/AArch64 `libffi/` port, `libffi/build.rs`, and `libffi-sys-rs/build/{common,not_msvc,msvc}.rs` were inspected. The source checkout remained unchanged.

## Configuration and source selection

A clean copy of `libffi-sys-rs/libffi` was configured for this target. The reproducible target-side configuration is:

```sh
apt-get update
apt-get install -y musl-tools build-essential linux-libc-dev
mkdir build && cd build
CC=musl-gcc ../upstream/configure \
  --host=x86_64-unknown-linux-musl \
  --disable-shared --disable-docs --with-pic \
  --prefix="$PWD/install"
make -j2 V=1
make install
```

On Ubuntu 24.04, Debian's musl include directory needed links to the installed Linux UAPI headers for `src/tramp.c`:

```sh
ln -s /usr/include/linux /usr/include/x86_64-linux-musl/linux
ln -s /usr/include/asm-generic /usr/include/x86_64-linux-musl/asm-generic
ln -s /usr/include/x86_64-linux-gnu/asm /usr/include/x86_64-linux-musl/asm
```

The successful configuration selected `TARGET=X86_64` and these C translation units:

- `src/prep_cif.c`
- `src/types.c`
- `src/raw_api.c`
- `src/java_raw_api.c`
- `src/closures.c`
- `src/tramp.c`
- `src/x86/ffi64.c`
- `src/x86/ffiw64.c`

It also selected `src/x86/unix64.S` and `src/x86/win64.S`, which C2Rust cannot translate and which are compiled by `build.rs`. `compile_commands.json` records the corresponding configured C commands using relative generation paths.

The actual C2Rust run used Clang target `x86_64-unknown-linux-musl` and Zig 0.16's x86_64 musl headers because the assigned Ubuntu machine was inaccessible. Its generated configuration had the same ABI settings as the later `musl-gcc` configuration: 16-byte `long double`, 16-byte alignment, closures enabled, UNIX64 default ABI, and the same source set. The only generated-header difference was hidden-symbol visibility; the final vendored `fficonfig.h` is from the `musl-gcc` configuration.

```sh
cargo install c2rust --version 0.22.1 --locked --root c2rust-tool
mkdir -p generated/src/x86
c2rust-tool/bin/c2rust transpile compile_commands.json \
  --output-dir generated --emit-modules --overwrite-existing \
  --translate-const-macros conservative
```

C2Rust reported its expected unsupported C complex statics in `types.c`; the rest of all eight selected C files was emitted. See `logs/c2rust.log`.

## Manual fixes

1. Recreated the three C complex `ffi_type` statics. On this ABI complex values occupy two scalar slots, so complex long double is size 32/alignment 16.
2. Represented x86_64 musl's 80-bit extended `long double` storage as a 16-byte, 16-byte-aligned opaque Rust type. It was not incorrectly mapped to `c_double` or IEEE binary128.
3. Replaced C2Rust's unstable opaque extern types with zero-sized opaque structs.
4. Replaced removed compiler atomic intrinsics in the translated dlmalloc fallback with stable `AtomicI32`/`AtomicU64` operations preserving acquire/release order.
5. Added required target-size casts and one raw-pointer cast needed by stable Rust.
6. Added `rust_call_shim.S`. Upstream `ffi_call_unix64` deliberately installs C `alloca` memory as the called function's stack, while C2Rust lowers that allocation to a heap `Vec`. The shim makes a dynamic real-stack copy before entering upstream assembly and restores its frame afterward. This avoids stack-probe failures and does not impose a fixed argument-size cap.
7. Adapted `ffi_cif` to the six-field x86_64 layout and `FFI_DEFAULT_ABI=FFI_UNIX64` (value 2). No macOS-only `aarch64_nfixedargs` field remains.

There was no translated Rust inline assembly for this backend, so no Rust asm-template brace escaping was needed. Upstream UNIX64 and Win64 assembly plus the Rust-call shim are all compiled by `build.rs`.

## Validation

Validation ran in an `x86_64` `rust:1.89-alpine` container, whose host and libc are x86_64 musl:

```sh
apk add --no-cache build-base binutils musl-dev file
cargo fmt --all -- --check
cargo check --target x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl
cargo test --target x86_64-unknown-linux-musl -- --nocapture --test-threads=1
```

`logs/final-validation.log` records:

- successful format, check, and build;
- defined `ffi_call`, `ffi_call_unix64`, shim, closure, prep, and type symbols from the final Rust archive;
- 9/9 end-to-end tests passing, including integer and floating-point register/stack calls, structs by value, x86_64 long-double descriptor layout, and an executable closure round trip;
- direct execution of the test binary a second time;
- an x86-64 static-PIE executable with no `DT_NEEDED` shared-library dependencies.

`logs/musl-tools-configure-build.log` records the independent Ubuntu `musl-tools` configure/build and symbols from upstream's reference archive.

## Limitations

- This remains mechanically translated, unsafe code and retains C2Rust warnings, including equivalent per-module C declarations that Rust diagnoses as clashing extern declarations.
- Rust has no stable C x87 `long double` scalar. The ABI descriptor is correct, but callers need a C-compatible 16-byte storage wrapper rather than a Rust numeric primitive.
- UNIX64 call and closure paths are exercised. Win64/EFI64 assembly is included because the configured x86_64 build selects it, but that non-default ABI was not executed under Linux.
- Unwind/exception propagation and deprecated Java raw APIs were not exercised.
