{
  keymap
    | { layers : Array (Array Number) }
    | doc "The keymap.ncl file that gets rendered to JSON"
    = import "keymap.ncl",

  make_key
    | doc "Constructs a JSON-representable key value from the given value from keymap.ncl."
    = fun v =>
      if std.is_number v then
        { "simple" = v }
      else
        std.fail_with "unsupported value",

  keymap_json
    | doc "The keymap.json output value."
    =
      keymap.layers |> std.array.at 0 |> std.array.map make_key,
}
