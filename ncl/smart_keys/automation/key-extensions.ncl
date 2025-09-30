{
  key_extensions = {
    automation = {
      string_macro = fun s =>
        {
          automation_instructions =
            let K = import "keys.ncl" in
            s
            |> std.string.uppercase
            |> std.string.characters
            |> std.array.map (fun char =>
              let { key_code = kc } = K."%{char}" in
              let ko = { key_code = { Keyboard = kc } } in
              [
                { Press = ko, },
                { Release = ko, },
              ]
            )
            |> std.array.flatten,
        },
    },
  },
}
