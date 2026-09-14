# Frozen JSON IR schemas (v0)

Units: Point centres in **mm**; KiCad CCW `rot` on PointsIR; SVG projector stores
`-rot` and uses y-down. Scale ≈ 2.8 px/mm, pad 16, pane gap 24.

**Boundaries:** Rhombus builds panes, scenes, and SVG from `points/v0` +
`legend/v0` (or `legend-bundle/v0`). Nickel produces LegendIR only. PointsIR
factories live in Rhombus (`grid`, `split_3x5_3`, `ansi_rows`, KiCad weave).

---

## `points/v0`

```jsonc
{
  "schema": "points/v0",
  "unit": 19.05,
  "points": [
    { "i": 0, "x": 9.525, "y": 9.525, "w": 19.05, "h": 19.05, "rot": 0.0 }
  ]
}
```

- `x`, `y`: key **centres** in mm
- `rot`: KiCad CCW degrees; scene IR stores `-rot` for SVG
- Ortho: centre `((c+0.5)*u, (r+0.5)*u)`
- KiCad CH32X-36 fixtures: `y_up=false` (larger y toward thumbs / bottom)

---

## `legend/v0`

Same length/order as `points`.

```jsonc
{
  "schema": "legend/v0",
  "cells": [
    {
      "legends": { "center": "Tab", "hold": "H(L8)", "tl": "", "tr": "", "bl": "" },
      "emphasis": "normal",  // normal|pressed|muted|unused|transparent
      "wash": 8              // optional: layer_mod.hold dest → LAYER_PALETTE[N]
    }
  ]
}
```

Empty strings omit Text nodes. Bundles (`legend-bundle/v0`) also carry
`layer_legends[]` and a `dense` cell list (corners from layers 1..3).

XXXX / NO → `pressed` (solid KeySide brick; no KeyTop). Transparent: low
fill-opacity, full stroke.

---

## `scene/v0`

Canvas in SVG pixels. Kinds: `Title`, `KeySide`, `KeyTop`, `Text` (roles
legend/hold/overlay).

Draw order: Title, then per key KeySide → KeyTop → Text(s). Pressed omits KeyTop
and legends. Rotated nodes wrap in `<g transform="rotate(rot cx cy)">`.
Multi-pane `stack` offsets each pane by prior height + `PANE_GAP`.

Per-layer wash (`theme["layers"]`, stacked): pastel side/top by `Pane.layer_index`.
Cell optional `wash` overrides pane index. pressed/muted/unused stay global.

`scene_of_panes` / `svg_of_scene` take `~theme` (`theme_light` default, or `theme_dark`).

Light: bg `#f4f1ec`; pressed brick KeySide `#5a5a5a`. Dark: bg `#1a1a1a`;
pressed KeySide `#5c5c5c` + stroke `#aaaaaa`.
