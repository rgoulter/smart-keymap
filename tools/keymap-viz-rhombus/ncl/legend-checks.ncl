# LegendIR exporter checks (Nickel).
#
# Evaluated by run.sh test (nickel eval --field evaluated_checks).
# Covers the exporter path the Rhombus tests miss: named-layer holds
# resolve to Int washes (the legend/v0 wash-is-Int contract), including
# the ch32x fn bundle end to end.
let C = import "checks.ncl" in
let K = import "keys.ncl" in
let E = import "export-legend-lib.ncl" in
let fn_bundle = import "../layouts/ch32x_60_improved/export-legend.ncl" in
{
  checks = {
    check_number_hold_keeps_wash = {
      actual = E.maybe_wash_with {} (K.Tab & K.hold (K.layer_mod.hold 8)),
      expected = { wash = 8 },
    },
    check_named_hold_resolves = {
      actual = E.maybe_wash_with { fn = 1 } (K.layer_mod.hold "fn"),
      expected = { wash = 1 },
    },
    check_unknown_name_drops_wash = {
      actual = E.maybe_wash_with {} (K.layer_mod.hold "fn"),
      expected = {},
    },
    check_plain_key_has_no_wash = {
      actual = E.maybe_wash_with { fn = 1 } K.A,
      expected = {},
    },
    check_fn_bundle_washes_are_ints = {
      actual =
        fn_bundle.base.cells
        |> std.array.filter (fun c => std.record.has_field "wash" c)
        |> std.array.map (fun c => c.wash),
      expected = [1],
    },
  },

  evaluated_checks = C.evaluate_checks checks,
}
