# keymap-viz-rhombus

Keymap visualiser, structured as a functional core with an imperative shell
(FCIS): pure Rhombus (`src/keyboard_layout.rhm`, `src/svg.rhm`) plus a thin
I/O shell (`src/io.rhm`).

| Stage | Owner | Artifact |
|-------|--------|----------|
| Keymap → LegendIR JSON | Nickel (`layouts/*/export-legend.ncl` + `ncl/`) | `legend-bundle/v0` |
| PointsIR | Rhombus factories / KiCad switch JSON | `points/v0` |
| KeyboardLayout → Scene → SVG | Rhombus (`src/`) | SVG under `out/` (gitignored) |

Rhombus does **not** author legends; Nickel does **not** draw SVG.

## Requirements

- Racket 8+ with `rhombus` + `rhombus-json` (`raco pkg install rhombus rhombus-json`)
  (`RACKET_BIN` overrides the `racket` binary; a `/workspace` install dir is
  prepended to `PATH` only when it exists)
- Nickel 1.14+ (for legend export; `NICKEL_BIN` overrides the binary)
- smart-keymap checkout (for keymap `.ncl` imports via `SMART_KEYMAP_ROOT`)

Root `devenv.nix` already includes `pkgs.racket` and an `enterShell` hint for
`raco pkg install --auto rhombus rhombus-json` (nickel was already present).
No separate patch file is shipped.

## Quick start

```bash
export PATH="/path/to/racket/bin:/path/to/nickel/bin:$PATH"
export SMART_KEYMAP_ROOT=/path/to/smart-keymap   # if not auto-detected

./run.sh test
./run.sh viz 48key-basic          # Nickel export → layouts/48key_basic/layout.rhm → out/
# or: just viz layout=48key-basic
# or: make viz LAYOUT=48key-basic

./run.sh artifacts                # all packaged layouts
```

Layouts: `hello` (hand-written 2×2 stacked, no Nickel), `hello-dense` (same cells, corner legends), `48key-basic`, `36key-rgoulter`, `36key-kicad`, `ch32x-60-improved`.

## Package layout

```
layouts/<name>/
  layout.rhm              Rhombus runner (PointsIR + LegendIR → SVG)
  export-legend.ncl       Nickel LegendIR export for that keymap (optional share)
ncl/
  export-legend-lib.ncl   shared LegendIR helpers
  legends.ncl             shared label / emphasis logic
  export-legend.sh        Nickel → out/.cache/*-bundle.json
src/keyboard_layout.rhm   ADTs, PointsIR factories, pane/scene (pure)
src/svg.rhm               Scene → SVG string (pure)
src/io.rhm                JSON load / write / emit helpers
scripts/extract-kicad-switches.py
fixtures/points/          small KiCad SW_* centre JSON (input geometry)
tests/test_core_*.rhm     focused pure-function tests
tests/snapshots/          tiny LegendIR parse smoke only
SCHEMA.md                 points/v0, legend/v0, scene/v0
```

Each keyboard keeps its runner and legend export together. Shared Nickel stays
under `ncl/`. `36key_kicad` symlinks `export-legend.ncl` to `36key_rgoulter`
(same LegendIR; different PointsIR).

## Your own keymap / points

Keep your layout dirs anywhere (e.g. a downstream repo) and run them with
`viz-path`, which assembles a scratch tree (engine plus your layout dir) so
the relative `../../src` imports keep working:

1. Export LegendIR with Nickel (`bundle_of` / your `export-legend.ncl`), or write `legend/v0` JSON.
   The keymap import resolves via `--import-path`, so the exporter may also live
   anywhere: `./ncl/export-legend.sh /path/to/export-legend.ncl /abs/out/bundle.json`.
2. Copy `layouts/48key_basic/`, change the PointsIR factory and `bundle` path in `layout.rhm`.
3. `./run.sh viz-path /path/to/your_layout/layout.rhm /path/to/export-legend.ncl`
   (omit the exporter for Nickel-free layouts like `hello`).
   Outputs land in the printed scratch `out/` dir; the caller collects and cleans them.

## Tests

```bash
./run.sh test    # runs every tests/test_core_*.rhm
```
