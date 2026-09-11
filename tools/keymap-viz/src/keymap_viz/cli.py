from __future__ import annotations
"""Imperative shell: argparse + filesystem only."""
import argparse
from pathlib import Path
from .demos import from_48key_json, ch32x36_ish, ch32x36_full, ansi_60, default_viz
from .render import render_svg
from .ir import VizConfig


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(prog="keymap-viz")
    p.add_argument("demo", choices=["stacked", "dense", "chords", "ch32x36", "ch32x36full", "ansi60", "all"])
    p.add_argument("-o", "--outdir", type=Path, default=Path("."))
    p.add_argument("--dark", action="store_true", help="solid_dark key style")
    args = p.parse_args(argv)
    outdir: Path = args.outdir
    outdir.mkdir(parents=True, exist_ok=True)

    style = "solid_dark" if args.dark else "keycap_tone"
    bg = "#1a1a1a" if args.dark else "#f4f1ec"
    title_c = "#eee" if args.dark else "#222"
    ir_path = Path(__file__).resolve().parent / "fixtures" / "ir-48key-basic.json"

    def write(name: str, svg: str) -> None:
        path = outdir / name
        path.write_text(svg)
        print(path)

    demos = args.demo if args.demo != "all" else ["stacked", "dense", "chords", "ch32x36", "ch32x36full", "ansi60"]
    if isinstance(demos, str):
        demos = [demos]

    for d in demos:
        if d in ("stacked", "dense", "chords"):
            ir = from_48key_json(ir_path)
            cfg = default_viz(mode=d, key_style=style, background=bg, title_color=title_c)
            write(f"{d}.svg", render_svg(ir, cfg))
        elif d == "ch32x36":
            ir = ch32x36_ish()
            cfg = default_viz(mode="single", key_style=style, background=bg, title_color=title_c)
            write("layout-ch32x36-ish.svg", render_svg(ir, cfg, title="CH32X-36-ish · y_up flipped"))
        elif d == "ch32x36full":
            ir = ch32x36_full()
            cfg = default_viz(mode="single", key_style=style, background=bg, title_color=title_c)
            write("layout-ch32x36-full.svg", render_svg(ir, cfg, title="CH32X-36 full split"))
        elif d == "ansi60":
            ir = ansi_60()
            cfg = default_viz(mode="single", key_style=style, background=bg, title_color=title_c)
            write("layout-60ansi.svg", render_svg(ir, cfg, title="60% ANSI"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
