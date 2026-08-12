#!/bin/sh
set -eu
target=powerpc64le-unknown-linux-musl
cargo fmt --all -- --check
cargo check --target "$target"
cargo build --release --target "$target"
cargo test --target "$target"
archive="target/$target/release/liblibffi_powerpc64le_musl_c2rust.a"
powerpc64le-linux-musl-nm -g --defined-only "$archive" > logs/nm-release.log
for symbol in ffi_call ffi_prep_cif ffi_prep_closure_loc ffi_closure_LINUX64 ffi_closure_alloc ffi_type_complex_longdouble; do
  grep -Eq "[[:space:]]$symbol$" logs/nm-release.log || { echo "missing $symbol" >&2; exit 1; }
done
