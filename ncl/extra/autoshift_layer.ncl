let K = import "keys.ncl" in
let keymap_ncl = (import "keymap-ncl-to-json.ncl").keymap_ncl in
let tap_hold_ncl = keymap_ncl.tap_hold in
{
  # Transform a single base key into autoshift, folding existing HRM hold into inner.
  #
  # - `null` stays null (transparent).
  # - Existing tap-hold (`{hold, ..tap}`) reuses its `hold` as inner hold.
  # - Plain keys become `k & hold (k & LeftShift)` (no HRM creation).
  # - Other composites (tap_dance, sticky, etc.) are left as-is — don't double-wrap.
  transform_key
    | doc m%"
        Turn a base key into autoshift, folding HRM if present.

        Inner profile must exist: `config.tap_hold.profiles.inner_hold = {timeout=null, interrupt_response="HoldOnKeyPress"}`.
        Outer uses the default profile (`timeout=200 Ignore` → `a`/`A`).
      "%
    = fun k =>
      k
      |> match {
        null => null,
        {hold = hrm_inner, ..tap_rest} =>
          let tap = tap_hold_ncl.strip_profile_meta tap_rest in
          tap & K.hold (tap & K.LeftShift & K.hold hrm_inner & K.tap_hold_profile "inner_hold"),
        _ => k & K.hold (k & K.LeftShift)
      },

  # Pipe-friendly autoshift for a single layer row.
  # `layer |> autoshift` — folds HRM where present, otherwise plain autoshift.
  autoshift
    | doc m%"
        Pipe-friendly: `keys |> autoshift` or `layer |> autoshift`.

        Folds any existing `hold` (`K.A & hold K.Ctrl` → `K.A & hold (K.A&LeftShift & hold K.Ctrl & inner)`).
        Plain `K.A` → `K.A & hold (K.A&LeftShift)`.
      "%
    = fun keys => keys |> std.array.map transform_key,

  checks | default = {},
  checks.autoshift_layer =
    let K = import "keys.ncl" in
    let inner = K.tap_hold_profile "inner_hold" in
    {
      check_transform_null_is_null = {
        actual = transform_key null,
        expected = null,
      },
      check_transform_plain_is_autoshift = {
        actual = transform_key K.A,
        expected = K.A & K.hold (K.A & K.LeftShift),
      },
      check_transform_existing_hrm_folds = {
        actual = transform_key (K.A & K.hold K.LeftAlt),
        expected = K.A & K.hold (K.A & K.LeftShift & K.hold K.LeftAlt & inner),
      },
      check_layer_pipe_is_autoshift = {
        actual = [K.A, K.B] |> autoshift,
        expected = [K.A & K.hold (K.A & K.LeftShift), K.B & K.hold (K.B & K.LeftShift)],
      },
      check_layer_existing_hrm_folds = {
        actual = [K.A & K.hold K.LeftCtrl, K.B] |> autoshift,
        expected = [
          K.A & K.hold (K.A & K.LeftShift & K.hold K.LeftCtrl & inner),
          K.B & K.hold (K.B & K.LeftShift),
        ],
      },
    },
}
