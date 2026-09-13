#!/usr/bin/env bash
# Imperative shell: eval Nickel demos and write SVG files.
#
# Usage: ncl/scripts/keymap-viz.sh [outdir] [demo]
#   demo: 48key-basic | 36key-split | 60ansi | all

set -euo pipefail

root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$root"

outdir="${1:-.}"
demo="${2:-all}"
mkdir -p "${outdir}"

write_demo() {
  local name="$1"
  local ncl="ncl/viz/demos/${name}.ncl"
  python3 - "${ncl}" "${outdir}" <<'PY'
import json, subprocess, sys
from pathlib import Path

ncl, outdir = sys.argv[1], Path(sys.argv[2])
raw = subprocess.check_output(
    [
        "nickel",
        "export",
        "--format",
        "json",
        "--import-path=ncl",
        "--field",
        "files",
        ncl,
    ],
    text=True,
)
for item in json.loads(raw):
    path = outdir / item["name"]
    path.write_text(item["svg"])
    print(path)
PY
}

case "${demo}" in
  all)
    write_demo 48key-basic
    write_demo 36key-split
    write_demo 60ansi
    ;;
  48key-basic | 36key-split | 60ansi)
    write_demo "${demo}"
    ;;
  *)
    echo "unknown demo: ${demo}" >&2
    echo "expected: 48key-basic | 36key-split | 60ansi | all" >&2
    exit 1
    ;;
esac
