# ARMv7 hard-float GNU/Linux C2Rust port

## Status

This is a **tested partial port** of libffi 3.5.2 from `libffi-rs` branch `rrir`, exact commit `893bd63954735468305c83ddd56718e3863fdacd`, for `armv7-unknown-linux-gnueabihf`.

The normal integer, float, double, stack-argument, structure, and closure paths pass under `qemu-arm`. One homogeneous-float-aggregate return case is retained as an ignored regression test. The translated `ffi_call` also has the allocation limitation described below; do not treat this artifact as production-ready.

The assigned EC2 instance was already in `shutting-down` state before the first SSH connection and never accepted SSH. Work therefore continued in an isolated Ubuntu 24.04 ARM64 Docker container on the controlling machine. No replacement instance or billable resource was created. Instance `i-09a7fa198422400e0` was subsequently verified `terminated`.

## Source review

Before translation, the last commits and the existing macOS/AArch64 port were inspected:

```text
893bd63 minor fixes
1f3108f test
8e43bdf start on cleanup
25ac199 fix compile
23efc98 initial c2rust
```

In particular, review covered `libffi/`, `libffi/build.rs`, `libffi/tests/e2e.rs`, and `libffi-sys-rs/build/{build.rs,common.rs,not_msvc.rs}`. This port uses the recent target assertion plus `cc` assembly compilation pattern, but its Rust was generated from an ARM hard-float configuration rather than copied from AArch64.

## Toolchain and configuration

The translation environment was Ubuntu 24.04 on AArch64 with:

- C2Rust 0.22.1 (installed with `cargo install c2rust --version 0.22.1 --locked`)
- `gcc-arm-linux-gnueabihf` 13.3.0
- QEMU user 8.2.2
- stable Rust 1.97.1
- LLVM/Clang 18

Upstream was configured out of tree:

```sh
mkdir config-armv7 && cd config-armv7
CC=arm-linux-gnueabihf-gcc CFLAGS='-O0 -g -fPIC' \
  ../libffi-sys-rs/libffi/configure \
  --host=arm-linux-gnueabihf \
  --build=aarch64-unknown-linux-gnu \
  --with-pic --disable-shared --disable-docs
make -j2 V=1
```

Configure selected `TARGET=ARM`, `src/arm/ffi.c`, and `src/arm/sysv.S`. The configured C translation units were:

```text
src/prep_cif.c
src/types.c
src/raw_api.c
src/java_raw_api.c
src/closures.c
src/tramp.c
src/arm/ffi.c
```

`porting/compile_commands.absolute.json` is the exact compilation database used. Each command uses Clang's `--target=arm-linux-gnueabihf`, `/usr/arm-linux-gnueabihf` as sysroot, `HAVE_CONFIG_H`, and the configured/source include paths. Translation command:

```sh
export LIBCLANG_PATH=/usr/lib/llvm-18/lib
export LLVM_CONFIG_PATH=/usr/bin/llvm-config-18
export LLVM_LIB_DIR=/usr/lib/llvm-18/lib
c2rust transpile compile_commands.json \
  --emit-modules --emit-no-std \
  --output-dir c2rust-armv7 --overwrite-existing
```

C2Rust intentionally reported unsupported C complex static initializers and the `__clear_cache` builtin; fixes are below. The complete log is `logs/c2rust.log`.

## Assembly and headers

C2Rust cannot translate `src/arm/sysv.S`, so it is preserved at `asm/arm/sysv.S` and compiled by `build.rs` using `cc`. `build.rs` asserts the exact Rust target, not merely host OS/architecture. Required configured headers are vendored under `asm/include`, with ARM's `internal.h` under `asm/arm`.

The assembly supplies `ffi_call_SYSV`, `ffi_call_VFP`, SYSV/VFP closure and go-closure entries, alternate static-trampoline entries, `trampoline_code_table`, and `ffi_arm_trampoline`. See `logs/nm-required-symbols.log`. No C inline-assembly template was translated to Rust, so there were no generated Rust format braces to escape. The test-only `global_asm!` has no literal braces.

## Manual translation fixes

1. **ARM ABI layout:** all generated `ffi_cif` definitions retain ARM's `vfp_used`, `vfp_reg_free`, `vfp_nargs`, and `vfp_args[16]`. There is no macOS-only `aarch64_nfixedargs`. Tests assert the 32-bit ARM `ffi_cif` size is 48 bytes.
2. **Complex statics:** C2Rust 0.22.1 cannot emit `_Complex` static initializers. `src/types.rs` manually defines the three complex descriptors and their element arrays with target sizes/alignment.
3. **Long double:** the configured compiler reports `SIZEOF_LONG_DOUBLE=8`, `__SIZEOF_LONG_DOUBLE__=8`, and binary64 precision (`__LDBL_MANT_DIG__=53`). Consequently the target ABI representation is an 8-byte, 8-aligned binary64 descriptor. This is target evidence, not an AArch64/macOS assumption.
4. **Cache clearing and closures:** `ffi_prep_closure_loc`, omitted by C2Rust because of the unsupported builtin, was translated manually and calls libgcc's `__clear_cache`. Both static and dynamic trampoline paths are retained.
5. **Stable Rust:** removed experimental extern types, replaced obsolete generated atomic intrinsics with stable `AtomicI32`/`AtomicUsize` operations, and corrected target-sized constant casts. The crate checks on stable Rust.
6. **C `alloca` emulation:** C2Rust converted temporary call/closure arrays to heap vectors. Closure argument arrays were retained. For `ffi_call`, ARM assembly temporarily installs the provided area as SP; a byte-aligned vector was incorrect. The fix uses 8-byte-aligned storage with a 64 KiB downward-growing callee stack reserve.

## Build and test

Install the target, cross linker, and QEMU/sysroot, then run:

```sh
rustup target add armv7-unknown-linux-gnueabihf
cargo fmt --check
cargo check --target armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf
cargo test --target armv7-unknown-linux-gnueabihf -- --test-threads=1
```

`.cargo/config.toml` configures `arm-linux-gnueabihf-gcc` and:

```text
qemu-arm -L /usr/arm-linux-gnueabihf
```

Final QEMU evidence (`logs/cargo-test-qemu-final.log`):

- closure trampoline callback: 1 passed
- ffi_call/version suite: 6 passed, 1 ignored
- passing calls cover seven `u8` arguments, signed integer extension, repeated `f64`, thirteen `f32` arguments (register/stack pressure), and a by-value mixed structure
- executable ELF attributes report ARMv7-A, VFPv3, and `Tag_ABI_VFP_args: VFP registers`

## Known limitations

- **Allocation leak:** every translated `ffi_call` currently retains its 64 KiB backing stack (plus the argument/frame area), and wide/small-structure scratch returns may retain another small allocation. Deallocating after the SP-swapping assembly call was not reliable because LLVM did not preserve the heap base across this unusual stack switch. A production fix should implement the C `alloca` behavior in a dedicated assembly/Rust wrapper that keeps the real thread stack.
- **HFA return regression:** the ARM hard-float three-`f32` aggregate test correctly prepares `ARM_TYPE_VFP_N` and VFP argument registers, but receives a zero result through the translated return path. It is kept ignored, not deleted (`tests/e2e.rs`).
- C2Rust's generated modules duplicate compatible C declarations and produce warnings about clashing extern declaration types. They compile and link but should eventually be consolidated.
- Tests use QEMU user emulation, not physical ARMv7 hardware.

Because of the first two limitations, this delivery is clearly labeled partial despite the passing stable build and six end-to-end call tests.
