# Shared LegendIR library (Nickel)

`export-legend-lib.ncl` + `legends.ncl` are shared helpers. Per-keymap exporters
live next to their Rhombus runners under `layouts/<name>/export-legend.ncl`.

```bash
# from this package (SMART_KEYMAP_ROOT set or auto-detected):
./ncl/export-legend.sh 48   # → out/.cache/48key-basic-bundle.json

# equivalent manual invoke (from smart-keymap root):
nickel export --format json \
  --import-path=/path/to/tools/keymap-viz-rhombus/ncl \
  --import-path=ncl \
  /path/to/tools/keymap-viz-rhombus/layouts/48key_basic/export-legend.ncl
```

Exporters import `export-legend-lib.ncl` from this directory and resolve
`../tests/ncl/…` keymaps via the parent of smart-keymap’s `ncl/` import path.
