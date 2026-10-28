#!/usr/bin/env python3
"""Extract SW_* footprint centres from a KiCad 6/7/8 .kicad_pcb into JSON.

Usage:
  extract-kicad-switches.py board.kicad_pcb > switches.json

Output: list of {ref, row, col, x, y, rot} sorted by ref. Units mm; rot KiCad CCW°.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path


def parse_sw_ref(ref: str):
    m = re.match(r"SW_(\d+)_(\d+)$", ref)
    if not m:
        return None
    return int(m.group(1)), int(m.group(2))


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    text = Path(sys.argv[1]).read_text(encoding="utf-8", errors="replace")
    # Match (footprint ... (at x y [rot]) ... (property "Reference" "SW_r_c" ...
    # KiCad sexpr is nested; use a pragmatic scan for SW_* refs + nearest (at ...)
    switches = []
    for m in re.finditer(
        r'\(footprint\s+"[^"]*"\s*(.*?)\n\s*\)\s*(?=\(footprint|\Z)',
        text,
        flags=re.S,
    ):
        block = m.group(0)
        ref_m = re.search(r'\(property\s+"Reference"\s+"(SW_\d+_\d+)"', block)
        if not ref_m:
            continue
        ref = ref_m.group(1)
        rc = parse_sw_ref(ref)
        if not rc:
            continue
        at_m = re.search(r"\(at\s+([-\d.]+)\s+([-\d.]+)(?:\s+([-\d.]+))?\)", block)
        if not at_m:
            continue
        x, y = float(at_m.group(1)), float(at_m.group(2))
        rot = float(at_m.group(3) or 0.0)
        row, col = rc
        switches.append(
            {"ref": ref, "row": row, "col": col, "x": x, "y": y, "rot": rot}
        )
    switches.sort(key=lambda s: (s["row"], s["col"]))
    json.dump(switches, sys.stdout, indent=2)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
