from __future__ import annotations
from dataclasses import dataclass, field
from typing import Literal


@dataclass(frozen=True)
class Key:
    i: int
    r: int | None = None
    c: int | None = None
    x: float | None = None
    y: float | None = None
    w: float = 1.0
    h: float = 1.0
    label: str = ""
    hold: str | None = None
    legends: dict[str, str] = field(default_factory=dict)


@dataclass(frozen=True)
class Layer:
    name: str
    keys: tuple[Key, ...]
    color: str | None = None


@dataclass(frozen=True)
class Chord:
    name: str
    indices: tuple[int, ...]
    label: str


@dataclass(frozen=True)
class Layout:
    kind: Literal["ortho", "points", "ansi_rows"]
    rows: int | None = None
    cols: int | None = None
    y_up: bool = False  # KiCad-style: True => flip Y for SVG
    unit: float = 19.05  # mm per 1u for points layouts


@dataclass(frozen=True)
class VizConfig:
    mode: Literal["single", "stacked", "dense", "chords"] = "single"
    layer_colors: dict[str, str] = field(default_factory=dict)
    dense_slots: dict[str, str] = field(default_factory=lambda: {
        "center": "Base", "tl": "Raise", "tr": "Lower", "bl": "Adjust",
    })
    key_style: Literal["solid_dark", "keycap_tone"] = "keycap_tone"
    keycap_base: str = "#e8e4df"
    keycap_legend: str = "#2a2a2a"
    background: str = "#f4f1ec"
    title_color: str = "#222222"
    separate_chords: bool = True


@dataclass(frozen=True)
class KeymapIR:
    source: str
    layout: Layout
    layers: tuple[Layer, ...]
    chords: tuple[Chord, ...] = ()
