let keymap | { layers : Array (Array Number) } = import "keymap.ncl" in

let make_key = fun v =>
  if std.is_number v then
    { "simple" = v }
  else
    std.fail_with "unsupported value"
in

keymap.layers |> std.array.at 0 |> std.array.map make_key
