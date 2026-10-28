# Per-layout runners (co-located)

Each directory is one keyboard/demo:

| Dir | Runner | Legend export |
|-----|--------|---------------|
| `48key_basic/` | `layout.rhm` | `export-legend.ncl` → keymap-48key-basic |
| `36key_rgoulter/` | `layout.rhm` | `export-legend.ncl` → keymap-36key-rgoulter |
| `36key_kicad/` | `layout.rhm` | symlink → `../36key_rgoulter/export-legend.ncl` |
| `ch32x_60_improved/` | `layout.rhm` | `export-legend.ncl` → keymap-66key-ansi-fn |
| `hello/` | `layout.rhm` (stacked) and `dense.rhm` | none: cells are written in `cells.rhm` |

```bash
./ncl/export-legend.sh 48                 # → out/.cache/48key-basic-bundle.json
racket layouts/48key_basic/layout.rhm     # or: just viz layout=48key-basic
```

To visualise your own keymap/points: copy a layout directory, point `bundle` at
your exported LegendIR (or write PointsIR + cells in Rhombus), and run it.
No need to edit a central numbered artifact list.
