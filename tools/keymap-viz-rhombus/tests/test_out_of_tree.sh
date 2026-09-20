#!/usr/bin/env bash
# Out-of-tree regression proof for viz layouts.
# Copies layouts/hello to a temp dir, renders it via `viz-path` (scratch
# assembly: engine plus the layout's own directory), and requires
# byte-identical SVG output to the in-tree run.
# Invoked by run.sh test; runs directly too.
set -euo pipefail
cd "$(dirname "$0")/.."

tmp="$(mktemp -d "${TMPDIR:-/tmp}/keymap-viz-test-XXXXXX")"
trap 'rm -rf "$tmp"' EXIT

rm -f out/hello-stacked.svg out/hello-stacked-dark.svg
./run.sh viz hello > /dev/null
cp -r layouts/hello "$tmp/hello"
scratch="$(./run.sh viz-path "$tmp/hello/layout.rhm" | sed -n 's/^scratch: //p' | tail -n 1)"
[ -n "$scratch" ] || { echo "viz-path printed no scratch dir" >&2; exit 1; }
diff out/hello-stacked.svg "$scratch/out/hello-stacked.svg"
diff out/hello-stacked-dark.svg "$scratch/out/hello-stacked-dark.svg"
rm -rf "$scratch"
echo "OK test_out_of_tree"
