#!/bin/sh
set -eu

if [ -n "${MIRI_LIBFFI_PATH:-}" ]; then
    libffi=$MIRI_LIBFFI_PATH
elif [ "$(uname -s)" = "Darwin" ]; then
    libffi="$(brew --prefix libffi)/lib/libffi.dylib"
elif command -v ldconfig >/dev/null 2>&1; then
    libffi=$(ldconfig -p | awk '$1 ~ /^libffi\.so(\.|$)/ { print $NF; exit }')
else
    echo "Could not locate a shared libffi library; set MIRI_LIBFFI_PATH." >&2
    exit 1
fi

if [ -z "$libffi" ] || [ ! -f "$libffi" ]; then
    echo "Could not locate a shared libffi library; set MIRI_LIBFFI_PATH." >&2
    exit 1
fi

libffi=$(realpath "$libffi")
export MIRIFLAGS="${MIRIFLAGS:+$MIRIFLAGS }-Zmiri-native-lib=$libffi"

exec cargo +nightly miri test --workspace --features system --lib
