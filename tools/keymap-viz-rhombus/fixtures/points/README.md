# KiCad switch centre fixtures

Small JSON lists of `SW_*` footprint centres extracted from keyboard-labs PCBs:

- `ch32x-36-lhs-switches.json` ← `keyboard-labs/pcb/keyboard-ch32x-36-lhs.kicad_pcb`
- `ch32x-36-rhs-switches.json` ← `keyboard-labs/pcb/keyboard-ch32x-36-rhs.kicad_pcb`

Fields: `ref`, `row`, `col`, `x`, `y` (mm), `rot` (KiCad CCW degrees).

To regenerate (requires the keyboard-labs tree):

```bash
./scripts/extract-kicad-switches.py path/to/keyboard-ch32x-36-lhs.kicad_pcb \
  > fixtures/points/ch32x-36-lhs-switches.json
```

These are input geometry for PointsIR weave (`weave_kicad_3x5_3`), not LegendIR.
