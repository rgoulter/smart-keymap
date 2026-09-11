from __future__ import annotations
import html
from .ir import KeymapIR, VizConfig, Layer
from .geometry import place_keys


def _esc(s: object) -> str:
    return html.escape(str(s))


def _key_fills(cfg: VizConfig) -> tuple[str, str, str]:
    if cfg.key_style == "keycap_tone":
        return cfg.keycap_base, "#c2bdb6", cfg.keycap_legend
    return "#2c2c2c", "#555555", "#f2f2f2"


def render_svg(ir: KeymapIR, cfg: VizConfig, *, title: str | None = None) -> str:
    if cfg.mode == "stacked":
        return _stacked(ir, cfg, title=title)
    if cfg.mode == "dense":
        return _dense(ir, cfg, title=title)
    if cfg.mode == "chords":
        return _chords_layer(ir, cfg, title=title)
    return _single(ir, cfg, ir.layers[0], title=title or ir.layers[0].name)


def _single(ir: KeymapIR, cfg: VizConfig, layer: Layer, *, title: str | None) -> str:
    title_h = 28 if title else 0
    placed, w, h = place_keys(ir.layout, layer.keys, title_h=title_h)
    fill, stroke, leg = _key_fills(cfg)
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0f}" height="{h:.0f}" viewBox="0 0 {w:.0f} {h:.0f}">',
        f'<rect width="100%" height="100%" fill="{cfg.background}"/>',
    ]
    if title:
        parts.append(
            f'<text x="16" y="20" fill="{cfg.title_color}" font-family="ui-sans-serif,system-ui,sans-serif" font-size="14">{_esc(title)}</text>'
        )
    for pk in placed:
        rot = -float(pk.key.legends.get("rot", "0") or 0)  # KiCad CCW → SVG Y-down
        cx, cy = pk.x + pk.w / 2, pk.y + pk.h / 2
        g_open = f'<g transform="rotate({rot:.1f} {cx:.1f} {cy:.1f})">' if rot else "<g>"
        parts.append(g_open)
        parts.append(
            f'<rect x="{pk.x:.1f}" y="{pk.y:.1f}" width="{pk.w:.1f}" height="{pk.h:.1f}" rx="6" fill="{fill}" stroke="{stroke}"/>'
        )
        parts.append(
            f'<text x="{cx:.1f}" y="{cy + 4:.1f}" text-anchor="middle" fill="{leg}" font-family="ui-monospace,Menlo,monospace" font-size="12">{_esc(pk.key.label)}</text>'
        )
        if pk.key.hold:
            parts.append(
                f'<text x="{cx:.1f}" y="{pk.y + pk.h - 6:.1f}" text-anchor="middle" fill="#c44" font-family="ui-monospace,Menlo,monospace" font-size="8">{_esc(pk.key.hold)}</text>'
            )
        parts.append("</g>")
    parts.append("</svg>")
    return chr(10).join(parts)


def _stacked(ir: KeymapIR, cfg: VizConfig, *, title: str | None) -> str:
    gap = 24
    header = 36
    sizes = []
    for layer in ir.layers:
        _, w, h = place_keys(ir.layout, layer.keys, title_h=28)
        sizes.append((w, h))
    W = max(w for w, _ in sizes) + 24
    H = header + sum(h for _, h in sizes) + gap * (len(sizes) - 1) + 16
    fill, stroke, leg = _key_fills(cfg)
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W:.0f}" height="{H:.0f}" viewBox="0 0 {W:.0f} {H:.0f}">',
        f'<rect width="100%" height="100%" fill="{cfg.background}"/>',
        f'<text x="16" y="24" fill="{cfg.title_color}" font-family="ui-sans-serif,system-ui,sans-serif" font-size="15">{_esc(title or "Stacked layers")}</text>',
    ]
    y_off = header
    for layer, (lw, lh) in zip(ir.layers, sizes):
        color = cfg.layer_colors.get(layer.name) or layer.color or "#6688aa"
        parts.append(
            f'<rect x="8" y="{y_off}" width="{lw + 8:.0f}" height="{lh:.0f}" rx="10" fill="none" stroke="{color}" stroke-width="2"/>'
        )
        parts.append(
            f'<text x="24" y="{y_off + 20}" fill="{color}" font-family="ui-sans-serif,system-ui,sans-serif" font-size="13" font-weight="600">{_esc(layer.name)}</text>'
        )
        placed, _, _ = place_keys(ir.layout, layer.keys, title_h=28)
        for p in placed:
            parts.append(
                f'<rect x="{p.x + 8:.1f}" y="{p.y + y_off:.1f}" width="{p.w:.1f}" height="{p.h:.1f}" rx="6" fill="{fill}" stroke="{stroke}"/>'
            )
            parts.append(
                f'<text x="{p.x + 8 + p.w/2:.1f}" y="{p.y + y_off + p.h/2 + 4:.1f}" text-anchor="middle" fill="{leg}" font-family="ui-monospace,Menlo,monospace" font-size="11">{_esc(p.key.label)}</text>'
            )
        y_off += lh + gap
    parts.append("</svg>")
    return "\n".join(parts)


def _dense(ir: KeymapIR, cfg: VizConfig, *, title: str | None) -> str:
    """Corners = other layers; bottom = hold. Chords are NOT drawn here."""
    by_name = {L.name: L for L in ir.layers}
    base = by_name[cfg.dense_slots.get("center", ir.layers[0].name)]
    title_h = 40
    placed, w, h = place_keys(ir.layout, base.keys, u=64, gap=8, title_h=title_h)
    fill, stroke, leg = _key_fills(cfg)
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0f}" height="{h:.0f}" viewBox="0 0 {w:.0f} {h:.0f}">',
        f'<rect width="100%" height="100%" fill="{cfg.background}"/>',
        f'<text x="16" y="22" fill="{cfg.title_color}" font-family="ui-sans-serif,system-ui,sans-serif" font-size="14">{_esc(title or "Dense (chords on separate layer)")}</text>',
    ]
    corners = {
        "tl": (6, 12, "start"),
        "tr": (-6, 12, "end"),
        "bl": (6, -8, "start"),
    }
    for p in placed:
        parts.append(
            f'<rect x="{p.x:.1f}" y="{p.y:.1f}" width="{p.w:.1f}" height="{p.h:.1f}" rx="6" fill="{fill}" stroke="{stroke}"/>'
        )
        parts.append(
            f'<text x="{p.x + p.w/2:.1f}" y="{p.y + p.h/2 + 4:.1f}" text-anchor="middle" fill="{leg}" font-family="ui-monospace,Menlo,monospace" font-size="13" font-weight="600">{_esc(p.key.label)}</text>'
        )
        for slot, (dx, dy, anchor) in corners.items():
            layer_name = cfg.dense_slots.get(slot)
            if not layer_name or layer_name not in by_name:
                continue
            other = by_name[layer_name].keys[p.i]
            if other.label in ("", "▽"):
                continue
            color = cfg.layer_colors.get(layer_name, "#668")
            tx = p.x + (p.w + dx if dx < 0 else dx)
            ty = p.y + (p.h + dy if dy < 0 else dy)
            parts.append(
                f'<text x="{tx:.1f}" y="{ty:.1f}" text-anchor="{anchor}" fill="{color}" font-family="ui-monospace,Menlo,monospace" font-size="8">{_esc(other.label)}</text>'
            )
        if p.key.hold:
            parts.append(
                f'<text x="{p.x + p.w/2:.1f}" y="{p.y + p.h - 5:.1f}" text-anchor="middle" fill="#c44" font-family="ui-monospace,Menlo,monospace" font-size="8">{_esc(p.key.hold)}</text>'
            )
    parts.append("</svg>")
    return "\n".join(parts)


def _chords_layer(ir: KeymapIR, cfg: VizConfig, *, title: str | None) -> str:
    base = ir.layers[0]
    title_h = 36
    placed, w, h = place_keys(ir.layout, base.keys, title_h=title_h)
    by_i = {p.i: p for p in placed}
    fill, stroke, _leg = _key_fills(cfg)
    mute = "#f0ece6" if cfg.key_style == "keycap_tone" else "#333333"
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0f}" height="{h:.0f}" viewBox="0 0 {w:.0f} {h:.0f}">',
        f'<rect width="100%" height="100%" fill="{cfg.background}"/>',
        f'<text x="16" y="22" fill="{cfg.title_color}" font-family="ui-sans-serif,system-ui,sans-serif" font-size="14">{_esc(title or "Chords (separate layer)")}</text>',
    ]
    active = {i for ch in ir.chords for i in ch.indices}
    for p in placed:
        f = fill if p.i in active else mute
        op = "1" if p.i in active else "0.35"
        parts.append(
            f'<rect x="{p.x:.1f}" y="{p.y:.1f}" width="{p.w:.1f}" height="{p.h:.1f}" rx="6" fill="{f}" stroke="{stroke}" opacity="{op}"/>'
        )
    for ch in ir.chords:
        pts = [by_i[i] for i in ch.indices if i in by_i]
        if len(pts) < 2:
            continue
        centers = [(p.x + p.w / 2, p.y + p.h / 2) for p in pts]
        parts.append(
            f'<polyline points="{" ".join(f"{x:.1f},{y:.1f}" for x, y in centers)}" fill="none" stroke="#d9480f" stroke-width="2" stroke-dasharray="5 4"/>'
        )
        mx = sum(x for x, _ in centers) / len(centers)
        my = min(y for _, y in centers) - 14
        parts.append(f'<rect x="{mx-18:.1f}" y="{my-10:.1f}" width="36" height="14" rx="3" fill="#fff5f0" stroke="#d9480f"/>')
        parts.append(
            f'<text x="{mx:.1f}" y="{my:.1f}" text-anchor="middle" fill="#d9480f" font-size="9" font-family="ui-monospace,Menlo,monospace">{_esc(ch.label)}</text>'
        )
    parts.append("</svg>")
    return "\n".join(parts)
