#!/usr/bin/env bash
set -euo pipefail
TARGET=powerpc64le-unknown-linux-gnu
export CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_GNU_LINKER=powerpc64le-linux-gnu-gcc
export CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_GNU_RUNNER="qemu-ppc64le -L /usr/powerpc64le-linux-gnu"
cargo fmt --check
cargo check --target "$TARGET"
cargo build --target "$TARGET"
cargo test --target "$TARGET"
