#!/usr/bin/env bash
set -euo pipefail
if [ -d /workspace/racket-install/racket/bin ]; then
  export PATH="/workspace/racket-install/racket/bin:${PATH:-}"
fi
RACKET="${RACKET_BIN:-${RACKET:-racket}}"
cd "$(dirname "$0")"

run_tests() {
  local f
  for f in tests/test_core_*.rhm; do
    echo "== $f =="
    "$RACKET" "$f"
  done
  ./tests/test_out_of_tree.sh
  echo "ALL TESTS PASSED"
}

layout_file() {
  case "$1" in
    48key-basic|48) echo layouts/48key_basic/layout.rhm ;;
    36key-rgoulter|36) echo layouts/36key_rgoulter/layout.rhm ;;
    36key-kicad|36kicad) echo layouts/36key_kicad/layout.rhm ;;
    ch32x-60-improved|60|66) echo layouts/ch32x_60_improved/layout.rhm ;;
    hello) echo layouts/hello/layout.rhm ;;
    hello-dense) echo layouts/hello/dense.rhm ;;
    *) echo "unknown layout: $1" >&2; return 1 ;;
  esac
}

export_for() {
  case "$1" in
    48key-basic|48) ./ncl/export-legend.sh 48 ;;
    36key-rgoulter|36|36key-kicad|36kicad) ./ncl/export-legend.sh 36 ;;
    ch32x-60-improved|60|66) ./ncl/export-legend.sh 66 ;;
    *) echo "unknown layout for export: $1" >&2; return 1 ;;
  esac
}

# Run a layout file living anywhere (e.g. a downstream repo): assemble a
# scratch tree with the engine (src, fixtures) plus the layout's own
# directory, then run it there. Relative ../../src imports keep working;
# outputs land in the scratch out/ dir, whose path is printed for the
# caller to collect (caller owns scratch cleanup).
# With an exporter, LegendIR is exported first: nickel resolves the keymap
# via --import-path, so the exporter may also live anywhere. The bundle
# path must match the one hardcoded in the layout runner.
viz_path() {
  local layout exporter bundle dir name scratch
  layout="$(readlink -f "${1:?usage: $0 viz-path <layout.rhm> [exporter.ncl] [bundle.json]}")"
  exporter="${2:-}"
  bundle="${3:-}"
  dir="$(dirname "$layout")"
  name="$(basename "$dir")"
  scratch="$(mktemp -d "${TMPDIR:-/tmp}/keymap-viz-XXXXXX")"
  mkdir -p "$scratch/layouts/$name"
  cp -r src fixtures "$scratch/"
  cp -r "$dir"/. "$scratch/layouts/$name/"
  if [ -n "$exporter" ]; then
    if [ -z "$bundle" ]; then
      bundle="$scratch/out/.cache/$name-bundle.json"
    fi
    ./ncl/export-legend.sh "$exporter" "$bundle"
  fi
  ( cd "$scratch" && "$RACKET" "layouts/$name/$(basename "$layout")" )
  echo "scratch: $scratch"
}

case "${1:-all}" in
  test) run_tests ;;
  viz)
    LAYOUT="${2:?usage: $0 viz <layout>}"
    if [ "$LAYOUT" != "hello" ] && [ "$LAYOUT" != "hello-dense" ]; then
      export_for "$LAYOUT"
    fi
    "$RACKET" "$(layout_file "$LAYOUT")"
    ;;
  viz-path)
    viz_path "${2:?usage: $0 viz-path <layout.rhm> [exporter.ncl] [bundle.json]}" "${3:-}" "${4:-}"
    ;;
  artifacts|all-layouts)
    for L in 48key-basic 36key-rgoulter 36key-kicad ch32x-60-improved; do
      export_for "$L"
      "$RACKET" "$(layout_file "$L")"
    done
    ;;
  png)
    VENV="${VENV:-/workspace/rhombus-kle-spike/.venv}"
    shopt -s nullglob
    for svg in out/*.svg; do
      png="${svg%.svg}.png"
      "$VENV/bin/python" -c "import cairosvg; cairosvg.svg2png(url='$svg', write_to='$png')"
      echo "wrote $png"
    done
    ;;
  all)
    run_tests
    for L in 48key-basic 36key-rgoulter 36key-kicad ch32x-60-improved; do
      export_for "$L"
      "$RACKET" "$(layout_file "$L")"
    done
    ;;
  *)
    echo "usage: $0 [test|viz <layout>|viz-path <layout.rhm> [exporter.ncl] [bundle.json]|artifacts|png|all]" >&2
    echo "layouts: hello | hello-dense | 48key-basic | 36key-rgoulter | 36key-kicad | ch32x-60-improved" >&2
    exit 1
    ;;
esac
