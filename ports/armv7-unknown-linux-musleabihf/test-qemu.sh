#!/bin/sh
set -eu
TARGET=armv7-unknown-linux-musleabihf
: "${CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER:=arm-linux-musleabihf-gcc}"
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER
cargo test --target "$TARGET" --no-run
bin=$(find "target/$TARGET/debug/deps" -maxdepth 1 -type f -name 'e2e-*' -perm -111 | head -n 1)
qemu-arm "$bin" --nocapture
arm-linux-musleabihf-readelf -d "$bin" > target/readelf-dynamic.txt 2>&1 || true
if grep -q 'NEEDED.*libffi' target/readelf-dynamic.txt; then
    echo 'ERROR: test binary links system libffi' >&2
    exit 1
fi
