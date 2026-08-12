#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd -P)
tmp_db=porting/compile_commands.absolute.json
trap 'rm -f "$tmp_db"' EXIT
python3 - "$root" "$tmp_db" <<'PY'
import json, os, sys
root, output = sys.argv[1:]
db = json.load(open("compile_commands.json"))
for entry in db:
    entry["directory"] = root
    entry["file"] = os.path.join(root, entry["file"])
    args = []
    for arg in entry["arguments"]:
        if arg.startswith("porting/"):
            arg = os.path.join(root, arg)
        elif arg.startswith("-Iporting/") or arg.startswith("-Iasm/"):
            arg = "-I" + os.path.join(root, arg[2:])
        args.append(arg)
    entry["arguments"] = args
json.dump(db, open(output, "w"), indent=2)
PY
mkdir -p generated porting/objects
c2rust transpile "$tmp_db" --output-dir generated --emit-modules \
    --overwrite-existing --translate-const-macros conservative
