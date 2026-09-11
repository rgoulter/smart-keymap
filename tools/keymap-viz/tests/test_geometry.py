from keymap_viz.ir import Key, Layout
from keymap_viz.geometry import place_keys


def test_y_up_flips_so_higher_source_y_is_higher_on_screen_top():
    # In SVG, smaller y is toward the top of the image.
    # With y_up=True, larger source y should get smaller SVG y (nearer top).
    keys = (
        Key(i=0, x=0, y=0, label="thumb"),   # bottom in KiCad
        Key(i=1, x=0, y=40, label="top"),    # top in KiCad
    )
    layout = Layout(kind="points", y_up=True, unit=19.05)
    placed, _, _ = place_keys(layout, keys, u=19.05, pad=0, title_h=0)
    by_i = {p.i: p for p in placed}
    assert by_i[1].y < by_i[0].y


def test_ortho_grid():
    keys = tuple(Key(i=r*2+c, r=r, c=c, label=f"{r},{c}") for r in range(2) for c in range(2))
    layout = Layout(kind="ortho", rows=2, cols=2)
    placed, w, h = place_keys(layout, keys, u=10, gap=0, pad=0)
    assert len(placed) == 4
    assert placed[0].x == 0 and placed[0].y == 0
    assert placed[3].x == 10 and placed[3].y == 10
