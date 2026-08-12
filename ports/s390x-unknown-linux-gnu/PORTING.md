# s390x GNU/Linux C2Rust port

## Status

This is a self-contained, stable-Rust crate for `s390x-unknown-linux-gnu` based
on libffi-rs revision `893bd63954735468305c83ddd56718e3863fdacd` and its
vendored libffi 3.5.2 sources. It does not link system libffi. The uniquely
named Cargo package is `libffi-c2rust-s390x-unknown-linux-gnu`.

The assigned EC2 instance was already `shutting-down` at the first state query
and never accepted SSH. No replacement or other AWS resource was used. The
port was therefore completed in an isolated Ubuntu 24.04 ARM64 Docker
container on the controlling machine, using the requested s390x cross-GCC and
`qemu-s390x`. QEMU execution is explicitly supported by the target task.

## Source review

Before translating, the following were inspected at exact source HEAD:

- commits `893bd63`, `1f3108f`, `8e43bdf`, `25ac199`, and `23efc98`;
- the current `libffi/` AArch64/macOS port, especially its target assertion,
  assembly build, tests, complex descriptors, and long-double fix;
- `libffi/build.rs`;
- `libffi-sys-rs/build/{build.rs,common.rs,not_msvc.rs}`; and
- the configured s390 files `src/s390/{ffi.c,sysv.S,ffitarget.h,internal.h}`.

The controlling source checkout was only read. All generated and edited files
were created under this port directory.

## Configuration and translation

The essential setup and upstream configuration commands were:

```sh
apt-get install gcc-s390x-linux-gnu libc6-dev-s390x-cross \
  binutils-s390x-linux-gnu qemu-user make autoconf
rustup target add s390x-unknown-linux-gnu

mkdir build && cd build
CC=s390x-linux-gnu-gcc CFLAGS='-O2 -g -fPIC' \
  ../libffi/configure \
  --host=s390x-linux-gnu --with-pic --disable-shared --disable-docs
make -j2 V=1
```

Configure selected exactly these translation units:

```text
src/prep_cif.c
src/types.c
src/raw_api.c
src/java_raw_api.c
src/closures.c
src/tramp.c
src/s390/ffi.c
src/s390/sysv.S
```

The compilation database used the configure-generated `fficonfig.h`,
`include/ffi.h`, s390 `ffitarget.h`, the s390x Linux sysroot, and all include
and preprocessor flags shown by `make V=1`. C2Rust was then run as:

```sh
c2rust 0.22.1 transpile --emit-modules --overwrite-existing \
  --output-dir generated compile_commands.json
```

C2Rust 0.22.1's AST exporter panics for Clang's
`SystemZBuiltinVaList` (`va_list type SystemZBuiltinVaList not yet
implemented`). The successful database therefore used Clang's x86_64 Linux
parser ABI as a frontend workaround while forcing `__s390x__`, using only the
configured s390 headers/sysroot, and retaining the configure results for
big-endian s390x. Both ABIs are LP64 with 16-byte `long double`; all differing
alignments were corrected from target GCC probes as described below. C2Rust
still transpiled the actual configured C translation units and s390-specific
`ffi.c`, rather than another architecture's port. See `logs/c2rust.log`.

## Preserved assembly

`build.rs` rejects every target except `(s390x, linux, gnu)` and compiles:

- `asm/s390/sysv.S`: upstream's configured s390x call, closure, and static
  trampoline assembly; and
- `asm/s390/call_int.S`: the exact `ffi_call_int` body emitted by the configured
  s390x GCC 13 build.

The latter is necessary because C2Rust cannot translate
`__builtin_frame_address(0)`. More importantly, this function uses dynamic
stack allocation to lay out the s390 ABI overflow area immediately above the
outgoing stack frame; replacing it with a Rust heap allocation would be ABI
incorrect. The remaining `src/s390/ffi.c` functions are the C2Rust output.
No C source is compiled by this crate.

Only assembly-required configured headers are vendored under `asm/include/`.

## Manual target-specific fixes

- Defined the C complex static descriptors that C2Rust reported as unsupported.
- Represented s390x GNU `long double` by descriptor metadata, not `c_double`:
  16-byte size and 8-byte alignment; complex long double is 32/8.
- Corrected `__int128` descriptor alignment from the parser ABI's 16 to the
  s390x GCC value 8.
- Kept `ffi_cif` at its actual six-field s390 layout. It deliberately has no
  macOS/AArch64 `aarch64_nfixedargs` field.
- Replaced removed C2Rust-generated atomic intrinsics with stable
  `AtomicI32`/`AtomicUsize` operations preserving Relaxed/Acquire/Release
  orderings.
- Replaced unstable generated glibc extern types with opaque stable Rust enums
  and added required constant casts.
- Used ffi-arg-sized closure result storage in the closure end-to-end test, as
  required for narrow integer closure returns.

## Build and test

Commands were run in the Ubuntu container with:

```sh
export CC_s390x_unknown_linux_gnu=s390x-linux-gnu-gcc
export CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_LINKER=s390x-linux-gnu-gcc
cargo fmt --all -- --check
cargo check --target s390x-unknown-linux-gnu
cargo build --target s390x-unknown-linux-gnu
cargo test --target s390x-unknown-linux-gnu -- --nocapture --test-threads=1
s390x-linux-gnu-nm -g --defined-only \
  target/s390x-unknown-linux-gnu/debug/liblibffi_c2rust_s390x.a
```

`.cargo/config.toml` supplies the same linker and the runner
`qemu-s390x -L /usr/s390x-linux-gnu`.

Final QEMU result:

```text
running 5 tests
test closure_trampoline_end_to_end ... ok
test ffi_call_floating_point ... ok
test ffi_call_integer_registers_and_stack ... ok
test ffi_call_struct_by_value ... ok
test target_type_descriptor_layouts ... ok

test result: ok. 5 passed; 0 failed
```

The tests cover GPR and stack-overflow arguments, FPR arguments/returns,
indirect structure arguments/returns and call-by-value copying, static closure
trampolines, and target long-double/complex descriptor sizes. The symbol check
found `ffi_call`, `ffi_call_SYSV`, `ffi_call_int`, closure/go-closure entry
points, closure allocation/preparation, `ffi_prep_cif`, all checked type
symbols, and `trampoline_code_table`. Full evidence is in `logs/`.

## Limitations

- Stable Rust has no C-compatible IEEE binary128 scalar, so long double is
  represented only by ABI-correct `ffi_type` metadata and is passed as opaque
  storage by the translated implementation. It is not incorrectly aliased to
  `c_double`. There is no numerical binary128 Rust callback test.
- Generated modules retain benign warnings about duplicate module-local C ABI
  declarations and unused generated expressions. Builds and tests succeed on
  stable Rust.
- Deprecated raw/Java raw APIs compile and export but are not exercised by the
  end-to-end suite.
- Testing was under QEMU user mode with the Ubuntu cross sysroot, not native
  s390x hardware, because the assigned worker was unavailable.

## AWS worker termination

Initial evidence:

```text
Instance: i-05579cb65309c6559
Public IP: 35.172.116.196
Initial state: shutting-down
SSH: connect to host 35.172.116.196 port 22: Operation timed out
State transition: User initiated (2026-08-12 05:33:20 GMT)
```

After `aws ec2 wait instance-terminated --region us-east-1 --instance-ids
i-05579cb65309c6559`, the final state was verified as `terminated` (state code
48). No other instance was terminated or modified.
