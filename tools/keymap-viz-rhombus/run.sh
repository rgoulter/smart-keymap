#!/usr/bin/env bash
set -euo pipefail
export PATH="/workspace/racket-install/racket/bin:${PATH:-}"
cd "$(dirname "$0")"

run_tests() {
  local f
  for f in tests/test_core_*.rhm; do
    echo "== $f =="
    racket "$f"
  done
  echo "ALL TESTS PASSED"
}

layout_file() {
  case "$1" in
    48key-basic|48) echo layouts/48key_basic/layout.rhm ;;
    36key-rgoulter|36) echo layouts/36key_rgoulter/layout.rhm ;;
    36key-kicad|36kicad) echo layouts/36key_kicad/layout.rhm ;;
    ch32x-60-improved|60|66) echo layouts/ch32x_60_improved/layout.rhm ;;
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

case "${1:-all}" in
  test) run_tests ;;
  viz)
    LAYOUT="${2:?usage: $0 viz <layout>}"
    export_for "$LAYOUT"
    racket "$(layout_file "$LAYOUT")"
    ;;
  artifacts|all-layouts)
    for L in 48key-basic 36key-rgoulter 36key-kicad ch32x-60-improved; do
      export_for "$L"
      racket "$(layout_file "$L")"
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
      racket "$(layout_file "$L")"
    done
    ;;
  *)
    echo "usage: $0 [test|viz <layout>|artifacts|png|all]" >&2
    echo "layouts: 48key-basic | 36key-rgoulter | 36key-kicad | ch32x-60-improved" >&2
    exit 1
    ;;
esac
