from __future__ import annotations
import json
from pathlib import Path
from .ir import Key, Layer, Layout, Chord, KeymapIR, VizConfig


def from_48key_json(path: Path) -> KeymapIR:
    raw = json.loads(path.read_text())
    layers = []
    holds = {
        13: "Alt", 14: "GUI", 15: "Ctl", 16: "Sft",
        19: "Sft", 20: "Ctl", 21: "GUI", 22: "Alt",
    }
    for li, L in enumerate(raw["layers"]):
        keys = []
        for k in L["keys"]:
            keys.append(Key(
                i=k["i"], r=k["r"], c=k["c"], label=k["label"],
                hold=holds.get(k["i"]) if li == 0 else None,
            ))
        layers.append(Layer(name=L["name"], keys=tuple(keys)))
    return KeymapIR(
        source=raw.get("source", str(path)),
        layout=Layout(kind="ortho", rows=4, cols=12),
        layers=tuple(layers),
        chords=(Chord("home-esc", (15, 16), "Esc"),),
    )


def ch32x36_ish() -> KeymapIR:
    """LHS from real KiCad SW_* footprints (keyboard-ch32x-36-lhs.kicad_pcb).

    On this board KiCad Y increases toward the thumbs — y_up=False.
    Thumbs SW_4_3..SW_4_5 carry rotation (-10 / -20 deg).
    """
    sw_path = Path(__file__).resolve().parent / "fixtures" / "ch32x-36-lhs-switches.json"
    switches = json.loads(sw_path.read_text())
    labels = {
        (1, 1): "'", (1, 2): ",", (1, 3): ".", (1, 4): "P", (1, 5): "Y",
        (2, 1): "A", (2, 2): "O", (2, 3): "E", (2, 4): "U", (2, 5): "I",
        (3, 1): ";", (3, 2): "Q", (3, 3): "J", (3, 4): "K", (3, 5): "X",
        (4, 3): "Esc", (4, 4): "Spc", (4, 5): "Tab",
    }
    holds = {(2, 1): "Alt", (2, 2): "GUI", (2, 3): "Ctl", (2, 4): "Sft"}
    keys: list[Key] = []
    for i, sw in enumerate(switches):
        r, c = sw["row"], sw["col"]
        keys.append(Key(
            i=i, x=sw["x"], y=sw["y"],
            label=labels.get((r, c), sw["ref"]),
            hold=holds.get((r, c)),
            legends={"rot": str(sw["rot"])},
        ))
    return KeymapIR(
        source="pcb/keyboard-ch32x-36-lhs.kicad_pcb SW_*",
        layout=Layout(kind="points", y_up=False, unit=19.05),
        layers=(Layer("Base", tuple(keys)),),
    )


def ansi_60() -> KeymapIR:
    rows = [
        [("Esc", 1), ("1", 1), ("2", 1), ("3", 1), ("4", 1), ("5", 1), ("6", 1), ("7", 1), ("8", 1), ("9", 1), ("0", 1), ("-", 1), ("=", 1), ("Bksp", 2)],
        [("Tab", 1.5), ("Q", 1), ("W", 1), ("E", 1), ("R", 1), ("T", 1), ("Y", 1), ("U", 1), ("I", 1), ("O", 1), ("P", 1), ("[", 1), ("]", 1), ("\\", 1.5)],
        [("Caps", 1.75), ("A", 1), ("S", 1), ("D", 1), ("F", 1), ("G", 1), ("H", 1), ("J", 1), ("K", 1), ("L", 1), (";", 1), ("'", 1), ("Enter", 2.25)],
        [("Shift", 2.25), ("Z", 1), ("X", 1), ("C", 1), ("V", 1), ("B", 1), ("N", 1), ("M", 1), (",", 1), (".", 1), ("/", 1), ("Shift", 2.75)],
        [("Ctrl", 1.25), ("GUI", 1.25), ("Alt", 1.25), ("Space", 6.25), ("Alt", 1.25), ("GUI", 1.25), ("Menu", 1.25), ("Ctrl", 1.25)],
    ]
    keys: list[Key] = []
    i = 0
    for r, row in enumerate(rows):
        for lab, w in row:
            keys.append(Key(i=i, r=r, c=0, w=w, label=lab))
            i += 1
    return KeymapIR(
        source="demo-60ansi",
        layout=Layout(kind="ansi_rows", rows=5),
        layers=(Layer("Base", tuple(keys)),),
    )



def ch32x36_full(*, split_gap_mm: float = 28.0) -> KeymapIR:
    """Full 36-key: LHS + RHS from real KiCad SW footprints, side by side.

    Each half keeps its own board coordinates; RHS is placed to the right of LHS
    with split_gap_mm between bounding boxes. Rotations kept as KiCad values
    (renderer negates for SVG).
    """
    fixtures = Path(__file__).resolve().parent / "fixtures"
    lhs = json.loads((fixtures / "ch32x-36-lhs-switches.json").read_text())
    rhs = json.loads((fixtures / "ch32x-36-rhs-switches.json").read_text())

    lhs_labels = {
        (1, 1): "'", (1, 2): ",", (1, 3): ".", (1, 4): "P", (1, 5): "Y",
        (2, 1): "A", (2, 2): "O", (2, 3): "E", (2, 4): "U", (2, 5): "I",
        (3, 1): ";", (3, 2): "Q", (3, 3): "J", (3, 4): "K", (3, 5): "X",
        (4, 3): "Esc", (4, 4): "Spc", (4, 5): "Ent",
    }
    # Mirrored alphas for RHS demo (same physical stagger, opposite hand)
    rhs_labels = {
        (1, 1): "F", (1, 2): "G", (1, 3): "C", (1, 4): "R", (1, 5): "L",
        (2, 1): "D", (2, 2): "H", (2, 3): "T", (2, 4): "N", (2, 5): "S",
        (3, 1): "B", (3, 2): "M", (3, 3): "W", (3, 4): "V", (3, 5): "Z",
        (4, 3): "Bksp", (4, 4): "Spc", (4, 5): "Tab",
    }
    holds_l = {(2, 1): "Alt", (2, 2): "GUI", (2, 3): "Ctl", (2, 4): "Sft"}
    holds_r = {(2, 5): "Alt", (2, 4): "GUI", (2, 3): "Ctl", (2, 2): "Sft"}

    lhs_max_x = max(s["x"] for s in lhs)
    rhs_min_x = min(s["x"] for s in rhs)
    # Place RHS so its leftmost point sits split_gap to the right of LHS rightmost
    x_shift = (lhs_max_x + split_gap_mm) - rhs_min_x

    keys: list[Key] = []
    i = 0
    for sw in lhs:
        r, c = sw["row"], sw["col"]
        keys.append(Key(
            i=i, x=sw["x"], y=sw["y"],
            label=lhs_labels.get((r, c), sw["ref"]),
            hold=holds_l.get((r, c)),
            legends={"rot": str(sw["rot"]), "half": "L"},
        ))
        i += 1
    for sw in rhs:
        r, c = sw["row"], sw["col"]
        keys.append(Key(
            i=i, x=sw["x"] + x_shift, y=sw["y"],
            label=rhs_labels.get((r, c), sw["ref"]),
            hold=holds_r.get((r, c)),
            legends={"rot": str(sw["rot"]), "half": "R"},
        ))
        i += 1

    return KeymapIR(
        source="pcb/keyboard-ch32x-36-{lhs,rhs}.kicad_pcb SW_*",
        layout=Layout(kind="points", y_up=False, unit=19.05),
        layers=(Layer("Base", tuple(keys)),),
    )


def default_viz(**overrides) -> VizConfig:
    base = dict(
        layer_colors={"Base": "#3b6ea5", "Raise": "#2f9e44", "Lower": "#e67700", "Adjust": "#9c36b5"},
        key_style="keycap_tone",
    )
    base.update(overrides)
    return VizConfig(**base)
