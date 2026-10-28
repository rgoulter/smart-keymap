# keymap.ncl

The Nickel code in `ncl/` has some functions
which help when writing `keymap.ncl` files.

## Chord indices (`chording.ncl`)

Mark chord members with any non-`_` character (typically `X`).
Indices are collected in scan order (left-to-right, top-to-bottom).

```nickel
let CH = import "chording.ncl" in
{
  indices = m%"
    _ _ X X _
  "% |> CH.indices,
}
```

## Sequence indices (`sequence.ncl`)

Mark sequence steps with `0 1 2 …`.
Returned indices are sorted by those markers, so the tap order
can differ from scan order.

```nickel
let Seq = import "sequence.ncl" in
{
  indices = m%"
    _ _ 1 _ _
    _ _ 0 _ _
  "% |> Seq.indices, # first the `0` key, then the `1` key
}
```
