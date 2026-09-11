# keymap-viz

Python FCIS keymap→SVG visualiser for reviewing layouts (e.g. Elecrow buyer
de-risk) before committing PCB/firmware work.

## Architecture

- `src/keymap_viz/ir.py` — data (`Key`, `Layer`, `Layout`, `KeymapIR`, …)
- `src/keymap_viz/geometry.py` — pure placement (`y_up` flips KiCad Y for SVG)
- `src/keymap_viz/render.py` — pure SVG string
- `src/keymap_viz/demos.py` — demo IR builders + package fixtures
- `src/keymap_viz/cli.py` — imperative shell (argparse + files)

## Modes

- `single` / `stacked` / `dense` (no chords on canvas) / `chords` (separate layer)
- `key_style`: `keycap_tone` (default) or `solid_dark`

SVG rotate uses **−KiCad angle** (KiCad CCW vs SVG Y-down CW).

CH32X-36 fixtures use `y_up=False` (board KiCad Y); full split places RHS to the
right of LHS with ~28 mm gap.

## Run

From this directory:

```sh
PYTHONPATH=src python -m keymap_viz.cli all -o /tmp/keymap-viz-out
PYTHONPATH=src python -m pytest -q
```

## Out of scope (follow-ups)

Nickel/ncl legend export, hub wiring, committing large PNG/SVG demo dumps or
KiCad `.kicad_pcb` files.
