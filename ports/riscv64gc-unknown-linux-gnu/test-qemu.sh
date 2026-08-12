#!/usr/bin/env bash
set -euo pipefail

TARGET=riscv64gc-unknown-linux-gnu
CROSS_CC=${CROSS_CC:-riscv64-linux-gnu-gcc}
QEMU=${QEMU:-qemu-riscv64}
LOADER=$($CROSS_CC -print-file-name=ld-linux-riscv64-lp64d.so.1)
SYSROOT=$(dirname "$(dirname "$(realpath "$LOADER")")")

export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=$CROSS_CC
export CC_riscv64gc_unknown_linux_gnu=$CROSS_CC
export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER="$QEMU -L $SYSROOT"

cargo test --target "$TARGET" -- --nocapture
