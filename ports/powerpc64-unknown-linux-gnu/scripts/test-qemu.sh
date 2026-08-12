#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
target=powerpc64-unknown-linux-gnu
archive="target/$target/debug/liblibffi_c2rust_powerpc64.a"

cargo fmt --all -- --check
cargo check --target "$target"
cargo build --target "$target"
cargo test --target "$target" -- --nocapture --test-threads=1

testbin=$(find "target/$target/debug/deps" -maxdepth 1 -type f -name 'e2e-*' -perm -111 -print -quit)
test -n "$testbin"

symbols=$(powerpc64-linux-gnu-nm -g --defined-only "$archive")
for symbol in \
    ffi_call ffi_call_LINUX64 ffi_call_go \
    ffi_prep_cif ffi_prep_cif_var ffi_prep_closure_loc \
    ffi_closure_alloc ffi_closure_free ffi_closure_LINUX64 \
    ffi_type_void ffi_type_sint64 ffi_type_double ffi_type_longdouble \
    ffi_type_complex_float ffi_type_complex_double ffi_type_complex_longdouble \
    trampoline_code_table
do
    grep -q " $symbol$" <<<"$symbols"
done

dynamic=$(powerpc64-linux-gnu-readelf -d "$testbin")
if grep -qi libffi <<<"$dynamic"; then
    echo "error: test binary dynamically links system libffi" >&2
    exit 1
fi
executable_symbols=$(powerpc64-linux-gnu-nm -g "$testbin")
grep -q ' ffi_call$' <<<"$executable_symbols"
qemu-ppc64 -L /usr/powerpc64-linux-gnu "$testbin" \
    --exact executable_closure_trampoline_calls_rust_callback --nocapture
