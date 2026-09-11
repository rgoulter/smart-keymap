from __future__ import annotations
from dataclasses import dataclass
from typing import Sequence
from .ir import Key, Layout


@dataclass(frozen=True)
class PlacedKey:
    i: int
    x: float
    y: float
    w: float
    h: float
    key: Key


def place_keys(
    layout: Layout,
    keys: Sequence[Key],
    *,
    u: float = 54.0,
    gap: float = 6.0,
    pad: float = 16.0,
    title_h: float = 0.0,
) -> tuple[list[PlacedKey], float, float]:
    """Pure placement. If layout.y_up (KiCad), SVG y is flipped."""
    placed: list[PlacedKey] = []
    y0 = pad + title_h

    if layout.kind == "ortho":
        assert layout.rows and layout.cols
        for k in keys:
            assert k.r is not None and k.c is not None
            x = pad + k.c * (u + gap)
            y = y0 + k.r * (u + gap)
            placed.append(PlacedKey(k.i, x, y, u, u, k))
        w = pad * 2 + layout.cols * u + (layout.cols - 1) * gap
        h = y0 + pad + layout.rows * u + (layout.rows - 1) * gap
        return placed, w, h

    if layout.kind == "points":
        xs = [k.x for k in keys if k.x is not None]
        ys = [k.y for k in keys if k.y is not None]
        min_x, max_x = min(xs), max(xs)
        min_y, max_y = min(ys), max(ys)
        scale = u / layout.unit
        for k in keys:
            assert k.x is not None and k.y is not None
            x = pad + (k.x - min_x) * scale
            if layout.y_up:
                y = y0 + (max_y - k.y) * scale
            else:
                y = y0 + (k.y - min_y) * scale
            ww = (k.w or 1.0) * u
            hh = (k.h or 1.0) * u
            placed.append(PlacedKey(k.i, x, y, ww * 0.92, hh * 0.92, k))
        w = pad * 2 + (max_x - min_x) * scale + u
        h = y0 + pad + (max_y - min_y) * scale + u
        return placed, w, h

    if layout.kind == "ansi_rows":
        x = pad
        max_x = pad
        prev_r = 0
        row_y = y0
        for k in keys:
            r = k.r or 0
            if r != prev_r:
                row_y = y0 + r * (u + gap)
                x = pad
                prev_r = r
            ww = k.w * u
            placed.append(PlacedKey(k.i, x, row_y, ww - gap, u - gap, k))
            x += ww
            max_x = max(max_x, x)
        rows = max((k.r or 0) for k in keys) + 1
        w = max_x + pad
        h = y0 + pad + rows * u + (rows - 1) * gap
        return placed, w, h

    raise ValueError(f"unknown layout.kind={layout.kind}")
