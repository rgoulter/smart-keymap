#!/usr/bin/env bash
# Export legend-bundle/v0 JSON via Nickel.
# Per-layout exporters live next to their Rhombus runners under layouts/<name>/.
# Shared lib: ncl/export-legend-lib.ncl + ncl/legends.ncl
# Usage: export-legend.sh <48|36|66|layout-name> [out.json]
set -euo pipefail

LAYOUT="${1:?layout id: 48|36|66|48key-basic|36key-rgoulter|ch32x-60-improved|…}"
OUT="${2:-}"

PKG="$(cd "$(dirname "$0")/.." && pwd)"
ROOT="${SMART_KEYMAP_ROOT:-}"
if [[ -z "$ROOT" ]]; then
  if [[ -d /workspace/rgoulter-repos/smart-keymap/ncl ]]; then
    ROOT=/workspace/rgoulter-repos/smart-keymap
  elif [[ -f "$(dirname "$0")/../../../ncl/keys.ncl" ]]; then
    # package at tools/keymap-viz-rhombus → repo root
    ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
  else
    echo "set SMART_KEYMAP_ROOT to the smart-keymap repo root" >&2
    exit 1
  fi
fi

case "$LAYOUT" in
  48|48key|48key-basic)
    SCRIPT="$PKG/layouts/48key_basic/export-legend.ncl"
    DEFAULT_OUT=out/.cache/48key-basic-bundle.json
    ;;
  36|36key|36key-rgoulter|36key-kicad|36kicad)
    SCRIPT="$PKG/layouts/36key_rgoulter/export-legend.ncl"
    DEFAULT_OUT=out/.cache/36key-rgoulter-bundle.json
    ;;
  66|66key|66key-ansi-fn|ch32x-60|ch32x-60-improved)
    SCRIPT="$PKG/layouts/ch32x_60_improved/export-legend.ncl"
    DEFAULT_OUT=out/.cache/66key-ansi-fn-bundle.json
    ;;
  *)
    echo "unknown layout: $LAYOUT (want 48|36|66 or a layout dir name)" >&2
    return 1 2>/dev/null || exit 1
    ;;
esac

OUT="${OUT:-$DEFAULT_OUT}"
# Resolve relative OUT against package root
if [[ "$OUT" != /* ]]; then
  OUT="$PKG/$OUT"
fi
mkdir -p "$(dirname "$OUT")"

NICKEL="${NICKEL:-nickel}"
if ! command -v "$NICKEL" >/dev/null 2>&1; then
  if [[ -x /workspace/nickel-install/bin/nickel ]]; then
    NICKEL=/workspace/nickel-install/bin/nickel
  else
    echo "nickel not on PATH" >&2
    exit 1
  fi
fi

echo "exporting $SCRIPT → $OUT (root=$ROOT)"
# Package ncl/ = shared lib; smart-keymap ncl/ = keys + keymap-ncl-to-json;
# ../tests/ncl/… in exporters resolves via parent of --import-path=$ROOT/ncl
( cd "$ROOT" && "$NICKEL" export --format json \
    --import-path="$PKG/ncl" \
    --import-path="$ROOT/ncl" \
    "$SCRIPT" ) > "$OUT"
echo "wrote $OUT ($(wc -c < "$OUT") bytes)"
