#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
mkdir -p .c2rust-db
trap "rm -rf .c2rust-db" EXIT HUP INT TERM
jq --arg root "$PWD" 'map(.directory = $root | .file = ($root + "/" + .file))' \
  compile_commands.json > .c2rust-db/compile_commands.json
c2rust transpile --emit-modules --overwrite-existing .c2rust-db/compile_commands.json
