# keymap-viz-rhombus

FCIS keymap visualiser.

| Stage | Owner | Artifact |
|-------|--------|----------|
| Keymap → LegendIR JSON | Nickel (`layouts/*/export-legend.ncl` + `ncl/`) | `legend-bundle/v0` |
| PointsIR | Rhombus factories / KiCad switch JSON | `points/v0` |
| KeyboardLayout → Scene → SVG | Rhombus (`src/`) | SVG under `out/` (gitignored) |

Rhombus does **not** author legends; Nickel does **not** draw SVG.

## Requirements

- Racket 8+ with `rhombus` + `rhombus-json` (`raco pkg install rhombus rhombus-json`)
- Nickel 1.14+ (for legend export)
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

Layouts: `48key-basic`, `36key-rgoulter`, `36key-kicad`, `ch32x-60-improved`.

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

1. Export LegendIR with Nickel (`bundle_of` / `layouts/<name>/export-legend.ncl`), or write `legend/v0` JSON.
2. Copy `layouts/48key_basic/`, change the PointsIR factory and `bundle` path in `layout.rhm`.
3. `racket layouts/your_layout/layout.rhm` — no edits to a shared numbered artifact file.

## Tests

```bash
./run.sh test    # runs every tests/test_core_*.rhm
```
